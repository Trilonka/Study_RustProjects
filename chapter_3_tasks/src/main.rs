fn main() {
    println!("Far to cel: {}", far_to_cel(-54.5));
    let n = 13;
    println!("Fibonacci {}: {}", n, fibonacci(n));
}

fn far_to_cel(f: f32) -> f32 {
    (f-32.0)*(5.0/9.0)
}

fn fibonacci(mut n: u32) -> u32 {
    if n < 3 {
        return 1;
    }
    if n == 3 {
        return 2;
    }
    let mut cor = (1, 1, 2);
    n -= 3;
    while n > 0 {
        cor.2 = cor.2 + cor.1;
        cor.1 = cor.1 + cor.0;
        cor.0 = cor.1 - cor.0;
        n -= 1;
    }
    return cor.2;
}

// Печатать текст не вижу смысла, понятно что нужно через циклы учесть повторы