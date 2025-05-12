use std::io::Write;

pub trait Computer {
    fn compute(&self, expr: &str) -> i32;
}

pub struct CommandLineComputer;

impl Computer for CommandLineComputer {
    fn compute(&self, expr: &str) -> i32 {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut op: Option<char> = None;
        for c in expr.trim().chars() {
            if c.is_digit(10) {
                if op.is_none() {
                    num1.push(c);
                } else {
                    num2.push(c);
                }
                continue;
            }
            match c {
                '+' | '-' | '*' | '/' if op.is_none() => op = Some(c),
                _ if c.is_whitespace() => continue,
                _ => panic!("Invalid character: {}", c),
            }
        }
        if num1.is_empty() || num2.is_empty() || op.is_none() {
            panic!("Invalid expression: {}", expr);
        }
        let (num1, num2) = (num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap());
        let op = op.unwrap();
        match op {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => unreachable!(),
        }
    }
}

pub struct UserType<T> {
    computer: T,
    expr: String,
}

impl<T: Computer> UserType<T> {
    pub fn new(computer: T) -> Self {
        Self {
            computer,
            expr: String::new(),
        }
    }

    pub fn type_expr(&mut self) {
        self.expr.clear();
        print!("Please type an expression: ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin()
            .read_line(&mut self.expr)
            .expect("Failed to read line");
    }

    pub fn compute(&self) -> i32 {
        self.computer.compute(&self.expr)
    }
} 