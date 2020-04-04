fn main() {
	println!("Problem 1: {}", problem1());
}

fn problem1() -> i64 {
	return (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
}
