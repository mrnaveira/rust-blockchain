use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::blockchain::{Blockchain, Transaction};
use super::transaction_pool::{TransactionPool};

struct ApiState {
    blockchain: Blockchain,
    transaction_pool: TransactionPool
}

// Returns a list of all the blocks in the blockchain
async fn get_blocks(state: web::Data<ApiState>) -> impl Responder {
    let blockchain = &state.blockchain;
    let blocks = blockchain.get_all_blocks();

    HttpResponse::Ok().json(&blocks)
}

// Adds a new transaction to the pool, to be included on the next block
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
pub async fn run(port: u16, blockchain: Blockchain, transaction_pool: TransactionPool) -> std::io::Result<()> {
    let url = format!("localhost:{}", port);
    let api_state = web::Data::new(ApiState {
        blockchain: blockchain,
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