use crate::{
    config::{find_val, ConfigValue},
    error,
    errors::Result,
};
use std::fmt::{self, Display, Formatter};

const DEFAULT_COMPILER: &str = "cc";
const DEFAULT_FLAGS: [&str; 4] = [
    "-Wall",
    "-Wextra",
    "-Wwrite-strings",
    "-Werror=discarded-qualifiers",
];
const DEFAULT_STANDARD: Standard = Standard {
    std: Std::C99,
    gnu_extensions: false,
};
const DEFAULT_PTYPE: ProjectType = ProjectType::Binary;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Std {
    C89 = 89,
    C99 = 99,
    C11 = 11,
    C17 = 17,
    C23 = 23,
}
pub struct Standard {
    std: Std,
    gnu_extensions: bool,
}
impl Display for Standard {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{}{}",
                if self.gnu_extensions { "gnu" } else { "c" },
                self.std as u8
            )
            .replace("23", "2x")
        )
    }
}
pub enum ProjectType {
    Binary,
    Shared,
    Static,
}
pub struct Project {
    pub name: String,
    pub version: String,
    pub standard: Standard,
    pub compiler: String,
    pub flags: Vec<String>,
    pub ptype: ProjectType,
}
impl Display for Project {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "CC       {}", self.compiler)?;
        writeln!(
            f,
            "CFLAGS   {}-std={}",
            self.flags
                .iter()
                .fold("".to_string(), |acc, v| format!("{}{} ", acc, v)),
            self.standard
        )?;
        writeln!(
            f,
            "TYPE     {}",
            match self.ptype {
                ProjectType::Binary => "BIN",
                ProjectType::Shared => "SHARED",
                ProjectType::Static => "STATIC",
            }
        )?;
        writeln!(f, "NAME     {}", self.name)?;
        write!(f, "VERSION  {}", self.version)
    }
}
impl Project {
    pub fn from_config(vals: Vec<ConfigValue>) -> Result<Self> {
        let name = if let Some(ConfigValue::Array(av)) = find_val(&vals, "name") {
            get_first(&av, "name")
        } else {
            error!("Key `name` must be a single string.")
        }?;
        let version = if let Some(ConfigValue::Array(av)) = find_val(&vals, "version") {
            get_first(&av, "version")
        } else {
            error!("Key `version` must be a single string.")
        }?;
        let standard = match find_val(&vals, "standard") {
            None => Ok(DEFAULT_STANDARD),
            Some(ConfigValue::Array(av)) => {
                let raw = get_first(&av, "standard")?;
                if raw.as_str() == "ansi" {
                    Ok(Standard {
                        gnu_extensions: false,
                        std: Std::C89,
                    })
                } else {
                    let prefix = if raw.starts_with("gnu") { "gnu" } else { "c" };

                    let standards = &[Std::C89, Std::C99, Std::C11, Std::C17, Std::C23];

                    Ok(Standard {
                        gnu_extensions: prefix == "gnu",
                        std: standards
                            .iter()
                            .filter_map(|s| {
                                if format!("{}{}", prefix, *s as u8) == raw {
                                    Some(*s)
                                } else {
                                    None
                                }
                            })
                            .next()
                            .map_or(
                                error!(
                                    "`{}` is not a valid C standard. Valid standards are: {}",
                                    raw,
                                    standards.iter().fold("ansi".to_string(), |acc, v| format!(
                                        "{}, c{}, gnu{}",
                                        acc, *v as u8, *v as u8
                                    ))
                                ),
                                Ok,
                            )?,
                    })
                }
            }
            _ => error!("Key `standard` must be a single string."),
        }?;
        let compiler = match find_val(&vals, "cc") {
            None => Ok(DEFAULT_COMPILER.to_string()),
            Some(ConfigValue::Array(av)) => get_first(&av, "cc"),
            _ => error!("Key `cc` must be a single string."),
        }?;
        let flags = match find_val(&vals, "flags") {
            None => Ok(DEFAULT_FLAGS.iter().map(|s| s.to_string()).collect()),
            Some(ConfigValue::Array(av)) => {
                let mut flags = vec![];
                for value in av {
                    if let ConfigValue::Ident(flag) = value {
                        flags.push(flag);
                    } else {
                        return error!("Each flag must be an identifier.");
                    }
                }
                Ok(flags)
            }
            _ => error!("Key `flags` must be an array."),
        }?;
        let ptype = match find_val(&vals, "type") {
            None => Ok(DEFAULT_PTYPE),
            Some(ConfigValue::Array(av)) => match get_first(&av, "type")?.as_str() {
                "binary" => Ok(ProjectType::Binary),
                "shared" => Ok(ProjectType::Shared),
                "static" => Ok(ProjectType::Static),
                x => error!("`{}` is not a valid project type. Available project types: binary, shared, static.", x),
            },
            _ => error!("Key `type` must be a single string."),
        }?;

        Ok(Self {
            name,
            version,
            standard,
            compiler,
            flags,
            ptype,
        })
    }
}
fn get_first(av: &[ConfigValue], k: impl ToString) -> Result<String> {
    let k = k.to_string();
    if av.len() == 1 {
        if let ConfigValue::Ident(name) = &av[0] {
            Ok(name.to_string())
        } else {
            error!("Key `{}` must be a single string.", k)
        }
    } else {
        error!("Key `{}` must be a single string.", k)
    }
}
