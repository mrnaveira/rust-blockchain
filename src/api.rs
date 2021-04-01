use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::blockchain::{SharedBlockchain, Transaction};
use super::transaction_pool::{TransactionPool};

struct ApiState {
    shared_blockchain: SharedBlockchain,
    transaction_pool: TransactionPool
}

async fn get_blocks(state: web::Data<ApiState>) -> impl Responder {
    let shared_blockchain = &state.shared_blockchain;
    let blockchain = shared_blockchain.lock().unwrap();
    HttpResponse::Ok().json(&blockchain.blocks)
}

async fn add_transaction(state: web::Data<ApiState>, transaction_json: web::Json<Transaction>) -> impl Responder {
    let transaction = Transaction {
        sender: transaction_json.sender.clone(),
        recipient: transaction_json.recipient.clone(),
        amount: transaction_json.amount.clone()
    };

    let transaction_pool = &state.transaction_pool;
    transaction_pool.add_transaction(transaction);

    HttpResponse::Ok()
}

#[actix_rt::main]
pub async fn run(port: u16, shared_blockchain: SharedBlockchain, transaction_pool: TransactionPool) -> std::io::Result<()> {
    let url = format!("localhost:{}", port);
    let api_state = web::Data::new(ApiState {
        shared_blockchain: shared_blockchain,
        transaction_pool: transaction_pool
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