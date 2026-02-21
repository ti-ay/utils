use time_it::time_it;

fn main() {
    let _result = time_it!(fib(40));
    let _result = time_it!(fib(47));

    let _result = time_it!(fib(30));

    let _result = time_it!("Fibonacci of 40", fib(40));

    println!("Done");
}

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
