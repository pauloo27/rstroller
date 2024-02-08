use super::CommandFlag;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct ParsedArgs {
    pub args: Vec<String>,
    pub flags: HashMap<String, String>,
}

impl ParsedArgs {
    pub fn get(&self, idx: usize) -> Option<&String> {
        self.args.get(idx)
    }
}

pub struct ArgParser {
    flags: HashMap<String, &'static CommandFlag>,
}

impl ArgParser {
    pub fn new() -> Self {
        ArgParser {
            flags: HashMap::new(),
        }
    }

    pub fn add_flag(&mut self, flag: &'static CommandFlag) {
        self.flags
            .insert(format!("--{}", flag.name.to_string()), flag);
        if let Some(short_name) = flag.short_name {
            self.flags
                .insert(format!("-{}", short_name.to_string()), flag);
        }
    }

    pub fn parse(&self) -> Result<ParsedArgs, String> {
        let mut args = vec![];
        let mut flags = HashMap::new();

        for arg in env::args() {
            match self.parse_flag(&arg)? {
                Some((name, value)) => {
                    flags.insert(name, value);
                }
                None => args.push(arg),
            }
        }

        Ok(ParsedArgs { args, flags })
    }

    fn parse_flag(&self, arg: &str) -> Result<Option<(String, String)>, String> {
        if !arg.starts_with("-") {
            return Ok(None);
        }

        let splitted_arg = arg.split("=").collect::<Vec<&str>>();
        let flag_prefix = splitted_arg
            .get(0)
            .map(|s| s.to_string())
            .ok_or("Invalid flag")?;

        let flag_value = splitted_arg.get(1).map(|s| s.to_string());

        let flag = match self.flags.get(&flag_prefix) {
            None => return Err(format!("Unknown flag: {}", flag_prefix)),
            Some(flag) => flag,
        };

        if flag.has_value != flag_value.is_some() {
            let expected = flag.has_value;
            Err(format!(
                "Flag {} {} value",
                flag.name,
                if expected {
                    "requires"
                } else {
                    "does not require"
                },
            ))?;
        }

        Ok(Some((
            flag.name.to_string(),
            flag_value.unwrap_or("".to_string()),
        )))
    }
}
