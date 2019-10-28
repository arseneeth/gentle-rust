fn by_ref(x: &i32) -> i32{
	*x + 1
}

fn modifies(x: &mut f64){
	*x = 1.0;
}

fn main() {
	// by_ref
	let i = 10;
	let res1 = by_ref(&i);
	let res2 = by_ref(&41);

    println!("{} {}", res1, res2);

    //modifies
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);
}
