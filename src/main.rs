use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    println!("********************************************");
    println!("********************************************");
    println!("Welcome by Generate a nth Fibonacci number");
    println!("Enter a integer for n: ");
    let mut n = String::new();
    stdin().read_line(&mut n)?;
    let n = n.trim().parse()?;

    let number = fib(n);

    println!("The Fibonacci({}) => {}", n, number,);
    println!("********************************************");
    println!("********************************************");

    Ok(())
}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
