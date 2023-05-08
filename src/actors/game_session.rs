
use std::{collections::HashMap, sync::Mutex};

use crate::actors::*;
use crate::structures::*;


use actix_web_actors::ws;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct Initialize {
    pub gl_addr: Addr<ev_gameleader::EvGameleader>
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct ToggleBuzzers;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct Buzz {
    pub name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct Register {
    pub name: String,
    pub player: Addr<ev_game::EvGame>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct Remove {
    pub name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct RemoveAll {
    pub code: ws::CloseCode
}

#[derive(Debug)]
pub(crate) struct Game {
    pub buzz_on: Mutex<bool>,
    pub game_leader: Option<Mutex<Addr<ev_gameleader::EvGameleader>>>,
    pub players: Mutex<HashMap<String, (bool, Addr<ev_game::EvGame>)>>,
}

impl Game {
    fn broadcast_all(&mut self, item: AppMessage) {
        if let Some(gl) = &self.game_leader {
            let gl_addr = gl.lock().unwrap();
            gl_addr.do_send(item.clone());
            self.players.lock().unwrap().iter()
                .for_each(|(_, (_, addr))| addr.do_send(item.clone()));
        }
    }
}

impl Actor for Game {
    type Context = Context<Self>;
}

impl Handler<Initialize> for Game {
    type Result = ();

    fn handle(&mut self, msg: Initialize, _ctx: &mut Self::Context) -> Self::Result {
        self.game_leader = Some(Mutex::new(msg.gl_addr));
    }

}
impl Handler<Register> for Game {
    type Result = ();

    fn handle(&mut self, msg: Register, _ctx: &mut Self::Context) -> Self::Result {
        if self.players.lock().unwrap().contains_key(&msg.name) {
            //TODO: send message to sender about duplicate names
            return;
        }
        if self.game_leader.is_some() {
            self.players.lock().unwrap().keys()
                .for_each(|k| msg.player.do_send(AppMessage {
                    ty: "register_recieve".to_string(), name: Some(k.clone())
                }));
            self.players.lock().unwrap().insert(msg.name.clone(), (false, msg.player));
            self.broadcast_all(AppMessage { ty: "register_recieve".to_string(), name: Some(msg.name) })
        }
    }

}

impl Handler<Remove> for Game {
    type Result = ();

    fn handle(&mut self, msg: Remove, _ctx: &mut Self::Context) -> Self::Result {
        if self.game_leader.is_some() {
            self.players.lock().unwrap().remove(&msg.name);
            self.broadcast_all(AppMessage { ty: "remove_recieve".to_string(), name: Some(msg.name) })
        }
        
    }
}

impl Handler<RemoveAll> for Game {
    type Result = ();

    fn handle(&mut self, msg: RemoveAll, _ctx: &mut Self::Context) -> Self::Result {
        self.players.lock().unwrap().values()
            .for_each(|player| player.1.do_send(ev_game::KickOut {
                code: msg.code,
            })
        );
    }
}

impl Handler<ToggleBuzzers> for Game {
    type Result = ();

    fn handle(&mut self, _msg: ToggleBuzzers, _ctx: &mut Self::Context) -> Self::Result {
        *self.buzz_on.get_mut().unwrap() ^= true;
        for (_, (b, _)) in self.players.get_mut().unwrap().iter_mut() {
            *b = false;
        }
    }

}
impl Handler<Buzz> for Game {
    type Result = ();

    fn handle(&mut self, msg: Buzz, _ctx: &mut Self::Context) -> Self::Result {
        //TODO: prevent multiple buzzes per instance of buzz_on being true
        if self.game_leader.is_some() {
            if *self.buzz_on.lock().unwrap() {
                let player = self.players.get_mut().unwrap().get_mut(&msg.name);
                if let Some((b, _)) = player {
                    if !*b {
                        *b = true;
                        self.broadcast_all(AppMessage { ty: "buzz_recieve".to_string(), name: Some(msg.name.clone()) });
                    }
                }
            }
        }
    }

}