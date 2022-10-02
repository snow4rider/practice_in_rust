fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(7);
    v.push(6);

    for i in &v {
        println!("{}",i);
    }

    //let mut b = vec![100,32,57]

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    //println!("Hello, world!");
}
