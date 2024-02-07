use super::{Command, CommandExecContext, CommandName};
use std::{collections::HashMap, rc::Rc};

pub struct App<'a> {
    pub name: String,
    pub description: String,
    pub commands: HashMap<CommandName, Rc<Command<'a>>>,
}

impl<'a> App<'a> {
    pub fn new(name: String, description: String) -> Self {
        App {
            name,
            description,
            commands: HashMap::new(),
        }
    }

    pub fn add_command(mut self, cmd: Command<'a>) -> Self {
        let cmd_ref = Rc::new(cmd);
        self.commands.insert(cmd_ref.name.clone(), cmd_ref);
        self
    }

    pub fn run_cmd(&mut self) -> Option<CommandName> {
        let arg = std::env::args().nth(1)?;
        let cmd_name: CommandName = arg.parse().ok()?;

        match self.commands.get(&cmd_name) {
            None => Some(cmd_name),
            Some(cmd) => {
                (cmd.handler)(CommandExecContext { app: self });
                Some(cmd_name)
            }
        }
    }

    pub fn help(&self) {
        println!("-> {} - {}", self.name, self.description);
        for (_, cmd) in &self.commands {
            println!("    -> {} - {}", cmd.name, cmd.description);
        }
    }
}
