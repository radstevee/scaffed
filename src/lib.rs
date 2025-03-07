#![feature(associated_type_defaults)]

use std::{fmt::Display, path::PathBuf};

use promkit::preset::{confirm::Confirm, listbox::Listbox, readline::Readline};

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
pub fn prompt_options<T: Display, O: IntoIterator<Item = T>>(title: impl AsRef<str>, options: O) -> String {
    let mut prompt = Listbox::new(options)
        .title(title)
        .listbox_lines(5)
        .prompt()
        .expect("Failed reading stdin");

    prompt.run().expect("Failed reading prompt")
}

/// Prompt a simple yes/no question and return the result.
pub fn prompt_bool(title: impl AsRef<str>) -> bool {
    let mut prompt = Confirm::new(&title)
        .prompt()
        .expect("Failed reading stdin");

    let result = prompt.run().expect("Failed reading prompt");
    let result_parsed = result.parse::<bool>()
        .unwrap_or_else(|_| prompt_bool(title));

    result_parsed
}

