use crate::{
    model::{Blockchain, Transaction, TransactionPool},
    util::{execution::Runnable, Context},
};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;

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

impl Runnable for Api {
    fn run(&self) -> Result<()> {
        let api_blockchain = self.blockchain.clone();
        let api_pool = self.pool.clone();

        start_server(self.port, api_blockchain, api_pool)
    }
}

impl Api {
    pub fn new(context: &Context) -> Api {
        Api {
            port: context.config.port,
            blockchain: context.blockchain.clone(),
            pool: context.pool.clone(),
        }
    }
}

#[actix_web::main]
async fn start_server(port: u16, blockchain: Blockchain, pool: TransactionPool) -> Result<()> {
    let url = format!("localhost:{}", port);
    // These variables are really "Arc" pointers to a shared memory value
    // So when we clone them, we are only cloning the pointers and not the actual data
    let api_state = web::Data::new(ApiState { blockchain, pool });

    HttpServer::new(move || {
        App::new()
            .app_data(api_state.clone())
            .route("/blocks", web::get().to(get_blocks))
            .route("/transactions", web::post().to(add_transaction))
    })
    .bind(url)
    .unwrap()
    .run()
    .await?;

    Ok(())
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
        amount: transaction_json.amount,
    };

    let pool = &state.pool;
    pool.add_transaction(transaction);

    HttpResponse::Ok()
}
