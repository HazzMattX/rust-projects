enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    Factorial,
}
impl Operator {
    fn calculate(op: Operator, num1: f64, num2: f64) -> f64 {
        use Operator::*;
        // The actual math
        match op {
            Add => num1 + num2,
            Subtract => num1 - num2,
            Multiply => num1 * num2,
            Divide => num1 / num2,
            Exponent => num1.powf(num2),
            GreaterThan => {
                if num1 > num2 { 1.0 } else { 0.0 }
            },
            GreaterThanOrEqualTo => {
                if num1 >= num2 { 1.0 } else { 0.0 }
            },
            LessThan => {
                if num1 < num2 { 1.0 } else { 0.0 }
            },
            LessThanOrEqualTo => {
                if num1 <= num2 { 1.0 } else { 0.0 }
            },
            Factorial => {
                let mut fact = 1.0;
                for i in 1..=(num1 as u64) {
                    fact *= i as f64;
                }
                fact
            }
        }
    }
}
fn main() {
    // First number input
    println!("First number");
    let mut num1 = String::new();
    std::io::stdin().read_line(&mut num1).expect("Failed");
    let num1: f64 = num1.trim().parse().expect("Needs to be a number");
    // Operator selection
    println!("Choose operator: +, -, *, /, ^, >, >=, <, <=, !");
    let mut operator = String::new();
    std::io::stdin().read_line(&mut operator).expect("Failed");
    // Second number input
    println!("Second number");
    let mut num2 = String::new();
    std::io::stdin().read_line(&mut num2).expect("Failed");
    let num2: f64 = num2.trim().parse().expect("Needs to be a number");
    // Which equation to use: Line 14
    use Operator::*;
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
    let result: f64 = Operator::calculate(operator, num1, num2);
    println!("{result}");
}