fn main() {
    let num1 = 10;
    let num2 = 20;
    
    let sum = add_numbers(num1, num2);
    
    println!("The sum of {} and {} is: {}", num1, num2, sum);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
