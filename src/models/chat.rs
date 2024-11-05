use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::message::Message;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub messages: Vec<Message>,
    #[serde(skip)]
    pub conversation_id: String,
    #[serde(skip)]
    pub stream: bool,
}

impl Chat {
    /// Creates a new `Chat` instance with the given messages.
    ///
    /// # Arguments
    ///
    /// * `messages` - A vector of `Message` objects to initialize the `Chat` instance.
    ///
    /// # Returns
    ///
    /// A new `Chat` instance.
    pub fn new(messages: Vec<Message>, conversation_id: String, stream: bool) -> Self {
        Self {
            messages,
            conversation_id,
            stream,
        }
    }

    /// Creates a new `ChatBuilder` instance.
    ///
    /// # Returns
    ///
    /// A `ChatBuilder` for building a `Chat` instance with messages.
    pub fn builder() -> ChatBuilder {
        ChatBuilder::new()
    }
}

/// A builder for creating `Chat` instances with specified messages.
///
/// This builder allows you to incrementally add `Message` objects to a `Chat`.
pub struct ChatBuilder {
    messages: Vec<Message>,
    conversation_id: Option<String>,
    stream: bool,
}

impl ChatBuilder {
    /// Creates a new `ChatBuilder` instance.
    ///
    /// # Returns
    ///
    /// A `ChatBuilder` instance with an empty messages list.
    pub fn new() -> Self {
        Self {
            messages: vec![],
            conversation_id: None,
            stream: false,
        }
    }

    /// Adds a message to the `Chat` instance.
    ///
    /// # Arguments
    ///
    /// * `role` - The role of the message sender (e.g., "user" or "assistant").
    /// * `content` - A vector of `Content` items representing the message content.
    ///
    /// # Returns
    ///
    /// The builder instance with the new message added.
    pub fn add_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    /// Sets the conversation ID for the `Chat` instance.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The conversation ID for the chat.
    ///
    /// # Returns
    ///
    /// The builder instance with the specified conversation ID.
    pub fn conversation_id(mut self, conversation_id: String) -> Self {
        self.conversation_id = Some(conversation_id);
        self
    }

    /// Sets the stream flag for the `Chat` instance.
    ///
    /// # Arguments
    ///
    /// * `stream` - A boolean flag indicating whether the chat should be streamed.
    ///
    /// # Returns
    ///
    /// The builder instance with the specified stream flag.
    pub fn stream(mut self) -> Self {
        self.stream = true;
        self
    }

    /// Builds the `Chat` instance.
    ///
    /// # Returns
    ///
    /// A `Chat` instance with the specified messages.
    pub fn build(self) -> Result<Chat, Error> {
        Ok(Chat {
            messages: self.messages,
            conversation_id: self
                .conversation_id
                .ok_or(Error::ConfigError("Conversation ID is required".to_owned()))?,
            stream: self.stream,
        })
    }
}

impl Default for ChatBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::message::{Content, Role};

    use super::*;

    #[test]
    fn test_build_chat_with_multiple_messages() {
        let chat = Chat::builder()
            .add_message(
                Message::builder()
                    .role(Role::User)
                    .add_content("text", "Hello")
                    .build()
                    .unwrap(),
            )
            .conversation_id("some-id".to_string())
            .add_message(
                Message::builder()
                    .role(Role::Assistant)
                    .add_content("text", "Hi there!")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        assert_eq!(chat.messages.len(), 2);
        assert_eq!(chat.messages[0].role, Role::User);
        assert_eq!(chat.messages[0].content[0].text, "Hello");
        assert_eq!(chat.messages[1].role, Role::Assistant);
        assert_eq!(chat.messages[1].content[0].text, "Hi there!");
    }

    #[test]
    fn test_chat_builder_missing_conversation_id() {
        let chat_result = Chat::builder()
            .add_message(
                Message::builder()
                    .role(Role::User)
                    .add_content("text", "Hello")
                    .build()
                    .unwrap(),
            )
            .build();

        assert!(chat_result.is_err());
        assert_eq!(
            matches!(chat_result.err().unwrap(), Error::ConfigError(_)),
            true
        );
    }

    #[test]
    fn test_message_builder_with_content() {
        let message = Message::builder()
            .role(Role::User)
            .add_content("text", "How are you?")
            .build()
            .unwrap();

        assert_eq!(message.role, Role::User);
        assert_eq!(message.content.len(), 1);
        assert_eq!(message.content[0].type_field, "text");
        assert_eq!(message.content[0].text, "How are you?");
    }

    #[test]
    fn test_message_builder_without_role() {
        let message_result = Message::builder()
            .add_content("text", "Missing role")
            .build();

        assert!(message_result.is_err());
        assert_eq!(
            matches!(message_result.err().unwrap(), Error::ConfigError(_)),
            true
        );
    }

    #[test]
    fn test_chat_new_function_with_conversation_id() {
        let messages = vec![Message::new(
            Role::User,
            vec![Content {
                type_field: "text".to_string(),
                text: "Hello from new".to_string(),
            }],
        )];
        let chat = Chat::new(messages, "some-id".to_owned(), false);

        assert_eq!(chat.messages.len(), 1);
        assert_eq!(chat.messages[0].role, Role::User);
        assert_eq!(chat.messages[0].content[0].text, "Hello from new");
    }
}
