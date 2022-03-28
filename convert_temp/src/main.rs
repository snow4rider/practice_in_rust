fn main() {
    let temp = 50.00;
    println!("The temperature is {} degrees fahrenheit.", temp);

    let new_temp = convert_temp(temp);
    println!("The temperature is {} in celsius.", new_temp);
}

fn convert_temp(fahrenheit: f64) -> f64 {
    // cel = (fahrenheit - 32) * 5/9
    let cel = (fahrenheit - 32.0) * 5.0/9.0;
    return cel
}