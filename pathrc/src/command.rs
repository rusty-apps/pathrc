use std::str::FromStr;

pub enum Command {
    ALIAS,
    UNALIAS,
    SET,
    UNSET,
    EXPORT,
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alias" => Ok(Self::ALIAS),
            "unalias" => Ok(Self::UNALIAS),
            "set" => Ok(Self::SET),
            "unset" => Ok(Self::UNSET),
            "export" => Ok(Self::EXPORT),
            _ => Err(CommandIgnore),
        }
    }
}

pub struct CommandError;
