
use actix::prelude::*;
use actix_web_actors::ws;

use crate::{actors::game_session::*, structures::AppMessage};

#[derive(Message)]
#[rtype(result = "()")]
pub struct KickOut {
    pub code: ws::CloseCode
}

pub(crate) struct EvGame {
    pub name: String,
    pub game_addr: Addr<Game>
}

impl Actor for EvGame {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for EvGame {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Close(_)) => self.game_addr.do_send(Remove {
                name: self.name.clone(),
            }),
            Ok(ws::Message::Text(msg)) => {
                let deser = serde_json::from_str::<AppMessage>(msg.trim()).unwrap();
                match deser.ty.as_str() {
                "register" => {
                    self.game_addr.do_send(Register {
                        name: self.name.clone(),
                        player: ctx.address(),
                    });
                },
                "buzz" => {
                    self.game_addr.do_send(Buzz {
                        name: self.name.clone(),
                    });
                },
                _ => println!("{:?}", deser)
            }
            },
            Err(_) => println!("aaaah"),
            _ => ()
        }
    }
}

impl Handler<AppMessage> for EvGame {
    type Result = ();

    fn handle(&mut self, msg: AppMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap())
    }
}

impl Handler<KickOut> for EvGame {
    type Result = ();

    fn handle(&mut self, msg: KickOut, ctx: &mut Self::Context) -> Self::Result {
        ctx.close(Some(ws::CloseReason { code: msg.code, description: None }))
    }
}