use crate::fmt::render;

pub enum Exc {
	/// wraps all occurrences of errors in I/O operations
	IoError(std::io::Error),
	ConfError(figment::Error),
}

impl ToString for Exc {
	fn to_string(&self) -> String {
		let attn = "<bold red>error:</>";
		match self {
			Exc::IoError(err) => format!("{attn} {err}"),
			Exc::ConfError(err) => format!("{attn} {err}"),
		}
	}
}

impl Exc {
	/// Print the error message to STDOUT, rendering any markup that the error
	/// message may contain.
	pub fn print(&self) {
		println!("{}", render(self.to_string()));
	}
}
