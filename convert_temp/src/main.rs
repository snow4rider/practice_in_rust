fn main() {
    let temp = 50.00;
    println!("The tempurature is {} degrees fahrenhiet.", temp);

    let new_temp = convert_temp(temp);
    println!("The tempurature is {} in celcius.", new_temp);
}

fn convert_temp(fahren: f64) -> f64 {
    // cel = (fahren - 32) * 5/9
    let cel = (fahren - 32.0) * 5.0/9.0;
    return cel
}