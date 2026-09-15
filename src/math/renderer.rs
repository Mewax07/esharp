use crate::math::{Calculation, Expr, Function};

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render_expr(&self, expr: &Expr) -> String {
        self.render_expr_internal(expr)
    }

    fn render_expr_internal(&self, expr: &Expr) -> String {
        match expr {
            Expr::Num(n) => {
                if n.fract() == 0.0 {
                    format!("<span class=\"math-number\">{:.0}</span>", n)
                } else {
                    format!("<span class=\"math-number\">{}</span>", n)
                }
            }
            Expr::Var(name) => format!("<span class=\"math-var\">{}</span>", name),
            Expr::Add(left, right) => {
                format!(
                    "<span class=\"math-expr\">{} + {}</span>",
                    self.render_expr_internal(left),
                    self.render_expr_internal(right)
                )
            }
            Expr::Sub(left, right) => {
                format!(
                    "<span class=\"math-expr\">{} - {}</span>",
                    self.render_expr_internal(left),
                    self.render_expr_internal(right)
                )
            }
            Expr::Mul(left, right) => {
                format!(
                    "<span class=\"math-expr\">{} × {}</span>",
                    self.render_expr_internal(left),
                    self.render_expr_internal(right)
                )
            }
            Expr::Div(left, right) => {
                format!(
                    "<span class=\"math-fraction\">\n\t\t\t\t\t<span class=\"math-numerator\">{}</span>\n\t\t\t\t\t<span class=\"math-denominator\">{}</span>\n\t\t\t\t</span>",
                    self.render_expr_internal(left),
                    self.render_expr_internal(right)
                )
            }
            Expr::Pow(left, right) => {
                format!(
                    "<span class=\"math-power\">\n<span class=\"math-base\">{}</span>\n<span class=\"math-exponent\">^{}</span>\n</span>",
                    self.render_expr_internal(left),
                    self.render_expr_internal(right)
                )
            }
            Expr::Neg(expr) => {
                format!(
                    "<span class=\"math-negate\">-({})</span>",
                    self.render_expr_internal(expr)
                )
            }
            Expr::Call { name, arguments } => {
                let args: Vec<String> = arguments
                    .iter()
                    .map(|arg| self.render_expr_internal(arg))
                    .collect();
                format!(
                    "<span class=\"math-function\">{}({})</span>",
                    name,
                    args.join(", ")
                )
            }
        }
    }

    pub fn render_calculation(&self, calc: &Calculation) -> String {
        let mut html = String::new();

        html.push_str(&format!(
            "<div class=\"calculation\">\n\t<div class=\"calc-title\">Expression: {}</div>",
            calc.expression
        ));

        if let Some(ref ast) = calc.ast {
            html.push_str(&format!(
                "<div class=\"calc-ast\">AST: {}</div>\n",
                self.format_ast(ast)
            ));
        }

        if let Some(result) = calc.result {
            html.push_str(&format!(
                "<div class=\"calc-result\">Result: <span class=\"math-number\">{}</span></div>\n",
                result
            ));
        }

        if !calc.steps.is_empty() {
            html.push_str("<div class=\"calc-steps\">\n\t<h4>Steps:</h4>\n\t<ol>\n");

            for step in &calc.steps {
                html.push_str(&format!(
                    "\t\t<li><span class=\"step-desc\">{}:</span> {} = {}</li>\n",
                    step.description,
                    step.expression,
                    step.value.map_or("N/A".to_string(), |v| format!(
                        "<span class=\"math-number\">{}</span>",
                        v
                    ))
                ));
            }

            html.push_str("\t</ol>\n</div>\n");
        }

        html.push_str("</div>\n");
        html
    }

    pub fn format_ast(&self, expr: &Expr) -> String {
        self.format_ast_internal(expr, 0)
    }

    fn format_ast_internal(&self, expr: &Expr, indent: usize) -> String {
        let indent_str = "  ".repeat(indent);

        match expr {
            Expr::Num(n) => format!("{}{}", indent_str, n),
            Expr::Var(name) => format!("{}{}", indent_str, name),
            Expr::Add(left, right) => {
                format!(
                    "{}+ Add\n{}\n{}",
                    indent_str,
                    self.format_ast_internal(left, indent + 1),
                    self.format_ast_internal(right, indent + 1)
                )
            }
            Expr::Sub(left, right) => {
                format!(
                    "{}- Sub\n{}\n{}",
                    indent_str,
                    self.format_ast_internal(left, indent + 1),
                    self.format_ast_internal(right, indent + 1)
                )
            }
            Expr::Mul(left, right) => {
                format!(
                    "{}× Mul\n{}\n{}",
                    indent_str,
                    self.format_ast_internal(left, indent + 1),
                    self.format_ast_internal(right, indent + 1)
                )
            }
            Expr::Div(left, right) => {
                format!(
                    "{}÷ Div\n{}\n{}",
                    indent_str,
                    self.format_ast_internal(left, indent + 1),
                    self.format_ast_internal(right, indent + 1)
                )
            }
            Expr::Pow(left, right) => {
                format!(
                    "{}^ Pow\n{}\n{}",
                    indent_str,
                    self.format_ast_internal(left, indent + 1),
                    self.format_ast_internal(right, indent + 1)
                )
            }
            Expr::Neg(expr) => {
                format!(
                    "{}- Neg\n{}",
                    indent_str,
                    self.format_ast_internal(expr, indent + 1)
                )
            }
            Expr::Call { name, arguments } => {
                let args: Vec<String> = arguments
                    .iter()
                    .map(|arg| self.format_ast_internal(arg, indent + 1))
                    .collect();
                format!(
                    "{}{} Call ({} args)\n{}",
                    indent_str,
                    name,
                    arguments.len(),
                    args.join("\n")
                )
            }
        }
    }

    pub fn render_function(&self, func: &Function) -> String {
        format!(
            "<div class=\"function\">\n\t<div class=\"func-name\">Function: {}</div>\n\t<div class=\"func-params\">Params: {}</div>\n\t<div class=\"func-body\">Body: {}</div>\n</div>",
            func.name,
            func.params.join(", "),
            self.render_expr(&func.body)
        )
    }

    pub fn render_full_page(&self, calc: &Calculation) -> String {
        format!(
            "<!DOCTYPE html>\n<html>\n<head>\n\t<meta charset=\"utf-8\">\n\t<title>Math Calculation</title>\n\t<style>\n\t\t{}\n\t</style>\n</head>\n<body>\n\t<div class=\"container\">\n\t\t{}\n\t</div>\n</body>\n</html>",
            self.get_styles(),
            self.render_calculation(calc)
        )
    }

    fn get_styles(&self) -> String {
        r#"
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            padding: 20px;
            background-color: #f5f5f5;
            margin: 0;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        .calculation {
            margin-bottom: 30px;
            padding: 15px;
            border: 1px solid #ddd;
            border-radius: 5px;
        }
        .calc-title {
            font-size: 1.2em;
            font-weight: bold;
            color: #333;
            margin-bottom: 10px;
        }
        .calc-ast {
            font-family: 'Courier New', monospace;
            background: #f9f9f9;
            padding: 10px;
            border-radius: 3px;
            margin: 10px 0;
            white-space: pre;
        }
        .calc-result {
            font-size: 1.1em;
            color: #2c3e50;
            margin: 10px 0;
            padding: 10px;
            background: #e8f4f8;
            border-radius: 3px;
        }
        .calc-steps {
            margin-top: 15px;
        }
        .calc-steps h4 {
            color: #3498db;
            margin-bottom: 10px;
        }
        .calc-steps ol {
            padding-left: 20px;
        }
        .calc-steps li {
            margin: 5px 0;
            padding: 8px;
            background: #f0f0f0;
            border-radius: 3px;
        }
        .step-desc {
            font-weight: bold;
            color: #2980b9;
        }
        .math-number {
            color: #e74c3c;
            font-weight: bold;
        }
        .math-var {
            color: #27ae60;
            font-style: italic;
        }
        .math-expr {
            color: #7f8c8d;
        }
        .math-fraction {
            display: inline-block;
            vertical-align: middle;
            position: relative;
        }
        .math-numerator {
            display: block;
            text-align: center;
            border-bottom: 1px solid #ccc;
            padding: 2px 0;
        }
        .math-denominator {
            display: block;
            text-align: center;
            padding: 2px 0;
        }
        .math-power {
            display: inline-block;
        }
        .math-base {
            display: inline-block;
        }
        .math-exponent {
            display: inline-block;
            font-size: 0.8em;
            vertical-align: super;
        }
        "#
        .to_string()
    }
}
