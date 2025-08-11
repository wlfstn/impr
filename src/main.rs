use std::env;

const VERSION: &str = "1.0.0";

fn main() {
	let args: Vec<String> = env::args().collect();

	let mut inches: u32 = 0;
	let mut sections: u32 = 1;
	let mut dimension = None;

	let mut onlyInches = false;

	let mut i: usize = 1;
	while i < args.len() {
		match args[i].as_str() {
			"-v" | "--version" => {
				println!("IMP version: {}", VERSION);
				return;
			}
			"-in" => {
				onlyInches = true;
			}
			"-d" => {
				i += 1;
				dimension = args.get(i).cloned();
			}
			"-s" => {
				i += 1;
				sections = args.get(i).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1);
			}
			_ => {}
		}

		if sections < 1 {
			eprintln!("Sections must be one or greater.");
			return;
		}
	}
}
