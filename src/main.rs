use std::fmt;

enum Expression {
    Number(i32),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Add(left, right) => write!(f, "({} + {})", left, right),
            Expression::Subtract(left, right) => write!(f, "({} - {})", left, right),
            Expression::Multiply(left, right) => write!(f, "({} * {})", left, right),
            Expression::Divide(left, right) => write!(f, "({} / {})", left, right),
        }
    }
}

fn evaluate(expr: &Expression) -> f32 {
    match expr {
        Expression::Number(n) => *n as f32,
        Expression::Add(left, right) => evaluate(left) + evaluate(right),
        Expression::Subtract(left, right) => evaluate(left) - evaluate(right),
        Expression::Multiply(left, right) => evaluate(left) * evaluate(right),
        Expression::Divide(left, right) => evaluate(left) / evaluate(right),
    }
}

fn main() {
    let expression = Expression::Add(
        Box::new(Expression::Multiply(
            Box::new(Expression::Number(4)),
            Box::new(Expression::Number(5)),
        )),
        Box::new(Expression::Subtract(
            Box::new(Expression::Number(10)),
            Box::new(Expression::Number(2)),
        )),
    );

    println!("Expression: {}", expression);
    println!("Result: {}", evaluate(&expression));

    let expression2 = Expression::Divide(
        Box::new(Expression::Number(20)),
        Box::new(Expression::Add(
            Box::new(Expression::Number(4)),
            Box::new(Expression::Number(1)),
        )),
    );

    println!("Expression: {}", expression2);
    println!("Result: {}", evaluate(&expression2));

    let expression3 = Expression::Subtract(
        Box::new(Expression::Add(
            Box::new(Expression::Number(6)),
            Box::new(Expression::Number(4)),
        )),
        Box::new(Expression::Multiply(
            Box::new(Expression::Number(2)),
            Box::new(Expression::Number(5)),
        )),
    );

    println!("Expression: {}", expression3);
    println!("Result: {}", evaluate(&expression3));
}