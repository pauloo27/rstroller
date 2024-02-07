use std::{collections::HashMap, rc::Rc};

mod command_name;

pub use command_name::CommandName;

pub struct Command<'a> {
    pub name: CommandName,
    pub description: String,
    pub handler: &'a dyn Fn(Rc<Command>),
}

impl<'a> Command<'a> {
    pub fn new(name: CommandName, description: String, handler: &dyn Fn(Rc<Command>)) -> Command {
        Command {
            name,
            description,
            handler,
        }
    }
}

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

    pub fn add_command(mut self, cmd: Rc<Command<'a>>) -> Self {
        self.commands.insert(cmd.name.clone(), cmd);
        self
    }

    pub fn run(&mut self) -> Option<CommandName> {
        let arg = std::env::args().nth(1)?;
        let cmd_name = CommandName::from(&arg)?;

        match self.commands.get(&cmd_name) {
            None => Some(cmd_name),
            Some(cmd) => {
                (cmd.handler)(cmd.clone());
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
