use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let result = divide(1, 0)?;
    println!("Result: {}", result);
    Ok(())
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(format!("Division by zero is not allowed in: {} / {}", a, b));
    }
    Ok(a / b)
}
