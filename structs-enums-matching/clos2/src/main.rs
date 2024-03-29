fn apply<F>(x: f64, f: F) -> f64
where F: Fn(f64)->f64 {
	f(x)
}

fn main() {
	let m = 2.0;
	let c = 1.0;
	let lin = |y| m*y + c;

	let res1 = apply(3.0, lin);
	let res2 = apply(3.14, |x| x.sin());

    println!("{} {}", res1, res2);
}
