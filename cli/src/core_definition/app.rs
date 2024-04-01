use super::args;
use super::CommandNameConstraints;
use super::{Command, CommandExecContext, CommandFlag};
use std::collections::HashMap;
use std::process;

pub struct App<'a, CommandName>
where
    CommandName: CommandNameConstraints,
{
    pub name: String,
    pub description: String,
    pub commands: HashMap<CommandName, Command<'a, CommandName>>,
    pub flags: Vec<&'a CommandFlag>,
    pub arg_parser: args::ArgParser,
}

impl<'a, CommandName> App<'a, CommandName>
where
    CommandName: CommandNameConstraints,
{
    pub fn new(name: String, description: String) -> Self {
        App {
            name,
            description,
            commands: HashMap::new(),
            flags: vec![],
            arg_parser: args::ArgParser::new(),
        }
    }

    pub fn add_flag(mut self, flag: &'static CommandFlag) -> Self {
        self.flags.push(flag);
        self.arg_parser.add_flag(flag);
        self
    }

    pub fn add_command(mut self, cmd: Command<'a, CommandName>) -> Self {
        self.commands.insert(cmd.name.clone(), cmd);
        self
    }

    pub fn run_cmd(&mut self) -> Option<CommandName> {
        let args = match self.arg_parser.parse() {
            Ok(args) => args,
            Err(e) => {
                eprintln!("Error parsing arguments: {}", e);
                process::exit(1);
            }
        };

        let arg = args.get(1)?;
        let cmd_name: CommandName = arg.parse().ok()?;

        match self.commands.get(&cmd_name) {
            None => Some(cmd_name),
            Some(cmd) => {
                (cmd.handler)(CommandExecContext { app: self, args });
                Some(cmd_name)
            }
        }
    }

    pub fn help(&self) {
        println!("{} - {}", self.name, self.description);

        println!();
        println!("Flags:");
        for flag in &self.flags {
            let value = if flag.has_value { "=<value>" } else { "" };
            let short_name = match flag.short_name {
                None => String::from(""),
                Some(v) => format!("-{}, ", v),
            };

            println!(
                "  {}--{}{} - {}",
                short_name, flag.name, value, flag.description
            );
        }

        println!();
        println!("Commands:");
        for cmd in self.commands.values() {
            if cmd.usage.is_empty() {
                println!("  {} - {}", cmd.name, cmd.description);
            } else {
                println!("  {} {} - {}", cmd.name, cmd.usage, cmd.description);
            }
        }
    }
}
