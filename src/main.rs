use std::env;

const VERSION: &str = "1.3.0"; // Fixes the 12\" bug

fn main() {
	let args: Vec<String> = env::args().collect();

	let mut dimension = None;
	let mut sections = 1;
	let mut total_inches = false;

	let mut i = 1;
	while i < args.len() {
		match args[i].as_str() {
			"-d" => {
				i += 1;
				dimension = args.get(i).cloned();
			}
			"-s" => {
				i += 1;
				sections = args.get(i).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1);
			}
			"-in" => {
				total_inches = true;
			}
			"-v" | "--version" => {
				println!("IMP version: {}", VERSION);
				return;
			}
			_ => {}
		}
		i += 1;
	}

	let dim_str = match dimension {
		Some(d) => d,
		None => {
			eprintln!("Please provide a dimension using the -d flag, e.g., -d 8f6");
			return;
		}
	};

	if sections == 0 {
		eprintln!("Number of sections must be a positive integer.");
		return;
	}

	let total_feet = match parse_dimension(&dim_str) {
		Ok(feet) => feet,
		Err(e) => {
			eprintln!("Error parsing dimension: {}", e);
			return;
		}
	};

	if total_inches {
		let total_inches = (total_feet * 12.0).round();
		println!("Total inches: {}", total_inches);
		return;
	}

	let section_size_feet = total_feet / sections as f64;
	let (feet, inches) = split_to_feet_and_inches(section_size_feet);
	println!("Each section size: {}'{}\"", feet, inches);
}

fn parse_dimension(dim_str: &str) -> Result<f64, String> {
	let mut dim = dim_str.to_lowercase().replace([' ', '\'', '"'], "");
	dim = dim.replace("ft", "f").replace("in", "i");

	let parts: Vec<&str> = dim.split('f').collect();
	if parts.len() == 2 {
		let feet: f64 = parts[0].parse().map_err(|_| "Invalid feet value")?;
		let inches: f64 = if parts[1].is_empty() {
			0.0
		} else {
			parts[1].replace('i', "").parse().map_err(|_| "Invalid inches value")?
		};
		Ok(feet + inches / 12.0)
	} else {
		Err("Invalid dimension format".into())
	}
}

fn split_to_feet_and_inches(total_feet: f64) -> (u32, u32) {
	let feet = total_feet.floor() as u32;
	let inches = ((total_feet - feet as f64) * 12.0).ceil() as u32;
	if inches == 12 {
		(feet + 1, 0)
	} else {
		(feet, inches)
	}
}
