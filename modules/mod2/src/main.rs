mod foo {
    #[derive(Debug)]
    pub struct Foo {
        s: &'static str
    }

    impl Foo{
    	pub fn new(st: &'static str) -> Foo{
    		Foo{s: st}
    	}
    }
}

fn main() {
    let f = foo::Foo::new("hello");
    println!("{:?}", f);
}
