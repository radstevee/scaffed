use std::{
    env::temp_dir,
    fmt::Display,
    fs::{self, File},
    io::{self, Cursor},
};

use zip::ZipArchive;

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

const DOWNLOAD_URL_BASE: &str = "https://services.gradle.org/distributions";

impl GradleScaffold {
    fn download_wrapper(
        &self,
        config: GradleConfiguration,
    ) -> io::Result<ZipArchive<Cursor<Vec<u8>>>> {
        let url = format!(
            "{DOWNLOAD_URL_BASE}/gradle-{}-bin.zip",
            config.gradle_version
        );
        let temp_file_path =
            temp_dir().join(format!("gradle-wrapper-{}.zip", config.gradle_version));
        let mut temp_file =
            File::create(&temp_file_path).expect("Failed creating Gradle wrapper temp file");

        let response = reqwest::blocking::get(url).expect("Failed downloading Gradle wrapper");
        let mut cursor = Cursor::new(
            response
                .bytes()
                .expect("Failed extracting bytes from Gradle response"),
        );
        io::copy(&mut cursor, &mut temp_file).expect("Failed writing Gradle zip to temp file");

        let zip_data = fs::read(&temp_file_path).expect("Failed reading Gradle zip");
        let cursor = Cursor::new(zip_data);

        let zip = ZipArchive::new(cursor).expect("Failed reading Gradle zip");
        Ok(zip)
    }

    pub fn setup_wrapper(
        &self,
        project: ProjectConfiguration,
        config: GradleConfiguration,
    ) -> io::Result<()> {
        let zip = self.download_wrapper(config)?;
        

        Ok(())
    }
}

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
        self.setup_wrapper(project, config)
            .expect("Failed downloading Gradle wrapper");
        unimplemented!()
    }
}
