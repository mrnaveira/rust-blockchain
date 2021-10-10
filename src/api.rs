use super::transaction_pool::TransactionPool;
use crate::blockchain::{Blockchain, Transaction};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

struct ApiState {
    blockchain: Blockchain,
    pool: TransactionPool,
}

#[derive(Debug, Clone)]
pub struct Api {
    port: u16,
    blockchain: Blockchain,
    pool: TransactionPool,
}

impl Api {
    pub fn new(port: u16, blockchain: &Blockchain, pool: &TransactionPool) -> Api {
        let api = Api {
            port: port,
            blockchain: blockchain.clone(),
            pool: pool.clone(),
        };

        return api;
    }

    pub fn run(&self) -> std::io::Result<()> {
        let api_blockchain = self.blockchain.clone();
        let api_pool = self.pool.clone();
        return start_server(self.port, api_blockchain, api_pool);
    }
}

#[actix_rt::main]
async fn start_server(
    port: u16,
    blockchain: Blockchain,
    pool: TransactionPool,
) -> std::io::Result<()> {
    let url = format!("localhost:{}", port);
    // These variables are really "Arc" pointers to a shared memory value
    // So when we clone them, we are only cloning the pointers and not the actual data
    let api_state = web::Data::new(ApiState {
        blockchain: blockchain,
        pool: pool,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(api_state.clone())
            .route("/blocks", web::get().to(get_blocks))
            .route("/transactions", web::post().to(add_transaction))
    })
    .bind(url)
    .unwrap()
    .run()
    .await
}

// Returns a list of all the blocks in the blockchain
async fn get_blocks(state: web::Data<ApiState>) -> impl Responder {
    let blockchain = &state.blockchain;
    let blocks = blockchain.get_all_blocks();

    HttpResponse::Ok().json(&blocks)
}

// Adds a new transaction to the pool, to be included on the next block
async fn add_transaction(
    state: web::Data<ApiState>,
    transaction_json: web::Json<Transaction>,
) -> impl Responder {
    let transaction = Transaction {
        sender: transaction_json.sender.clone(),
        recipient: transaction_json.recipient.clone(),
        amount: transaction_json.amount.clone(),
    };

    let pool = &state.pool;
    pool.add_transaction(transaction);

    HttpResponse::Ok()
}
