use std::{fmt::Display, fs, io, process::exit};

use crate::{ProjectConfiguration, command, prompt, prompt_bool, prompt_options};

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

pub(crate) trait StringExt {
    fn push_linebreak(&mut self);
}

impl StringExt for String {
    fn push_linebreak(&mut self) {
        self.push('\n');
    }
}

impl GradleScaffold {
    // TODO: multi-module
    pub(crate) fn setup_buildscripts(
        &self,
        project: &ProjectConfiguration,
        config: &GradleConfiguration,
    ) -> io::Result<()> {
        let file = project.directory.join("build.gradle.kts");

        let mut content = String::new();

        if config.language != GradleLanguageType::Unspecified {
            content.push_str("plugins {");
            content.push_linebreak();
            content.push_str("    ");

            content.push_str(match config.language {
                GradleLanguageType::Java => "java",
                // TODO: kotlin version support
                GradleLanguageType::Kotlin => r#"kotlin("jvm") version "2.1.10""#,
                _ => unreachable!(),
            });

            content.push_linebreak();
            content.push('}');
        }

        fs::write(file, content).expect("Failed writing buildscript");
        Ok(())
    }

    pub(crate) fn setup_buildsettings(
        &self,
        project: &ProjectConfiguration,
    ) -> io::Result<()> {
        let file = project.directory.join("settings.gradle.kts");

        let mut content = String::new();
        content.push_str(&format!("rootProject.name = \"{}\"", project.name));

        fs::write(file, content).expect("Failed writing buildsettings");

        Ok(())
    }

    pub(crate) fn run_wrapper(
        &self,
        project: &ProjectConfiguration,
        config: &GradleConfiguration,
    ) -> io::Result<()> {
        command(
            "gradle",
            vec!["wrapper", "--gradle-version", config.gradle_version.as_str()],
            Some(project.clone().directory),
            false
        );
        Ok(())
    }

    pub(crate) fn setup_project(
        &self,
        project: ProjectConfiguration,
        config: GradleConfiguration,
    ) -> io::Result<()> {
        self.setup_buildsettings(&project)?;
        self.setup_buildscripts(&project, &config)?;
        self.run_wrapper(&project, &config)?;
        Ok(())
    }

    pub(crate) fn check_for_system_install(&self) -> bool {
        let status = command("bash", vec!["-c", "command -v gradle"], None, true);

        status.success()
    }
}

impl Scaffold for GradleScaffold {
    type Config = GradleConfiguration;

    fn configure(&self, _project: ProjectConfiguration) -> io::Result<Self::Config> {
        if !self.check_for_system_install() {
            eprintln!("No Gradle system install found!");
            exit(1);
        }

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

    fn scaffold(&self, project: ProjectConfiguration, config: Self::Config) -> io::Result<()> {
        self.setup_project(project, config)
    }
}
