use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	let name: &str = if args.len() > 1 {
		&args[1]
	} else {
		"World"
	};

	println!("Hello {} from impr", name);
}
