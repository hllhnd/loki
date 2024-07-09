use std::error::Error;
use std::str::FromStr;

use indoc::indoc;

/// The type of a package.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PackageKind {
	/// An executable package.
	Application,
	/// A library package.
	Library,
}

impl FromStr for PackageKind {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"application" => Ok(PackageKind::Application),
			"library" => Ok(PackageKind::Library),
			_ => Err(()),
		}
	}
}

impl From<PackageKind> for &str {
	fn from(kind: PackageKind) -> &'static str {
		match kind {
			PackageKind::Application => "application",
			PackageKind::Library => "library",
		}
	}
}

impl PackageKind {
	pub fn as_str(&self) -> &str {
		Into::<&str>::into(*self)
	}
}

/// The C standard as published by ISO, with optional GNU extensions.
///
/// All variants correspond to an -std= flag for GCC and Clang.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Standard {
	/// The C89 standard, published as ISO/IEC 9899:1990.
	C89,
	/// The C99 standard, published as ISO/IEC 9899:1999.
	C99,
	/// The C11 standard, published as ISO/IEC 9899:2011.
	C11,
	/// The C17 standard, published as ISO/IEC 9899:2018.
	C17,
	/// The C23 standard, to be published as ISO/IEC 9899:2024.
	C23,

	/// The C89 standard, published as ISO/IEC 9899:1990, with GNU extensions.
	Gnu89,
	/// The C99 standard, published as ISO/IEC 9899:1999, with GNU extensions.
	Gnu99,
	/// The C11 standard, published as ISO/IEC 9899:2011, with GNU extensions.
	Gnu11,
	/// The C17 standard, published as ISO/IEC 9899:2018, with GNU extensions.
	Gnu17,
	/// The C23 standard, to be published as ISO/IEC 9899:2024, with GNU extensions.
	Gnu23,
}

impl FromStr for Standard {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"c89" => Ok(Standard::C89),
			"c99" => Ok(Standard::C99),
			"c11" => Ok(Standard::C11),
			"c17" => Ok(Standard::C17),
			"c23" => Ok(Standard::C23),

			"gnu89" => Ok(Standard::Gnu89),
			"gnu99" => Ok(Standard::Gnu99),
			"gnu11" => Ok(Standard::Gnu11),
			"gnu17" => Ok(Standard::Gnu17),
			"gnu23" => Ok(Standard::Gnu23),

			_ => Err(()),
		}
	}
}

impl From<Standard> for &str {
	fn from(standard: Standard) -> &'static str {
		match standard {
			Standard::C89 => "c89",
			Standard::C99 => "c99",
			Standard::C11 => "c11",
			Standard::C17 => "c17",
			Standard::C23 => "c23",

			Standard::Gnu89 => "gnu89",
			Standard::Gnu99 => "gnu99",
			Standard::Gnu11 => "gnu11",
			Standard::Gnu17 => "gnu17",
			Standard::Gnu23 => "gnu23",
		}
	}
}

impl Standard {
	pub fn as_str(&self) -> &str {
		Into::<&str>::into(*self)
	}
}

/// The metadata of a package.
#[derive(Clone, Debug, PartialEq)]
pub struct Package {
	/// The name of the package.
	pub name:     String,
	/// The type of the package.
	pub kind:     PackageKind,
	/// The version of the package.
	pub version:  String,
	/// The C standard this package is written in.
	pub standard: Standard,
}

impl Package {
	/// Parse the package metadata from a TOML document.
	pub fn parse(toml: impl AsRef<str>) -> Result<Self, Box<dyn Error + Send + Sync>> {
		let table = toml::from_str::<toml::Value>(toml.as_ref())?;

		let package = table
			.get("package")
			.ok_or("missing package table")?
			.as_table()
			.ok_or("package is not a table")?;

		let name = package
			.get("name")
			.ok_or("missing name field")?
			.as_str()
			.ok_or("name is not a string")?;

		let kind = package
			.get("type")
			.ok_or("missing type field")?
			.as_str()
			.ok_or("type is not a string")?;

		let kind = PackageKind::from_str(kind).map_err(|_| "unknown package type")?;

		let version = package
			.get("version")
			.ok_or("missing version field")?
			.as_str()
			.ok_or("version is not a string")?;

		let standard = package
			.get("standard")
			.ok_or("missing standard field")?
			.as_str()
			.ok_or("standard is not a string")?;

		let standard = Standard::from_str(standard).map_err(|_| "unknown standard")?;

		Ok(Package {
			name:     name.to_string(),
			kind:     kind,
			version:  version.to_string(),
			standard: standard,
		})
	}

	/// Serialize the package metadata to a TOML document.
	pub fn serialize(&self) -> String {
		format!(
			indoc! {r#"
				[package]
				name     = "{}"
				type     = "{}"
				version  = "{}"
				standard = "{}"
			"#},
			self.name,
			self.kind.as_str(),
			self.version,
			self.standard.as_str(),
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_package_kind() {
		assert_eq!(PackageKind::from_str("application"), Ok(PackageKind::Application));
		assert_eq!(PackageKind::from_str("library"), Ok(PackageKind::Library));
		assert_eq!(PackageKind::from_str("unknown"), Err(()));
	}

	#[test]
	fn test_standard() {
		assert_eq!(Standard::from_str("c89"), Ok(Standard::C89));
		assert_eq!(Standard::from_str("c99"), Ok(Standard::C99));
		assert_eq!(Standard::from_str("c11"), Ok(Standard::C11));
		assert_eq!(Standard::from_str("c17"), Ok(Standard::C17));
		assert_eq!(Standard::from_str("c23"), Ok(Standard::C23));

		assert_eq!(Standard::from_str("gnu89"), Ok(Standard::Gnu89));
		assert_eq!(Standard::from_str("gnu99"), Ok(Standard::Gnu99));
		assert_eq!(Standard::from_str("gnu11"), Ok(Standard::Gnu11));
		assert_eq!(Standard::from_str("gnu17"), Ok(Standard::Gnu17));
		assert_eq!(Standard::from_str("gnu23"), Ok(Standard::Gnu23));

		assert_eq!(Standard::from_str("unknown"), Err(()));
	}

	#[test]
	fn test_package() {
		let toml = r#"
			[package]
			name = "test"
			type = "application"
			version = "0.1.0"
			standard = "c23"
		"#;

		let package = Package::parse(toml).unwrap();

		assert_eq!(package.name, "test");
		assert_eq!(package.kind, PackageKind::Application);
		assert_eq!(package.version, "0.1.0");
		assert_eq!(package.standard, Standard::C23);
	}
}
