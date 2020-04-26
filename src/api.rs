use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

use crate::blockchain::Blockchain;

struct AppState {
    blockchain_mutex: Mutex<Blockchain>
}

async fn get_blocks(state: web::Data<AppState>) -> impl Responder {
    let blockchain_mutex = &state.blockchain_mutex;
    let blockchain = blockchain_mutex.lock().unwrap();
    HttpResponse::Ok().json(&blockchain.blocks)
}

#[actix_rt::main]
pub async fn run(port: u16, blockchain: Blockchain) -> std::io::Result<()> {
    let url = format!("localhost:{}", port);

    let my_data = web::Data::new(AppState {
        blockchain_mutex: Mutex::new(blockchain),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(my_data.clone())
            .route("/blocks", web::get().to(get_blocks))
    })
    .bind(url).unwrap()
    .run()
    .await
}