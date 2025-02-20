mod equations;
use equations::Operator;
fn get_input(prompt: &str) -> String {
    use std::io::Write;
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn main() {
    // First number input
    let num1 = get_input("Enter number: ").trim().parse().unwrap();
    // Operator selection
    let operator = get_input("Choose operator: +, -, *, /, ^, >, >=, <, <=, !: ");
    use equations::Operator::*;
    let operator = match operator.trim() {
        "+" => Add,
        "-" => Subtract,
        "*" => Multiply,
        "/" => Divide,
        "^" => Exponent,
        ">" => GreaterThan,
        ">=" => GreaterThanOrEqualTo,
        "<" => LessThan,
        "<=" => LessThanOrEqualTo,
        "!" => Factorial,
        _ => {
            println!("Invalid operator");
            return;
        }
    };
    let num2 = match operator {
        Factorial => None,
        _ => {
            let num2 = get_input("Enter second number: ").trim().parse().unwrap();
            Some(num2)
        },
    };
    let result: f64 = Operator::calculate(operator, num1, num2);
    println!("{result}");
}
