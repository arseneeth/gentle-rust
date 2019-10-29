fn main() {
	let multilingual = "Hi! ¡Hola! привет!";

	match multilingual.find('п') {
		Some(idx) => {
            let hi = &multilingual[idx..];
            println!("Russian hi {}", hi);			
		},
		None => println!("couldn't find the greeting, Товарищ")
	}

	let n = 1;

    let mut text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many",
    };

    println!("{}", text);

    text = match n {
        0...3 => "small",
        4...6 => "medium",
        _ => "large",
     };

    println!("{}", text);
}
