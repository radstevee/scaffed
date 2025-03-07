#![feature(associated_type_defaults)]

use std::{
    env,
    fmt::Display,
    fs, io,
    path::PathBuf,
    process::{Command, ExitStatus, Stdio},
};

use promkit::preset::{confirm::Confirm, listbox::Listbox, readline::Readline};
use scaffold::{Scaffold, gradle::GradleScaffold};

pub mod scaffold;

/// A project configuration, passed in to scaffolds.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectConfiguration {
    /// The project name.
    pub name: String,

    /// The directory to use for the project. This will be created if it
    /// does not exist already.
    pub directory: PathBuf,
}

/// Prompt a basic string and return the result.
pub fn prompt(title: impl AsRef<str>) -> String {
    let mut prompt = Readline::default()
        .title(title)
        .prompt()
        .expect("Failed reading stdin");

    prompt.run().expect("Failed reading prompt")
}

/// Prompt an input from a vector of available options and return the selected index.
pub fn prompt_options<T: Display, O: IntoIterator<Item = T>>(
    title: impl AsRef<str>,
    options: O,
) -> String {
    let mut prompt = Listbox::new(options)
        .title(title)
        .listbox_lines(5)
        .prompt()
        .expect("Failed reading stdin");

    prompt.run().expect("Failed reading prompt")
}

/// Prompt a simple yes/no question and return the result.
pub fn prompt_bool(title: impl AsRef<str>) -> bool {
    let mut prompt = Confirm::new(&title).prompt().expect("Failed reading stdin");

    let result = prompt.run().expect("Failed reading prompt");

    if result == "y" {
        true
    } else if result == "n" {
        false
    } else {
        result
            .parse::<bool>()
            .unwrap_or_else(|_| prompt_bool(title))
    }
}

/// Spawns a command in the given working directory and returns the exit code.
pub fn command(command: &str, args: Vec<&str>, cwd: Option<PathBuf>, silent: bool) -> ExitStatus {
    let mut command = Command::new(command);
    command.args(args);

    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }

    if silent {
        command.stdout(Stdio::null());
    }

    command
        .spawn()
        .expect("Failed spawning child")
        .wait()
        .expect("Failed waiting for child 😔")
}

/// Gets the project directory and name and returns the configuration object.
pub fn get_project() -> ProjectConfiguration {
    let args = env::args().collect::<Vec<String>>();
    let dir = args.get(1).expect("Usage: scaffed <directory>").clone();
    let directory = PathBuf::from(dir);
    let name = prompt("What is the name of the project?");

    ProjectConfiguration { directory, name }
}

/// Runs scaffed for a given project configuration.
pub fn run(project: ProjectConfiguration) -> io::Result<()> {
    if !fs::exists(&project.directory).unwrap_or(true) {
        fs::create_dir_all(&project.directory)?;
    }

    let config = GradleScaffold::configure(&GradleScaffold, project.clone())?;
    GradleScaffold::scaffold(&GradleScaffold, project, config)
}

/// Calls get_project and runs scaffed.
pub fn get_project_and_run() -> io::Result<()> {
    run(get_project())
}
