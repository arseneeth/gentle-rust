// Original example from the book:

// fn main() {
// 	let s1 = "hello dolly".to_string();
// 	let s2 = s1;

//     println!("s1 {}", s1);
// }


// Modified example in order to compile and run

fn main() {
	let s1 = "hello dolly".to_string();
	let s2 = &s1;

    println!("s1 {}", s2);
}
