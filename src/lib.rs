use std::error::Error;
use std::fmt;

use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub method: Method,
    pub session_token: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Responce {
    pub result: bool,
    pub params: Value
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub params: Value
}

#[derive(Serialize, Deserialize)]
pub enum EventType {
    NewPrivateMessage,
    NewCharMessage
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
    SendChatMessage {

    },
    GetPrivateChatMessages {
        second_user_login: String
    },
    GetAvailableChats {
        
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestResult {
    Ok(Value),
    Err(RequestError),
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