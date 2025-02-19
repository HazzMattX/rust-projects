pub enum Operator {
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
    pub fn calculate(op: Operator, num1: f64, num2: Option<f64>) -> f64 {
            use Operator::*;
            // The actual math
            match op {
                Add => num1 + num2.expect("Second number required"),
                Subtract => num1 - num2.expect("Second number required"),
                Multiply => num1 * num2.expect("Second number required"),
                Divide => num1 / num2.expect("Second number required"),
                Exponent => num1.powf(num2.expect("Second number required")),
                GreaterThan => {
                    if num1 > num2.expect("Second number required") { 1.0 } else { 0.0 }
                },
                GreaterThanOrEqualTo => {
                    if num1 >= num2.expect("Second number required") { 1.0 } else { 0.0 }
                },
                LessThan => {
                    if num1 < num2.expect("Second number required") { 1.0 } else { 0.0 }
                },
                LessThanOrEqualTo => {
                    if num1 <= num2.expect("Second number required") { 1.0 } else { 0.0 }
                },
                Factorial => (1..=num1 as u64).product::<u64>() as f64,
            }
        }
}
