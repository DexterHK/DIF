// Function to multiply two numbers
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    let num1 = 5;
    let num2 = 10;

    // Call the multiply function and store the result
    let result = multiply(num1, num2);

    // Print the result
    println!("Result of {} * {} = {}", num1, num2, result);
}
