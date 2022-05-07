fn main() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    print!("Test passed!");
   
}

//f=(1.8*c)+32
fn celsius_to_fahrenheit(cel: f64) -> f64 {
    let fehrenheit = (1.8 * cel) + 32.0;
    return fehrenheit;
}
