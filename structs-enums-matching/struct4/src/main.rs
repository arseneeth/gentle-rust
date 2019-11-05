#[derive(Debug)]
struct Person {
	first_name: String,
	last_name: String
}

impl Person {

	fn new(first: &str, last: &str) -> Person{
		Person{
			first_name: first.to_string(),
	        last_name: last.to_string()
		}
	}

	fn copy(&self) -> Self {
		Self::new(&self.first_name, &self.last_name)
	}

	fn set_first_name(&mut self, name: &str) {
		self.first_name = name.to_string();
	}

	fn to_tuple(self) -> (String, String) {
		(self.first_name, self.last_name)
	}
}

fn main() {
    let person = Person::new("John", "Smith");

    println!("{:?}", person);

    let mut copy = Person::copy(&person); 

    Person::set_first_name(&mut copy, "Jane");

    println!("{:?}", copy);

    println!("{:?}", copy.to_tuple());
}
