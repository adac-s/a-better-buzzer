mod routes;
mod structures;
mod actors;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{web, App, middleware, HttpServer};
use cookie::Key;
use crate::structures::BuzzerApp;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let game_data = web::Data::new(BuzzerApp{list_of_games: Arc::new(Mutex::new(HashMap::new()))});

    HttpServer::new(move || {
        use tera::Tera;
        use crate::routes::*;

        let tera = Tera::new("templates/**/*.html").unwrap();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            .app_data(game_data.clone())
            .app_data(web::Data::new(tera))
            .service((index, gameleader, ev_gl, game, ev_g, admission))
            .service(actix_files::Files::new("/static", "./static/").show_files_listing().use_last_modified(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}