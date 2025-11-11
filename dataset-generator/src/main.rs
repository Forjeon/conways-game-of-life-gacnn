use std::env;
use std::io::BufWriter;
use std::path::Path;

use dataset_generator::{ConwayBoard, run};

fn encode_instance(input: &ConwayBoard, timestep: u32, target: &ConwayBoard) -> String {
	let mut encoding = String::new();
	for cell in input {
		encoding.push_str(match cell {
			true => "1,",
			false => "0,",
		});
	}
	encoding.push_str(&format!("{timestep},"));
	for cell in target {
		encoding.push_str(match cell {
			true => "1,",
			false => "0,",
		});
	}
	encoding.pop();
	encoding
}

fn main() -> Result<String, String> {
	let args: Vec<String> = env::args().collect();
	
	// Validate CLI args
	match args.len() {
		0 => unreachable!(),
		1 => return Err("Missing required outfilepath, num_instances, max_timestep, and sparsity arguments".to_string()),
		2 => {
			match args.get(1).unwrap() {
				"-h" | "--help" => return Ok(format!("Usage: {} outfilepath num_instances max_timestep sparsity", args.get(0).unwrap())),
				_ => return Err("Missing required num_instances, max_timestep, and sparsity arguments".to_string())
			}
		},
		3 => return Err("Missing required num_instances, max_timestep, sparsity arguments".to_string()),
		4 => return Err("Missing required max_timestep and sparsity argument".to_string()),
		5 => return Err("Missing required sparsity argument".to_string()),
		_ => (),
	};

	let outfilepath = Path::new(args.get(1).unwrap());

	if outfilepath.is_dir() {
		return Err("outfilepath must not point to a directory".to_string());
	}

	let num_instances = args.get(2).unwrap().parse::<u32>()?;
	let max_timestep = args.get(3).unwrap().parse::<u32>()?;
	let sparsity = args.get(4).unwrap().parse::<f32>()?;

	if sparsity <= 0.0 || sparsity >= 1.0 {
		return Err("sparsity must be a value between 0 and 1".to_string());
	}

	// Generate instances
	//let instances = run(100_000, 10, 0.25);
	let instances = run(num_instances, max_timestep, sparsity);

	// Write instances
	todo!();
	Ok(format!("Successfully saved {num_instances} generated instances to {}", outfilepath.display()))
}
