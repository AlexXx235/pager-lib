use std::error::Error;
use std::fmt;

use chrono::NaiveDateTime;
use diesel::sql_types::Timestamp;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub request_id: u32,
    pub method: Method,
    pub session_token: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    NewPrivateMessage(PrivateMessage),
    NewChatMessage(ChatMessage)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    SignUp {
        login: String,
        password: String
    },
    LogIn {
        login: String,
        password: String
    },
    SendPrivateMessage {
        message: String,
        receiver_login: String
    },
    GetPrivateChatMessages {
        second_user_login: String
    },
    GetAvailableChats
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MethodResult {
    SignUp,
    LogIn {
        session_token: String,
    },
    SendPrivateMessage,
    GetPrivateChatMessages {
        messages: Vec<PrivateMessage>
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateMessage {
    pub text: String,
    pub sender_id: i32,
    pub receiver_id: i32,
    raw_timestamp: i64,
}

impl PrivateMessage {
    pub fn new(text: String, timestamp: NaiveDateTime, sender_id: i32, receiver_id: i32) -> Self {
        PrivateMessage { text, raw_timestamp: timestamp.timestamp(), sender_id, receiver_id }
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.raw_timestamp, 0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub text: String,
    pub sender_id: i32,
    pub chat_id: i32,
    pub raw_timestamp: i64
}

impl ChatMessage {
    pub fn new(text: String, timestamp: NaiveDateTime, sender_id: i32, chat_id: i32) -> Self {
        ChatMessage { text, raw_timestamp: timestamp.timestamp(), sender_id, chat_id }
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.raw_timestamp, 0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestResult {
    pub request_id: u32,
    pub result: Result<MethodResult, RequestError>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestError {
    Auth(AuthError),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::Auth(err) => {
                write!(f, "RequestError: authentication error: {}", err)
            }
        }
    }
}

impl Error for RequestError {}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthError {
    AlreadySignedUp,
    IncorrectCredentials,
    IncorrectSessionToken
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::AlreadySignedUp => {
                write!(f, "already signed up")
            },
            AuthError::IncorrectCredentials => {
                write!(f, "incorrect login or password")
            },
            AuthError::IncorrectSessionToken => {
                write!(f, "incorrect session token")
            }
        }
    }
}