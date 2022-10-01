use fib::fibonacci;

fn main() {
    //println!("{}", fibonacci(4));

    for i in 0..11 {
        println!("the {i} number is {}.", fibonacci(i));   
    }
}
