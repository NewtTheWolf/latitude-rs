use serde::{Deserialize, Serialize};

/// Represents the configuration settings
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Options {
    pub version_id: Option<String>,
    pub project_id: Option<u64>,
}

impl Options {
    /// Creates a new `Options` instance with the specified version ID and project ID.
    ///
    /// # Arguments
    ///
    /// * `version_id` - The version ID to be set in the configuration.
    /// * `project_id` - The project ID to be set in the configuration.
    pub fn new(version_id: Option<String>, project_id: Option<u64>) -> Self {
        Self {
            version_id,
            project_id,
        }
    }

    pub fn builder() -> OptionsBuilder {
        OptionsBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct OptionsBuilder {
    pub version_id: Option<String>,
    pub project_id: Option<u64>,
}

impl OptionsBuilder {
    /// Sets the version ID for the `Options`.
    ///
    /// # Arguments
    ///
    /// * `version_id` - The version ID to be set in the configuration.
    pub fn version_id(mut self, version_id: String) -> Self {
        self.version_id = Some(version_id);
        self
    }

    /// Sets the project ID for the `Options`.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID to be set in the configuration.
    pub fn project_id(mut self, project_id: u64) -> Self {
        self.project_id = Some(project_id);
        self
    }

    /// Builds the `Options` instance with the specified version ID and project ID.
    ///
    /// # Returns
    ///
    /// An `Options` instance.
    pub fn build(self) -> Options {
        Options {
            version_id: self.version_id,
            project_id: self.project_id,
        }
    }
}
