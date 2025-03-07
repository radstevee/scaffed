use std::io;
use crate::ProjectConfiguration;

pub mod gradle;

/// An empty scaffold configuration.
pub struct EmptyConfig;

/// Something that can scaffold a project.
pub trait Scaffold {
    type Config: Sized = EmptyConfig;

    /// Scaffold the given project.
    fn scaffold(&mut self, project: ProjectConfiguration, config: Self::Config) -> io::Result<()>;

    /// Get the configuration of this scaffold for the given project.
    /// This is usually done through prompting.
    fn configure(&mut self, project: ProjectConfiguration) -> io::Result<Self::Config>;
}

