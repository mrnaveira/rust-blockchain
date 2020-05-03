use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

use crate::blockchain::{Blockchain, Transaction};

struct AppState {
    blockchain_mutex: Mutex<Blockchain>
}

async fn get_blocks(state: web::Data<AppState>) -> impl Responder {
    let blockchain_mutex = &state.blockchain_mutex;
    let blockchain = blockchain_mutex.lock().unwrap();
    HttpResponse::Ok().json(&blockchain.blocks)
}

async fn add_transaction(state: web::Data<AppState>, transaction_json: web::Json<Transaction>) -> impl Responder {
    let transaction = Transaction {
        sender: transaction_json.sender.clone(),
        recipient: transaction_json.recipient.clone(),
        amount: transaction_json.amount.clone()
    };

    let blockchain_mutex = &state.blockchain_mutex;
    let mut blockchain = blockchain_mutex.lock().unwrap();
    blockchain.add_transaction(transaction.clone());

    HttpResponse::Ok()
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
            .route("/transactions", web::post().to(add_transaction))
    })
    .bind(url).unwrap()
    .run()
    .await
}