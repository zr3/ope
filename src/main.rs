use anyhow::{Context, Result};
use clap::{arg, command, Command};
use directories::ProjectDirs;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

// log quick snappy notes to keep track of side thoughts
struct Config {
    hack_path: PathBuf,
    todo_path: PathBuf,
}

fn main() -> Result<()> {
    // instantiate config
    let proj_dirs = ProjectDirs::from("codes", "gnaw", "ope").expect("no project dir exists");
    std::fs::create_dir_all(proj_dirs.data_dir())?;
    let hack_file = proj_dirs.data_dir().join("hack.log");
    let todo_file = proj_dirs.data_dir().join("todo.log");
    let config = Config {
        hack_path: hack_file,
        todo_path: todo_file,
    };

    // set up logging
    let applog_file = proj_dirs.data_dir().join("ope.log");
    let log_config = ConfigBuilder::new().set_time_format_rfc3339().build();
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(applog_file)
        .with_context(|| format!("could not open file `{}`", "ope.log"))?;
    let _ = WriteLogger::init(LevelFilter::Info, log_config, log_file);

    // parse CLI arguments
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("i")
                .about("WHAT DID YOU DO??")
                .arg(arg!([MISTAKE])),
        )
        .subcommand(
            Command::new("will")
                .about("what will you do?")
                .arg(arg!([HOPESANDDREAMS])),
        )
        .subcommand(Command::new("look").about("see your lists"))
        .subcommand(Command::new("didit").about("mark a TODO as DONE"))
        .subcommand(Command::new("fixed").about("mark a HACK as DONE"))
        .get_matches();

    match matches.subcommand() {
        Some(("i", sub_matches)) => run_hack(
            config,
            Note {
                value: sub_matches
                    .get_one::<String>("MISTAKE")
                    .expect("need a mistake")
                    .to_string(),
            },
        ),
        Some(("will", sub_matches)) => run_todo(
            config,
            Note {
                value: sub_matches
                    .get_one::<String>("HOPESANDDREAMS")
                    .expect("need a todo")
                    .to_string(),
            },
        ),
        Some(("look", _sub_matches)) => run_look(config),
        Some(("didit", _sub_matches)) => run_look(config),
        Some(("fixed", _sub_matches)) => run_look(config),
        _ => unreachable!("could not find a valid subcommand"),
    }
}

struct Note {
    value: String,
}

fn run_hack(config: Config, note: Note) -> Result<()> {
    write_note(config.hack_path.as_path(), String::from("HACK"), note)
}

fn run_todo(config: Config, note: Note) -> Result<()> {
    write_note(config.todo_path.as_path(), String::from("TODO"), note)
}

fn write_note(path: &std::path::Path, note_type: String, note: Note) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("could not open file `{}`", path.display()))?;
    let timestamp = OffsetDateTime::now_utc().format(&Rfc3339)?;
    writeln!(&mut file, "{}: [{}] {}", timestamp, note_type, note.value)?;
    Ok(())
}

fn run_look(config: Config) -> Result<()> {
    let hacks: String =
        String::from_utf8_lossy(&std::fs::read(config.hack_path.as_path())?).parse()?;
    println!("FIX THIS:");
    println!("{}", hacks);
    let todos: String =
        String::from_utf8_lossy(&std::fs::read(config.todo_path.as_path())?).parse()?;
    println!("DO THIS:");
    println!("{}", todos);
    Ok(())
}
