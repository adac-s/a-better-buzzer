use std::collections::HashMap;
use std::sync::Mutex;

use crate::structures::*;
use crate::actors::*;
use actix::Actor;
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Error, error, http::header::ContentType, HttpRequest};
use actix_web_actors::ws;
use tera::Tera;
extern crate tera;
use rand::{thread_rng, distributions::{Alphanumeric, DistString}};


#[get("/")]
async fn index(templates: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let ctx = tera::Context::new();
    let rendered = templates.render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("template error"))?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}

#[get("/gameleader")]
async fn gameleader(
    templates: web::Data<Tera>, 
    game_data: web::Data<BuzzerApp>, 
    session: Session
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let game_id = Alphanumeric.sample_string(&mut thread_rng(), 8);
    ctx.insert("id", &game_id);
    session.insert("game_id", &game_id)?;
    let rendered = templates.render("gameleader.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("template error"))?;
    game_data.list_of_games.lock().map_err(|_| error::ErrorInternalServerError("unable to aquire lock"))?
        .insert(game_id, game_session::Game { 
            buzz_on: Mutex::new(false),
            game_leader: None,
            players: Mutex::new(HashMap::new()),
        }.start());
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}

#[get("/ev_gl")]
async fn ev_gl(
    req: HttpRequest,
    stream: web::Payload,
    session: Session,
    game_data: web::Data<BuzzerApp>,
) -> Result<HttpResponse, Error> {
    let gid = session.get::<String>("game_id")?.unwrap();
    let gaddr = game_data.list_of_games.lock()
        .map_err(|_| error::ErrorInternalServerError("unable to aquire lock"))?.get(&gid).expect("game already init").clone();
    ws::start(ev_gameleader::EvGameleader {
        game_addr: gaddr,
    }, &req, stream)
}

#[get("/game")]
async fn game(templates: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let gid = session.get::<String>("game_id")?.unwrap();
    let name = session.get::<String>("user")?.unwrap();
    ctx.insert("game_id", &gid);
    ctx.insert("user", &name
        .replace("&gt", ">")
        .replace("&lt", "<")
        .replace("&amp", "&")
    );
    let rendered = templates.render("game.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("template error"))?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}

#[get("/ev_g")]
async fn ev_g(
    req: HttpRequest,
    stream: web::Payload,
    session: Session,
    game_data: web::Data<BuzzerApp>,
) -> Result<HttpResponse, Error> {
    //TODO: might crash server
    let gid = session.get::<String>("game_id")?.unwrap();
    let username = session.get::<String>("user")?.unwrap();
    let gaddr = game_data.list_of_games.lock()
        .map_err(|_| error::ErrorInternalServerError("unable to aquire lock"))?
        .get(&gid).expect("game already init").clone();
    ws::start(ev_game::EvGame {
        name: username,
        game_addr: gaddr,
    }, &req, stream)
}

#[post("/admission")]
async fn admission(web::Form(ticket): web::Form<Ticket>, game_data: web::Data<BuzzerApp>, session: Session) -> Result<web::Redirect, Error> {
    let hm = game_data.list_of_games.lock()
        .map_err(|_| error::ErrorInternalServerError("unable to aquire lock"))?;
    
    if hm.contains_key(&ticket.lobby) {
        session.insert("game_id", &ticket.lobby)?;
        session.insert("user", &ticket.user
            .replace("&", "&amp")
            .replace("<", "&lt")
            .replace(">", "&gt")
        )?;
        Ok(web::Redirect::to("/game").see_other())
    }
    else {
        Ok(web::Redirect::to("/").see_other())
    }
}