pub fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fibonacci(n - 2) + fibonacci(n - 1),
    }
}

