use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn read_to_string(filename: &str) -> Result<String, io::Error> {
	let mut file = File::open(&filename)?;
	let mut text = String::new();
	file.read_to_string(&mut text)?;
	Ok(text)
}

fn main() {
    let file = env::args().nth(1).expect("please supply a filename");

    let text = read_to_string(&file).expect("bad file name!");

    println!("file had {} bytes", text.len());
}
