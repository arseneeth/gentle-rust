fn main() {
	let tuples = [(10,"ten"),(20,"twenty"),(30, "thirty"),(40,"forty")];
	let iter = tuples.iter().filter(|t| t.0 > 20).map(|t| t.1);

	for name in iter {
    	println!("{}", name);
	}
}
