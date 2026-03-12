fn greet(name: &str) -> String {
	// say hello
	format!("Hello, {name}!")
}

fn main() {
	println!("{}", greet("world"));
}
