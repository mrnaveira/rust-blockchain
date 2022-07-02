use crate::{
    model::{Block, Blockchain, Transaction, TransactionPool},
    util::{execution::Runnable, Context},
};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;

struct ApiState {
    blockchain: Blockchain,
    pool: TransactionPool,
}

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
            .route("/blocks", web::post().to(add_block))
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

// Adds a new block to the blockchain
async fn add_block(state: web::Data<ApiState>, block_json: web::Json<Block>) -> HttpResponse {
    let mut block = block_json.into_inner();

    // The hash of the block is mandatory and the blockchain checks if it's correct
    // That's a bit unconvenient for manual use of the API
    // So we ignore the comming hash and recalculate it again before adding to the blockchain
    block.hash = block.calculate_hash();

    let blockchain = &state.blockchain;
    let result = blockchain.add_block(block.clone());

    match result {
        Ok(_) => {
            info!("Received new block {}", block.index);
            HttpResponse::Ok().finish()
        }
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}

// Adds a new transaction to the pool, to be included on the next block
async fn add_transaction(
    state: web::Data<ApiState>,
    transaction_json: web::Json<Transaction>,
) -> impl Responder {
    let transaction = transaction_json.into_inner();
    let pool = &state.pool;
    pool.add_transaction(transaction);

    HttpResponse::Ok()
}
