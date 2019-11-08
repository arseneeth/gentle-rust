
fn match_tuple(t: (i32, String)) {
	let text = match t {
		(0, s) => format!("zero {}", s),
		(1, ref s) if s == "hello" =>format!("hello one!"),
		tt => format!("no match {:?}", tt)
		// or say _ => format!("no match") if you're not interested in the value
	};
	println!("{}", text);
}

fn main() {
	let tx = (1, "hello".to_string());
	match_tuple(tx);
}
