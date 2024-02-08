use openai::chat::{
    ChatCompletion, ChatCompletionChoice, ChatCompletionFunctionCall,
    ChatCompletionFunctionDefinition, ChatCompletionMessage, ChatCompletionMessageRole,
};
use serde_json::json;

use crate::result::Result;

pub fn system_msg(content: &str) -> ChatCompletionMessage {
    openai::chat::ChatCompletionMessage {
        content: Some(content.to_string()),
        role: ChatCompletionMessageRole::System,
        function_call: None,
        name: None,
    }
}

pub fn user_msg(content: &str) -> ChatCompletionMessage {
    openai::chat::ChatCompletionMessage {
        content: Some(content.to_string()),
        role: ChatCompletionMessageRole::User,
        function_call: None,
        name: None,
    }
}

pub fn assistant_msg(content: &str) -> ChatCompletionMessage {
    openai::chat::ChatCompletionMessage {
        content: Some(content.to_string()),
        role: ChatCompletionMessageRole::Assistant,
        function_call: None,
        name: None,
    }
}

pub fn assistant_function_msg(function_call: ChatCompletionFunctionCall) -> ChatCompletionMessage {
    openai::chat::ChatCompletionMessage {
        content: None,
        role: ChatCompletionMessageRole::Assistant,
        function_call: Some(function_call),
        name: None,
    }
}

pub async fn chat(messages: Vec<ChatCompletionMessage>) -> Result<String> {
    let choice = chat_with_functions(messages, vec![], None).await?;
    let message = choice.message.content.clone().unwrap_or("".to_string());

    Ok(message)
}

pub async fn chat_with_functions(
    messages: Vec<ChatCompletionMessage>,
    functions: Vec<ChatCompletionFunctionDefinition>,
    function_call: Option<&str>,
) -> Result<ChatCompletionChoice> {
    let mut builder = ChatCompletion::builder("gpt-4", messages).functions(functions);

    if let Some(function_call) = function_call {
        builder = builder.function_call(json!({"name": function_call}));
    }

    let result = builder.create().await?;
    let choice = result.choices.first().unwrap().clone();

    Ok(choice)
}
