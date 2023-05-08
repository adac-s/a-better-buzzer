
use actix::prelude::*;
use actix_web_actors::ws;

use crate::{actors::game_session::*, structures::AppMessage};


pub(crate) struct EvGameleader {
    pub game_addr: Addr<Game>
}

impl Actor for EvGameleader {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for EvGameleader {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Close(_)) => {
                self.game_addr.do_send(RemoveAll {
                    code: ws::CloseCode::Away,
                });
            },
            Ok(ws::Message::Text(msg)) => {
                let deser = serde_json::from_str::<AppMessage>(msg.trim()).unwrap();
                match deser.ty.as_str() {
                "initialize" => {
                    self.game_addr.do_send(Initialize {
                        gl_addr: ctx.address(),
                    });
                },
                "toggle_buzzer" => {
                    self.game_addr.do_send( ToggleBuzzers {});
                },
                _ => println!("{:?}", deser)
            }
            },
            Err(_) => println!("aaaah"),
            _ => ()
        }
    }
}

impl Handler<AppMessage> for EvGameleader {
    type Result = ();

    fn handle(&mut self, msg: AppMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap())
    }
}