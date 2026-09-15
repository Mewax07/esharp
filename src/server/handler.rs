use std::collections::HashMap;

use crate::{
    html,
    math::{
        Evaluator, Renderer,
        ast::{Calculation, Expr},
        lexer::Lexer,
        parser::Parser,
    },
    server::{Document, Method, Request, Response},
};

pub fn handle_request(request: Request) -> Response {
    match (&request.method, request.path.as_str()) {
        (Method::Get, "/") => handle_index(),
        (Method::Post, "/calculate") => handle_calculate(&request),
        (Method::Post, "/api/calculate") => handle_api_calculate(&request),
        (_, "/") => Response::method_not_allowed().allow("GET"),
        _ => Response::not_found(),
    }
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

fn handle_index() -> Response {
    let page = Document::new()
        .lang("fr")
        .title("ESharp")
        .body(
            html!("main")
                .class("container")
                .append(html!("h1").text_content("ESharp"))
                .append(html!("p").text_content("Un moteur mathématique écrit en Rust."))
                .append(
                    html!("form")
                        .action("/calculate")
                        .method("POST")
                        .append(
                            html!("input")
                                .r#type("text")
                                .name("expression")
                                .placeholder("2 + 5")
                                .self_closing(),
                        )
                        .append(html!("button").r#type("submit").text_content("Calculer")),
                ),
        )
        .render();

    Response::html(page)
}

fn handle_calculate(request: &Request) -> Response {
    let expression = extract_expression(request);

    let mut lexer = Lexer::new(&expression);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let Some(ast) = parser.parse() else {
        return Response::bad_request(format!("Impossible de parser : {}", expression));
    };

    let mut calculation = Calculation::new(&expression).with_ast(ast);

    let evaluator = Evaluator::new();

    if let Err(error) = evaluator.eval_calculation(&mut calculation) {
        return Response::bad_request(error.to_string());
    }

    let renderer = Renderer::new();

    Response::html(renderer.render_full_page(&calculation))
}

fn handle_api_calculate(request: &Request) -> Response {
    let expression = extract_expression(request);

    let mut lexer = Lexer::new(&expression);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let Some(ast) = parser.parse() else {
        return Response::json("{\"error\":\"Failed to parse expression\"}")
            .status(crate::server::StatusCode::BadRequest);
    };

    let mut calculation = Calculation::new(&expression).with_ast(ast);

    let evaluator = Evaluator::new();

    if let Err(error) = evaluator.eval_calculation(&mut calculation) {
        return Response::json(format!("{{\"error\":\"{}\"}}", error))
            .status(crate::server::StatusCode::BadRequest);
    }

    let ast = calculation.ast.as_ref().unwrap();

    let json = format!(
        "{{\
        \"expression\":\"{}\",\
        \"result\":{},\
        \"ast\":\"{}\"\
        }}",
        calculation.expression,
        calculation.result.unwrap_or(0.0),
        format_ast_for_json(ast),
    );

    Response::json(json)
}

fn extract_expression(request: &Request) -> String {
    if let Some(expression) = request.query.get("expression") {
        return expression.clone();
    }

    if let Some(expression) = request.body.strip_prefix("expression=") {
        return expression.split('&').next().unwrap_or("").to_string();
    }

    request.body.clone()
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
