fn main() {

	let f = |x| x * x;
	let res = f(102);

	let m = 2.0;
	let c = 1.0;
	let lin = |y| m*y + c;

    println!("res {}", res);
    println!("res {} {}", lin(1.0), lin(2.0));

}
