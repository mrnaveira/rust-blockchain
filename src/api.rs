use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};

use crate::blockchain::{Blockchain, Transaction};

struct ApiState {
    blockchain_arc: Arc<Mutex<Blockchain>>,
    transaction_pool_arc: Arc<Mutex<Vec<Transaction>>>
}

async fn get_blocks(state: web::Data<ApiState>) -> impl Responder {
    let blockchain_arc = &state.blockchain_arc;
    let blockchain = blockchain_arc.lock().unwrap();
    HttpResponse::Ok().json(&blockchain.blocks)
}

async fn add_transaction(state: web::Data<ApiState>, transaction_json: web::Json<Transaction>) -> impl Responder {
    let transaction = Transaction {
        sender: transaction_json.sender.clone(),
        recipient: transaction_json.recipient.clone(),
        amount: transaction_json.amount.clone()
    };

    let transaction_pool_arc = &state.transaction_pool_arc;
    let mut transaction_pool = transaction_pool_arc.lock().unwrap();
    transaction_pool.push(transaction);

    HttpResponse::Ok()
}

#[actix_rt::main]
pub async fn run(port: u16, blockchain_arc: Arc<Mutex<Blockchain>>, transaction_pool_arc: Arc<Mutex<Vec<Transaction>>>) -> std::io::Result<()> {
    let url = format!("localhost:{}", port);
    let api_state = web::Data::new(ApiState {
        blockchain_arc: blockchain_arc,
        transaction_pool_arc: transaction_pool_arc
    });

    HttpServer::new(move || {
        App::new()
            .app_data(api_state.clone())
            .route("/blocks", web::get().to(get_blocks))
            .route("/transactions", web::post().to(add_transaction))
    })
    .bind(url).unwrap()
    .run()
    .await
}