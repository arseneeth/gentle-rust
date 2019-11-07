fn main() {
	let t = (10, "hello".to_string());

	let (ref n, ref s) = t; 

    println!("{} {} {:?}", n, s, t);
}
