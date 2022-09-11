use crate::{database::Database, util::execution::Runnable};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use spec::{
    types::{Block, Transaction},
    Database as SpecDatabase,
};

pub struct Api {
    port: u16,
    database: Database,
}

impl Runnable for Api {
    fn run(&self) -> Result<()> {
        start_server(self.port, &self.database)
    }
}

impl Api {
    pub fn new(port: u16, database: &Database) -> Api {
        Api {
            port,
            database: database.clone(),
        }
    }
}

#[actix_web::main]
async fn start_server(port: u16, database: &Database) -> Result<()> {
    let url = format!("localhost:{}", port);
    let state = web::Data::new(database.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/blocks", web::get().to(get_blocks))
            .route("/blocks", web::post().to(add_block))
            .route("/transactions", web::get().to(get_transactions))
            .route("/transactions", web::post().to(add_transaction))
    })
    .bind(url)
    .unwrap()
    .run()
    .await?;

    Ok(())
}

// Returns a list of all the blocks in the blockchain
async fn get_blocks(database: web::Data<Database>) -> impl Responder {
    let blocks = database.get_all_blocks();

    HttpResponse::Ok().json(&blocks)
}

// Adds a new block to the blockchain
async fn add_block(database: web::Data<Database>, block_json: web::Json<Block>) -> HttpResponse {
    let block = block_json.into_inner();
    let result = database.append_block(&block);

    match result {
        Ok(_) => {
            info!("Received new block {}", block.index);
            HttpResponse::Ok().finish()
        }
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}

// Returns a list of all the transactions that are not yet included into a block
async fn get_transactions(database: web::Data<Database>) -> impl Responder {
    let transactions = database.get_transactions();
    HttpResponse::Ok().json(&transactions)
}

// Adds a new transaction to the pool, to be included on the next block
async fn add_transaction(
    database: web::Data<Database>,
    transaction_json: web::Json<Transaction>,
) -> impl Responder {
    let transaction = transaction_json.into_inner();
    let result = database.add_transaction(transaction);
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}
