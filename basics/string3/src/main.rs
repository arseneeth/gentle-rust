fn main() {
	let multilingual = "Hi! ¡Hola! привет!";
	for ch in multilingual.chars(){
		println!("{}", ch);
	}
	println!("");
	println!("len {}", multilingual.len());
	println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
    	let hi = &multilingual[maybe.unwrap()..];
    	println!("Russian hi {}", hi);
    }

    let words: Vec<&str> = multilingual.split_whitespace().collect();

    println!("words: {:?}", words);

    let stripped: String = multilingual.chars()
    	.filter(|ch| ! ch.is_whitespace()).collect();

    println!("stripped: {:?}", stripped);
}
