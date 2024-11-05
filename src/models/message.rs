use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: Role,
    pub content: Vec<Content>,
}

impl Message {
    /// Creates a new `Message` instance with the given role and content.
    ///
    /// # Arguments
    ///
    /// * `role` - The role of the message sender.
    /// * `content` - A vector of `Content` items representing the message content.
    ///
    /// # Returns
    ///
    /// A new `Message` instance.
    pub fn new(role: Role, content: Vec<Content>) -> Self {
        Self { role, content }
    }

    /// Creates a new `MessageBuilder` instance.
    ///
    /// # Returns
    ///
    /// A `MessageBuilder` for building a `Message` instance with specified role and content.
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }
}

/// A builder for creating `Message` instances with specified role and content.
pub struct MessageBuilder {
    role: Option<Role>,
    content: Vec<Content>,
}

impl MessageBuilder {
    /// Creates a new `MessageBuilder` instance.
    ///
    /// # Returns
    ///
    /// A `MessageBuilder` instance with empty role and content fields.
    pub fn new() -> Self {
        Self {
            role: None,
            content: vec![],
        }
    }

    /// Sets the role for the `Message`.
    ///
    /// # Arguments
    ///
    /// * `role` - The role of the message sender (e.g., "user" or "assistant").
    ///
    /// # Returns
    ///
    /// The builder instance with the specified role.
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }

    /// Adds content to the `Message`.
    ///
    /// # Arguments
    ///
    /// * `type_field` - The type of the content (e.g., "text").
    /// * `text` - The actual text content.
    ///
    /// # Returns
    ///
    /// The builder instance with the new content added.
    pub fn add_content(mut self, type_field: &str, text: &str) -> Self {
        self.content.push(Content {
            type_field: type_field.to_owned(),
            text: text.to_owned(),
        });
        self
    }

    /// Builds the `Message` instance.
    ///
    /// # Returns
    ///
    /// A `Message` instance with the specified role and content.
    pub fn build(self) -> Result<Message, Error> {
        Ok(Message {
            role: self
                .role
                .ok_or(Error::ConfigError("Role is required".to_owned()))?,
            content: self.content,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}

/// Role enumerates the different roles involved in message exchange (e.g., System, Assistant, User).
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
}
