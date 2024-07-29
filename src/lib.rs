use anyhow::Result;
use teloxide::{Bot, RequestError};
use teloxide::prelude::Requester;
use teloxide::payloads::SendMessageSetters;
#[derive(Debug, thiserror::Error)]
pub enum TgChannelError {
    // channel id must start with '-'
    #[error("Channel id must start with '-' : {0}")]
    ChannelIdInvalid(String),
    #[error("Telegram bot token is invalid : {0}")]
    TelegramBotTokenInvalid(String),
    #[error("Reqwest error: cant send message to channel :  {0}")]
    Reqwest(#[from] RequestError),
}


pub async fn send_message_to_channel(tg_bot_token: &str, tg_channel_id: &str,msg_text:&str) -> Result<(), TgChannelError> {
    // check channel id need start with -
    if !tg_channel_id.starts_with("-") {
        return Err(TgChannelError::ChannelIdInvalid(tg_channel_id.to_owned()).into());
    }

    // check tg_bot_token is contains : 
    if !tg_bot_token.contains(":") {
        return Err(TgChannelError::TelegramBotTokenInvalid(tg_bot_token.to_owned()).into());
    }
    let bot = Bot::new(tg_bot_token);
    // enable markdown
    let chat_id = tg_channel_id.to_owned();
    bot.send_message(chat_id, msg_text).parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await.map_err(|e| TgChannelError::Reqwest(e))?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use std::future::IntoFuture;

    use super::*;
    #[test]
    pub fn test_send_message_to_channel() {
        let tg_bot_token = "693xxxxx:AAHxxxx";
        let tg_channel_id = "-10xxxxxx";
        let msg_text = "Hello, this is a test message from Rust";
        let _res = send_message_to_channel(&tg_bot_token, &tg_channel_id, msg_text).into_future();
    }
}
