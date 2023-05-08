use crate::actors::game_session::Game;
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use actix::{Addr, Message};
pub(crate) struct BuzzerApp {
    pub list_of_games: Arc<Mutex<HashMap<String, Addr<Game>>>>
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Ticket {
    pub user: String,
    pub lobby: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub(crate) struct AppMessage {
    pub ty: String,
    pub name: Option<String>
}