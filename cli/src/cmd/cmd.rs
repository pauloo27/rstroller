use super::{App, CommandNameConstraints, ParsedArgs};

pub struct CommandExecContext<'a, CommandName>
where
    CommandName: CommandNameConstraints,
{
    pub app: &'a App<'a, CommandName>,
    pub args: ParsedArgs,
}

#[derive(Debug)]
pub struct CommandFlag {
    pub name: &'static str,
    pub short_name: Option<&'static str>,
    pub description: &'static str,
    pub has_value: bool,
}

impl CommandFlag {
    pub fn new(
        name: &'static str,
        short_name: Option<&'static str>,
        description: &'static str,
        has_value: bool,
    ) -> Self {
        CommandFlag {
            name,
            short_name,
            description,
            has_value,
        }
    }
}

pub struct Command<'a, CommandName>
where
    CommandName: CommandNameConstraints,
{
    pub name: CommandName,
    pub description: &'a str,
    pub usage: &'a str,
    pub handler: &'a dyn Fn(CommandExecContext<CommandName>),
}

impl<'a, CommandName> Command<'a, CommandName>
where
    CommandName: CommandNameConstraints,
{
    pub fn new(
        name: CommandName,
        description: &'a str,
        handler: &'a dyn Fn(CommandExecContext<CommandName>),
    ) -> Command<'a, CommandName> {
        Command {
            name,
            usage: "",
            description,
            handler,
        }
    }

    pub fn new_with_usage(
        name: CommandName,
        usage: &'a str,
        description: &'a str,
        handler: &'a dyn Fn(CommandExecContext<CommandName>),
    ) -> Command<'a, CommandName> {
        Command {
            name,
            usage,
            description,
            handler,
        }
    }
}
