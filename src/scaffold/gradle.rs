use std::{fmt::Display, io};

use crate::{ProjectConfiguration, prompt, prompt_bool, prompt_options};

use super::Scaffold;

/// A type of language to use for Gradle.
/// Currently only supports Java and Kotlin.
#[derive(Debug, Clone, PartialEq)]
pub enum GradleLanguageType {
    Kotlin,
    Java,
    Unspecified,
}

impl Display for GradleLanguageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl GradleLanguageType {
    /// Parses a language value from a string input.
    pub fn parse(input: String) -> GradleLanguageType {
        match input.to_lowercase().as_str() {
            "kotlin" | "kt" => Self::Kotlin,
            "java" | "j" => Self::Java,
            _ => Self::Unspecified,
        }
    }
}

/// Scaffold configuration for a Gradle scaffold.
#[derive(Debug, Clone, PartialEq)]
pub struct GradleConfiguration {
    /// The Gradle version.
    pub gradle_version: String,
    /// The module group.
    pub group: String,
    /// The module version.
    pub version: String,
    /// The language to scaffold for, or None.
    pub language: GradleLanguageType,

    /// Whether this is a multi-module project.
    pub multi: bool,
    /// The subprojects, if `multi` is true, otherwise None.
    pub subprojects: Option<Vec<String>>,
}

/// A scaffold for Gradle projects.
pub struct GradleScaffold;

impl Scaffold for GradleScaffold {
    type Config = GradleConfiguration;

    fn configure(&mut self, _project: ProjectConfiguration) -> io::Result<Self::Config> {
        let gradle_version = prompt("What Gradle version would you like to use?");
        let group = prompt("What group ID would you like to use?");
        let version = prompt("What version would you like your project to use?");
        let language = GradleLanguageType::parse(prompt_options(
            "What language would you like to use?",
            vec![
                GradleLanguageType::Java,
                GradleLanguageType::Kotlin,
                GradleLanguageType::Unspecified,
            ],
        ));
        let multi = prompt_bool("Would you like to use multiple modules?");
        let subprojects = if multi {
            Some(
                prompt("What are the names of the subprojects? (Seperated with a comma)")
                    .split(",")
                    .map(&str::to_string)
                    .collect::<Vec<String>>(),
            )
        } else {
            None
        };

        Ok(GradleConfiguration {
            gradle_version,
            group,
            version,
            language,
            multi,
            subprojects,
        })
    }

    fn scaffold(&mut self, project: ProjectConfiguration, config: Self::Config) -> io::Result<()> {
        unimplemented!()
    }
}
