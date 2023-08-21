use std::fmt;
use std::fmt::Formatter;

use serde::de::Visitor;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    pub package:       Package,
    pub configuration: Configuration,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            package:       Package {
                name: "my_project".to_owned(),
                kind: ProjectKind::Binary,
            },
            configuration: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ProjectKind,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectKind {
    Binary,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "config")]
pub struct Configuration {
    #[serde(rename = "c-standard")]
    pub standard:     Standard,
    #[serde(flatten)]
    pub optimization: Optimization,
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

impl Project {
    pub fn with_name<T>(name: T) -> Self
    where
        T: ToString,
    {
        let name = name.to_string();
        let mut project = Project::default();
        project.package.name = name;
        project
    }
}
