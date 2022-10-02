fn main() {


    let mut s = String::new();
    s.push('I');
    s.push(' ');
    s.push_str("am");
    println!{"{}",s};

    let data = "initial contents";
    let b = data.to_string();
    println!("{}", b);

    let c = "initial contents".to_string();
    println!("{}", c);

    for i in s.bytes(){
        println!("{}", i);
    }

    for i in s.chars(){
        println!("{}",i);
    }  
}