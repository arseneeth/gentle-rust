fn main() {
	let ints = [1, 2, 3, 4, 5];
	let slice = &ints;
	let first = slice.get(0);
	let last = slice.get(5);
	let maybe_last = *slice.get(5).unwrap_or(&-1);

    println!("first: {} {}", first.is_some(), first.is_none());
    println!("last: {} {}", last.is_some(), last.is_none());
    println!("maybe_last: {}", maybe_last);
    println!("first value: {}", first.unwrap());
}
