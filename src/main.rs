use anyhow::anyhow;
use std::env;
use std::fs;
use std::io::Write;

mod options;
use options::Options;

fn main() -> anyhow::Result<()> {
	let options = env::args().skip(1).collect::<Options>();

	if options.scratch_directory.exists() {
		// If the path already exists, make sure it's a directory
		if !options.scratch_directory.is_dir() {
			return Err(anyhow!(
				"{} exists and is not a directory",
				options.scratch_directory.display()
			));
		}
	} else {
		// Create the directory if it doesn't exist yet
		fs::create_dir(&options.scratch_directory)?;
	}

	// Create/append to .gitignore
	let mut gitignore = fs::OpenOptions::new()
		.create(true)
		.append(true)
		.open(options.scratch_directory.join(".gitignore"))?;
	writeln!(gitignore, "*")?;

	// Yay! We did it!
	let created_path = env::current_dir()?.join(options.scratch_directory);
	println!("created {}", created_path.display());

	Ok(())
}
