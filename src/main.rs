use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
expression = _{ or_expression }
or_expression = { and_expression ~ ( "OR" ~ and_expression )* }
and_expression = { not_expression ~ ( "AND" ~ not_expression )* }
not_expression = _{ "NOT"? ~ atom }
atom = _{ "TRUE" | "FALSE" | "(" ~ expression ~ ")" }

WHITESPACE = _{ " " }
"#]
pub struct BooleanLogicParser;

fn eval(expression: pest::iterators::Pair<Rule>, context: &mut std::collections::HashMap<String, bool>) -> bool {
    match expression.as_rule() {
        Rule::or_expression => {
            let mut inner = expression.into_inner();
            let first = eval(inner.next().unwrap(), context);
            inner.fold(first, |acc, pair| acc || eval(pair, context))
        }
        Rule::and_expression => {
            let mut inner = expression.into_inner();
            let first = eval(inner.next().unwrap(), context);
            inner.fold(first, |acc, pair| acc && eval(pair, context))
        }
        Rule::not_expression => {
            let mut inner = expression.into_inner();
            let pair = inner.next().unwrap();
            let is_not = pair.as_str() == "NOT";
            let value = if is_not {
                eval(inner.next().unwrap(), context)
            } else {
                eval(pair, context)
            };
            if is_not { !value } else { value }
        }
        Rule::atom => match expression.as_str() {
            "TRUE" => true,
            "FALSE" => false,
            _ => eval(expression.into_inner().next().unwrap(), context),
        },
        _ => unreachable!(),
    }
}

fn eval_boolean_logic(input: &str) -> bool {
    let expression = BooleanLogicParser::parse(Rule::expression, &input).unwrap_or_else(|e| panic!("{}", e));
    let mut context = std::collections::HashMap::new();
    eval(expression.into_iter().next().unwrap(), &mut context)
}

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "quit" {
            break;
        } else {
            println!("{}", eval_boolean_logic(input));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert_eq!(eval_boolean_logic("TRUE"), true);
    }

    #[test]
    fn test_false() {
        assert_eq!(eval_boolean_logic("FALSE"), false);
    }

    #[test]
    fn test_and_true_false() {
        assert_eq!(eval_boolean_logic("AND TRUE FALSE"), false);
    }

    #[test]
    fn test_or_true_false() {
        assert_eq!(eval_boolean_logic("OR TRUE FALSE"), true);
    }

    #[test]
    fn test_not_true() {
        assert_eq!(eval_boolean_logic("NOT TRUE"), false);
    }

    #[test]
    fn test_complex_expression() {
        assert_eq!(eval_boolean_logic("AND (OR TRUE FALSE) (NOT FALSE)"), true);
    }
}