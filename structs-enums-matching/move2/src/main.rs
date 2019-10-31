// Not working example

// fn dump(s: String){
// 	println!("{}", s);
// }

// fn main() {
//     let s1 = "hello dolly".to_string();

//     dump(s1);
//     println!("s1 {}", s1);
// }

// working example

fn dump(s: &str){
	println!("{}", s);
}

fn main() {
    let s1 = "hello dolly".to_string();

    dump(&s1);
    println!("s1 {}", s1);
}
