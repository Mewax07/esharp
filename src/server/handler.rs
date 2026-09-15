use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};

use crate::math::{
    Evaluator, Renderer,
    ast::{Calculation, Expr},
    lexer::Lexer,
    parser::Parser,
};

pub fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024 * 4];
    let bytes_read = stream.read(&mut buffer).unwrap_or(0);
    if bytes_read == 0 {
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let path = if let Some(first_line) = request.lines().next() {
        first_line.split_whitespace().nth(1).unwrap_or("/")
    } else {
        "/"
    };

    let (path, query_params) = if let Some(query_start) = path.find('?') {
        let (base_path, query) = path.split_at(query_start);
        let params = parse_query_params(query);
        (base_path, params)
    } else {
        (path, HashMap::new())
    };

    let body = if request.contains("\r\n\r\n") {
        let body_start = request.find("\r\n\r\n").unwrap() + 4;
        &request[body_start..]
    } else {
        ""
    };

    let response = match (path, query_params, body) {
        ("/calculate", _, body) if !body.is_empty() => handle_calculate(body),
        ("/api/calculate", _, body) if !body.is_empty() => handle_api_calculate(body),
        _ => handle_404(),
    };

    stream.write_all(&response).unwrap();
    stream.flush().unwrap();
}

fn parse_query_params(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            params.insert(key.to_string(), value.to_string());
        }
    }

    params
}

fn handle_calculate(body: &str) -> Vec<u8> {
    let expression = if let Some(expr_start) = body.find("expression=") {
        let expr = &body[expr_start + 10..];
        let expr = expr.split('&').next().unwrap_or("");
        let expr = expr.replace("+", " ");
        url_decode(&expr)
    } else {
        body.to_string()
    };

    let mut calculation = Calculation::new(&expression);

    let mut lexer = Lexer::new(&expression);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    if let Some(ast) = parser.parse() {
        calculation = calculation.with_ast(ast);

        let evaluator = Evaluator::new();
        let mut calc_with_steps = calculation.clone();
        if let Err(e) = evaluator.eval_calculation(&mut calc_with_steps) {
            return format!(
                "<!DOCTYPE html>\n<html>\n<body>\n\t<h1>Erreur</h1>\n\t<p>{}</p>\n\t<p>Expression: {}</p>\n</body>\n</html>",
                e, expression
            ).into_bytes();
        }

        let renderer = Renderer::new();
        let html = renderer.render_full_page(&calc_with_steps);
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/html; charset=utf-8\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {}",
            html.len(),
            html
        );

        return response.into_bytes();
    }

    format!(
        "<!DOCTYPE html>\n<html>\n<body>\n\t<h1>Erreur de parsing</h1>\n\t<p>Impossible de parser l'expression: {}</p>\n</body>\n</html>",
        expression
    ).into_bytes()
}

fn handle_api_calculate(body: &str) -> Vec<u8> {
    let expression = if let Some(expr_start) = body.find("expression=") {
        let expr = &body[expr_start + 10..];
        let expr = expr.split('&').next().unwrap_or("");
        url_decode(expr)
    } else {
        body.to_string()
    };

    let mut calculation = Calculation::new(&expression);

    let mut lexer = Lexer::new(&expression);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    if let Some(ast) = parser.parse() {
        calculation = calculation.with_ast(ast);

        let evaluator = Evaluator::new();
        if let Ok(_) = evaluator.eval_calculation(&mut calculation) {
            let json = format!(
                "{{\"expression\": \"{}\", \"result\": \"{}\", \"ast\": \"{}\"}}",
                calculation.expression,
                calculation.result.unwrap_or(0.0),
                format_ast_for_json(&calculation.ast.unwrap()),
            );

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                json.len(),
                json
            );

            return response.into_bytes();
        }
    }

    let error_json = format!(
        "{{\"error\": \"Failed to parse or evaluate expression: {}\"}}",
        expression
    );

    let response = format!(
        "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        error_json.len(),
        error_json
    );

    response.into_bytes()
}

fn handle_404() -> Vec<u8> {
    let html = "<!DOCTYPE html>\n<html>\n<body>\n\t<h1>404 Not Found</h1>\n\t<p>La page demandée n'existe pas.</p>\n</body>\n</html>";

    format!(
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    )
    .into_bytes()
}

fn url_decode(s: &str) -> String {
    s.replace("%2B", "+")
        .replace("%20", " ")
        .replace("%2F", "/")
        .replace("%2A", "*")
        .replace("%5E", "^")
        .replace("%28", "(")
        .replace("%29", ")")
        .replace("%3D", "=")
}

fn format_ast_for_json(expr: &Expr) -> String {
    match expr {
        Expr::Num(n) => format!("Num({})", n),
        Expr::Var(name) => format!("Var({})", name),
        Expr::Add(left, right) => format!(
            "Add({}, {})",
            format_ast_for_json(left),
            format_ast_for_json(right)
        ),
        Expr::Sub(left, right) => format!(
            "Sub({}, {})",
            format_ast_for_json(left),
            format_ast_for_json(right)
        ),
        Expr::Mul(left, right) => format!(
            "Mul({}, {})",
            format_ast_for_json(left),
            format_ast_for_json(right)
        ),
        Expr::Div(left, right) => format!(
            "Div({}, {})",
            format_ast_for_json(left),
            format_ast_for_json(right)
        ),
        Expr::Pow(left, right) => format!(
            "Pow({}, {})",
            format_ast_for_json(left),
            format_ast_for_json(right)
        ),
        Expr::Neg(expr) => format!("Neg({})", format_ast_for_json(expr)),
        Expr::Call { name, arguments } => {
            let args: Vec<String> = arguments.iter().map(format_ast_for_json).collect();
            format!("Call({}, [{}])", name, args.join(", "))
        }
    }
}
