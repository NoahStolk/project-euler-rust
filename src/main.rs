fn main() {
	println!("Problem 1: {}", problem1());
	println!("Problem 2: {}", problem2());
}

fn problem1() -> i64 {
	return (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
}

fn problem2() -> i64 {
	let fib_seq = (1..fib_max_val(4_000_000)).map(|x| fib(x));
	return fib_seq.filter(|x| x % 2 == 0).sum();
}

fn fib(n: i64) -> i64 {
	match n {
		0 => 1,
		1 => 1,
		_ => fib(n - 1) + fib(n - 2),
	}
}

fn fib_max_val(n: i64) -> i64 {
	for i in 0.. {
		if fib(i) > n {
			return i;
		}
	}
	return 0;
}
