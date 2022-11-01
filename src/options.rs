use colored::Colorize;
use std::path::PathBuf;
use std::process::exit;

#[derive(Clone, Debug, Default)]
struct OptionsBuilder {
	scratch_dir: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct Options {
	pub scratch_directory: PathBuf,
}

impl From<OptionsBuilder> for Options {
	fn from(builder: OptionsBuilder) -> Self {
		Options {
			scratch_directory: builder
				.scratch_dir
				.unwrap_or_else(|| PathBuf::from(".scratch/")),
		}
	}
}

impl<S> FromIterator<S> for Options
where
	S: AsRef<str>,
{
	fn from_iter<I>(args: I) -> Self
	where
		I: IntoIterator<Item = S>,
	{
		let mut options = OptionsBuilder::default();
		let mut args = args.into_iter();

		while let Some(arg) = args.next() {
			let arg = arg.as_ref();
			if (arg.len() >= 2 && arg.starts_with('-')) || arg.len() >= 3 && arg.starts_with("--") {
				match arg {
					"-v" | "-V" | "-version" | "--version" => {
						println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
						exit(0);
					}
					"-h" | "-help" | "--help" | "-?" => {
						println!(
							"{} {}",
							env!("CARGO_PKG_NAME").bold().magenta(),
							env!("CARGO_PKG_VERSION").bold().magenta()
						);
						println!();
						println!("usage: mkscratch [directory]");
						println!("       defaults to creating a .scratch/ directory");
						exit(0);
					}
					_ => {
						eprintln!(
							"{} {}",
							"error:".red(),
							format!("unrecognized option: {}", arg)
						);
						exit(1);
					}
				}
			} else {
				options.scratch_dir = Some(PathBuf::from(arg));
			}
		}

		options.into()
	}
}
