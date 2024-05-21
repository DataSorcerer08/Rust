fn main() {
    let float_num: f64 = 42.67;

    let int_num: i32 = float_num as i32;
    println!("The float number {} casted to integer is {}", float_num, int_num);

    let new_float_num: f64 = int_num as f64;
    println!("The integer {} casted back to float is {}", int_num, new_float_num);
}
