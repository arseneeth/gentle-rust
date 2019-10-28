fn sqr(x: f64) -> f64 {
	x * x
}

fn factorial(n: f64) -> f64 {
	if n == 0.0 {
		1.0
	} else {
		n * factorial(n-1.0)
	}
}

fn main() {
	let res = sqr(2.0);
	let fac = factorial(res);
    println!("square is {}", res);
	println!("factorial is {}", fac);
}
