use std::collections::HashSet;
use std::hash::Hash;


trait ToSet<T> {
    fn to_set(self) -> HashSet<T>;
}

impl <T,I> ToSet<T> for I
where T: Eq + Hash, I: Iterator<Item=T> {

    fn to_set(self) -> HashSet<T> {
       self.collect()
    }
}

fn make_set(words: &str) -> HashSet<&str> {
	words.split_whitespace().collect()
}


fn main() {
	let v = make_set("remember remember the 5th of November");
	let rain = make_set("Guns N' Roses performing November Rain");
    let intersect = v.intersection(&rain).to_set();

    println!("{:?}", intersect);
}
