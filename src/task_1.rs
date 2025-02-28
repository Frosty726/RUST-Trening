fn fib(n: u32) -> u32 {
	if n < 2 {
		1
	} else {
		fib(n - 1) + fib(n - 2)
	}
}

pub fn demonstrate() {
	let n = 20;
    println!("--- Task 1 ---");
    println!("fib({n}) = {}", fib(n));
    println!();
}