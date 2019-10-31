// example doesn't work

// fn main() {
    // let s1 = "hello dolly".to_string();
    // let mut rs1 = &s1;
    // {
    // 	let tmp = "hello world".to_string();
    // 	rs1 = &tmp;
    // } 

    // println!("ref {}", rs1);
// }

// modified in order to work

fn main() {
    let s1 = "hello dolly".to_string();
    let mut rs1 = &s1;
    {
    	let tmp = "hello world".to_string();
    	rs1 = &tmp;
    	println!("ref {}", rs1);
    } 

    // println!("ref {}", rs1);
}

