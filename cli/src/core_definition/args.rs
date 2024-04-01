use super::CommandFlag;
use anyhow::Result as AnyResult;
use anyhow::{anyhow, Context};
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
        self.flags.insert(format!("--{}", flag.name), flag);
        if let Some(short_name) = flag.short_name {
            self.flags.insert(format!("-{}", short_name), flag);
        }
    }

    pub fn parse(&self) -> AnyResult<ParsedArgs> {
        self.parse_from_iter(env::args())
    }

    pub fn parse_from_iter<I: IntoIterator<Item = String>>(
        &self,
        iter: I,
    ) -> AnyResult<ParsedArgs> {
        let mut args = vec![];
        let mut flags = HashMap::new();

        for arg in iter {
            match self.parse_flag(&arg).context("failed to parse flag")? {
                Some((name, value)) => {
                    flags.insert(name, value);
                }
                None => args.push(arg),
            }
        }

        Ok(ParsedArgs { args, flags })
    }

    fn parse_flag(&self, arg: &str) -> AnyResult<Option<(String, String)>> {
        if !arg.starts_with('-') {
            return Ok(None);
        }

        let splitted_arg = arg.split('=').collect::<Vec<&str>>();
        let flag_prefix = splitted_arg
            .first()
            .map(|s| s.to_string())
            .context("invalid flag")?;

        let flag_value = splitted_arg.get(1).map(|s| s.to_string());

        let flag = match self.flags.get(&flag_prefix) {
            None => return Err(anyhow!("Unknown flag: {}", flag_prefix)),
            Some(flag) => flag,
        };

        if flag.has_value != flag_value.is_some() {
            let expected = flag.has_value;
            Err(anyhow!(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_definition::CommandFlag;

    #[test]
    fn test_parse_flag() {
        let args = vec![
            "/usr/bin/rstroller".to_string(),
            "--player=player1".to_string(),
            "-s".to_string(),
            "test".to_string(),
        ];

        let mut parser = ArgParser::new();

        parser.add_flag(&CommandFlag {
            name: "player",
            description: "sample flag with value",
            short_name: Some("p"),
            has_value: true,
        });
        parser.add_flag(&CommandFlag {
            name: "sample",
            description: "sample flag without value",
            short_name: Some("s"),
            has_value: false,
        });

        let parsed = parser.parse_from_iter(args).unwrap();
        assert_eq!(
            parsed.args,
            vec!["/usr/bin/rstroller".to_string(), "test".to_string()]
        );
        assert_eq!(parsed.flags.get("player"), Some(&"player1".to_string()));
    }
}
