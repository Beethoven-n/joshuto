extern crate toml;
extern crate xdg;

use std::fmt;
use std::fs;
use std::collections::HashMap;
use std::process;

#[derive(Debug, Deserialize)]
pub struct JoshutoMimetypeEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
    pub fork: Option<bool>,
    pub silent: Option<bool>,
}

impl std::fmt::Display for JoshutoMimetypeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(self.program.as_str()).unwrap();
        if let Some(s) = self.args.as_ref() {
            for arg in s {
                write!(f, " {}", arg).unwrap();
            }
        }
        f.write_str("\t[").unwrap();
        if let Some(s) = self.fork {
            if s {
                f.write_str("fork,").unwrap();
            }
        }
        if let Some(s) = self.silent {
            if s {
                f.write_str("silent").unwrap();
            }
        }
        f.write_str("]")
    }
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawMimetype {
    mimetype: Option<HashMap<String, Vec<JoshutoMimetypeEntry>>>,
    extension: Option<HashMap<String, Vec<JoshutoMimetypeEntry>>>,
}

impl JoshutoRawMimetype {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoRawMimetype {
            mimetype: None,
            extension: None,
        }
    }

    pub fn flatten(self) -> JoshutoMimetype
    {
        let mimetype = self.mimetype.unwrap_or(HashMap::new());
        let extension = self.extension.unwrap_or(HashMap::new());

        JoshutoMimetype {
            mimetype,
            extension
        }
    }
}

#[derive(Debug)]
pub struct JoshutoMimetype {
    pub mimetype: HashMap<String, Vec<JoshutoMimetypeEntry>>,
    pub extension: HashMap<String, Vec<JoshutoMimetypeEntry>>,
}

impl JoshutoMimetype {

    pub fn new() -> Self
    {
        JoshutoMimetype {
            mimetype: HashMap::new(),
            extension: HashMap::new(),
        }
    }

    fn read_config() -> Option<JoshutoRawMimetype>
    {
        match xdg::BaseDirectories::with_profile(::PROGRAM_NAME, "") {
            Ok(dirs) => {
                let config_path = dirs.find_config_file(::MIMETYPE_FILE)?;
                match fs::read_to_string(&config_path) {
                    Ok(config_contents) => {
                        match toml::from_str(&config_contents) {
                            Ok(config) => {
                                Some(config)
                            },
                            Err(e) => {
                                eprintln!("Error parsing mimetype file: {}", e);
                                process::exit(1);
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    },
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                None
            },
        }
    }

    pub fn get_config() -> Self
    {
        match Self::read_config() {
            Some(config) => {
                config.flatten()
            }
            None => {
                Self::new()
            }
        }
    }
}
