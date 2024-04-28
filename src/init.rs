use crate::InitArgs;
use clap::ValueEnum;
use envmnt;
use rust_embed::RustEmbed;
use std::{str};

#[derive(RustEmbed)]
#[folder = "init/"]
struct Asset;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Shell {
    /// Bash and Bash compatible Bourne shells
    Bash,
    /// Fish shell (experimental)
    Fish,
    /// Z Shell
    Zsh,
    /// Default selected from environment
    Env,
}

pub fn get_shell_name(shell: Shell) -> String {
    match shell {
        Shell::Bash => String::from("bash"),
        Shell::Fish => String::from("fish"),
        Shell::Zsh => String::from("zsh"),
        Shell::Env => String::from("env"),
    }
}

pub fn get_file_name(shell_name: String) -> String {
    let shared_bourne_behaviour = envmnt::get_or("PATHRC_SHARED_CONFIG", "TRUE");
    match shell_name.as_str() {
        "bash" => match shared_bourne_behaviour.as_str() {
            "TRUE" => ".pathrc.sh".to_string(),
            _ => ".pathrc.bash".to_string(),
        },
        "zsh" => match shared_bourne_behaviour.as_str() {
            "TRUE" => ".pathrc.sh".to_string(),
            _ => ".pathrc.zsh".to_string(),
        },
        "fish" => ".pathrc.fish".to_string(),
        _ => panic!("ISSUE: Invalid shell name '{}'.", shell_name),
    }
}

pub fn print_init_script(shell: String) {
    let base: &str = "pathrc.";
    let filename = format!("{base}{shell}");
    let script_name: &str = &*filename;
    let script = Asset::get(script_name).unwrap();
    let string_data = str::from_utf8(script.data.as_ref());
    print!("{}", string_data.unwrap());
}

pub fn get_shell_from_env() -> String {
    let shell_env = envmnt::get_or("SHELL", "NONE");
    let (_, shell_name) = shell_env.rsplit_once('/').unwrap();
    match shell_name {
        "bash" => "bash".to_string(),
        "fish" => "fish".to_string(),
        "zsh" => "zsh".to_string(),
        _ => panic!(
            "ISSUE: Unable to determine shell from environment \
            or shell is not currently supported '{}'. If this is a \
            supported shell then add as argument",
            shell_env
        ),
    }
}

pub fn handle_init(init_args: &InitArgs) {
    let shell = get_shell_name(init_args.shell);
    if init_args.enable_shared_config == Option::from(false) {
        println!("export PATHRC_SHARED_CONFIG=FALSE");
    } else {
        println!("export PATHRC_SHARED_CONFIG=TRUE");
    }
    match shell.as_str() {
        "env" => {
            let env_shell: String = get_shell_from_env();
            println!("export PATHRC_SHELL={:?}", env_shell);
            print_init_script(env_shell);
        }
        _ => {
            println!("export PATHRC_SHELL={:?}", shell);
            print_init_script(shell);
        }
    }
}

