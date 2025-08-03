use std::io::{self, Write};

#[derive(Debug)]
enum Operation {
    Push(f64),
    Add,
    Sub,
    Mul,
    Div,
    Pop,
    Dup,
    Swap,
    Print,
    Clear,
}

struct StackMachine {
    stack: Vec<f64>,
}

impl StackMachine {
    fn new() -> Self {
        StackMachine { stack: Vec::new() }
    }

    fn execute(&mut self, op: Operation) -> Result<(), String> {
        match op {
            Operation::Push(value) => {
                self.stack.push(value);
                Ok(())
            }
            Operation::Add => self.binary_op(|a, b| a + b),
            Operation::Sub => self.binary_op(|a, b| a - b),
            Operation::Mul => self.binary_op(|a, b| a * b),
            Operation::Div => {
                if self.stack.len() < 2 {
                    return Err("Not enough values on stack for division".to_string());
                }
                let b = self.stack.pop().unwrap();
                if b == 0.0 {
                    self.stack.push(b);
                    return Err("Division by zero".to_string());
                }
                let a = self.stack.pop().unwrap();
                self.stack.push(a / b);
                Ok(())
            }
            Operation::Pop => {
                if self.stack.is_empty() {
                    Err("Stack is empty".to_string())
                } else {
                    self.stack.pop();
                    Ok(())
                }
            }
            Operation::Dup => {
                if let Some(&top) = self.stack.last() {
                    self.stack.push(top);
                    Ok(())
                } else {
                    Err("Stack is empty".to_string())
                }
            }
            Operation::Swap => {
                if self.stack.len() < 2 {
                    Err("Need at least 2 values on stack to swap".to_string())
                } else {
                    let len = self.stack.len();
                    self.stack.swap(len - 1, len - 2);
                    Ok(())
                }
            }
            Operation::Print => {
                if let Some(&top) = self.stack.last() {
                    println!("Top: {}", top);
                } else {
                    println!("Stack is empty");
                }
                Ok(())
            }
            Operation::Clear => {
                self.stack.clear();
                Ok(())
            }
        }
    }

    fn binary_op<F>(&mut self, f: F) -> Result<(), String>
    where
        F: Fn(f64, f64) -> f64,
    {
        if self.stack.len() < 2 {
            return Err("Not enough values on stack for operation".to_string());
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(f(a, b));
        Ok(())
    }

    fn print_stack(&self) {
        if self.stack.is_empty() {
            println!("Stack: []");
        } else {
            print!("Stack: [");
            for (i, val) in self.stack.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", val);
            }
            println!("]");
        }
    }
}

fn parse_command(input: &str) -> Result<Operation, String> {
    let trimmed = input.trim();
    
    if let Ok(num) = trimmed.parse::<f64>() {
        return Ok(Operation::Push(num));
    }

    match trimmed.to_lowercase().as_str() {
        "+" | "add" => Ok(Operation::Add),
        "-" | "sub" => Ok(Operation::Sub),
        "*" | "mul" => Ok(Operation::Mul),
        "/" | "div" => Ok(Operation::Div),
        "pop" => Ok(Operation::Pop),
        "dup" => Ok(Operation::Dup),
        "swap" => Ok(Operation::Swap),
        "print" | "." => Ok(Operation::Print),
        "clear" | "cls" => Ok(Operation::Clear),
        _ => Err(format!("Unknown command: {}", trimmed)),
    }
}

fn print_help() {
    println!("\nStack Machine Calculator");
    println!("========================");
    println!("Commands:");
    println!("  <number>      - Push a number onto the stack");
    println!("  + or add      - Pop two values, push their sum");
    println!("  - or sub      - Pop two values, push difference (second - first)");
    println!("  * or mul      - Pop two values, push their product");
    println!("  / or div      - Pop two values, push quotient (second / first)");
    println!("  pop           - Remove the top value from the stack");
    println!("  dup           - Duplicate the top value on the stack");
    println!("  swap          - Swap the top two values on the stack");
    println!("  print or .    - Print the top value of the stack");
    println!("  clear or cls  - Clear the stack");
    println!("  stack         - Show the entire stack");
    println!("  help          - Show this help message");
    println!("  quit or exit  - Exit the calculator");
    println!();
}

fn main() {
    let mut machine = StackMachine::new();
    
    print_help();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                
                if trimmed.is_empty() {
                    continue;
                }
                
                match trimmed.to_lowercase().as_str() {
                    "quit" | "exit" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" => {
                        print_help();
                        continue;
                    }
                    "stack" => {
                        machine.print_stack();
                        continue;
                    }
                    _ => {}
                }
                
                match parse_command(trimmed) {
                    Ok(op) => {
                        if let Err(e) = machine.execute(op) {
                            println!("Error: {}", e);
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                break;
            }
        }
    }
}