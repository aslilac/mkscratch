use std::{env, fs, io};

fn main() -> io::Result<()> {
    fs::create_dir(".scratch")?;
    fs::write(".scratch/.gitignore", "*\n")?;

    let created_path = {
        let mut path = env::current_dir()?;
        path.push(".scratch");
        String::from(path.to_str().unwrap())
    };

    println!("Created {}", created_path);

    Ok(())
}
