use std::collections::HashSet;

fn make_set(words: &str) -> HashSet<&str> {
	words.split_whitespace().collect()
}

fn main() {
	let v = make_set("remember remember the 5th of November");
	let rain = make_set("Guns N' Roses performing November Rain");

	for i in v.intersection(&rain) {
		println!("{:?}", i);
	}  
}
