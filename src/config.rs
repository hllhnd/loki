use std::fmt;
use std::fmt::Formatter;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

use color_eyre::Report;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Serialize;

const INIT_CODE: &str = r#"#include <stdio.h>

int main(void) {
    printf("Hello, world!\n");
    return 0;
}
"#;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Project {
    pub package:       Package,
    pub configuration: Configuration,
}

impl Project {
    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            package:       Package {
                name: name.into(),
                ..Default::default()
            },
            configuration: Default::default(),
        }
    }

    /// Generate a new project based on `self` at the given path.
    ///
    /// # Errors
    /// This method will error if the project directory exists but is non-empty, or if a miscellaneous I/O error occurs.
    pub fn generate_at(self, root_path: impl AsRef<Path>) -> Result<(), Report> {
        let loki_toml_path = root_path.as_ref().join("loki.toml");
        let source_path = root_path.as_ref().join("src");
        let main_path = source_path.join("main.c");
        match fs::create_dir(root_path.as_ref()) {
            Ok(()) => (),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                if fs::read_dir(root_path)?.next().is_some() {
                    // TODO: this should be a custom error type but a panic is good enough for now
                    panic!("Directory is not empty");
                }
            },
            Err(e) => return Err(e.into()),
        }

        fs::write(loki_toml_path, toml::to_string(&self)?)?;
        fs::create_dir(source_path)?;
        fs::write(main_path, INIT_CODE)?;

        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ProjectKind,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectKind {
    #[default]
    Binary,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "config")]
pub struct Configuration {
    pub compiler:     PathBuf,
    #[serde(rename = "c-standard")]
    pub standard:     Standard,
    #[serde(flatten)]
    pub optimization: Optimization,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            compiler:     PathBuf::from("clang"),
            standard:     Default::default(),
            optimization: Default::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Standard {
    #[default]
    C89,
    C99,
    C11,
    C17,
    C23,
    Gnu89,
    Gnu99,
    Gnu11,
    Gnu17,
    Gnu23,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Optimization {
    #[serde(rename = "opt-level")]
    pub level: OptimizationLevel,
    pub lto:   Option<Lto>,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum OptimizationLevel {
    #[default]
    O0,
    O1,
    O2,
    O3,
    Og,
    Os,
    Oz,
    Ofast,
}

impl Serialize for OptimizationLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            OptimizationLevel::O0 => serializer.serialize_i64(0),
            OptimizationLevel::O1 => serializer.serialize_i64(1),
            OptimizationLevel::O2 => serializer.serialize_i64(2),
            OptimizationLevel::O3 => serializer.serialize_i64(3),
            OptimizationLevel::Og => serializer.serialize_str("g"),
            OptimizationLevel::Os => serializer.serialize_str("s"),
            OptimizationLevel::Oz => serializer.serialize_str("z"),
            OptimizationLevel::Ofast => serializer.serialize_str("fast"),
        }
    }
}

impl<'de> Deserialize<'de> for OptimizationLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const ERROR: &str = "an integer or string that is one of 0, 1, 2, 3, \"g\", \"s\", \"z\", or \"fast\"";

        struct OptimizationLevelVisitor;

        impl Visitor<'_> for OptimizationLevelVisitor {
            type Value = OptimizationLevel;

            fn expecting(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
                formatter.write_str(ERROR)
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(match value {
                    0 => OptimizationLevel::O0,
                    1 => OptimizationLevel::O1,
                    2 => OptimizationLevel::O2,
                    3 => OptimizationLevel::O3,
                    _ => return Err(E::custom(ERROR)),
                })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(match value {
                    "g" => OptimizationLevel::Og,
                    "s" => OptimizationLevel::Os,
                    "z" => OptimizationLevel::Oz,
                    "fast" => OptimizationLevel::Ofast,
                    _ => return Err(E::custom(ERROR)),
                })
            }
        }

        deserializer.deserialize_any(OptimizationLevelVisitor)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Lto {
    Full,
    Thin,
}
