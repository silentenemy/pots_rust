fn fib(n: u32) -> u32 {
    if n <= 2 {
        // первый шаг.
        return 1;
    } else {
        // рекурсия.
        return fib(n-1)+fib(n-2);
    }
}

fn main() {
    let n = 7;
    println!("fib({n}) = {}", fib(n));
}
