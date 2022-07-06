use crate::{node::Node, util::execution::Runnable};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use spec::{Block, Transaction};

pub struct Api {
    port: u16,
    node: Node,
}

impl Runnable for Api {
    fn run(&self) -> Result<()> {
        start_server(self.port, &self.node)
    }
}

impl Api {
    pub fn new(port: u16, node: &Node) -> Api {
        Api {
            port,
            node: node.clone(),
        }
    }
}

#[actix_web::main]
async fn start_server(port: u16, node: &Node) -> Result<()> {
    let url = format!("localhost:{}", port);
    let state = web::Data::new(node.clone());

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
async fn get_blocks(node: web::Data<Node>) -> impl Responder {
    let blocks = node.get_all_blocks();

    HttpResponse::Ok().json(&blocks)
}

// Adds a new block to the blockchain
async fn add_block(node: web::Data<Node>, block_json: web::Json<Block>) -> HttpResponse {
    let block = block_json.into_inner();
    let result = node.add_block(block.clone());

    match result {
        Ok(_) => {
            info!("Received new block {}", block.index);
            HttpResponse::Ok().finish()
        }
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}

// Returns a list of all the transactions that are not yet included into a block
async fn get_transactions(node: web::Data<Node>) -> impl Responder {
    let transactions = node.get_transactions();
    HttpResponse::Ok().json(&transactions)
}

// Adds a new transaction to the pool, to be included on the next block
async fn add_transaction(
    node: web::Data<Node>,
    transaction_json: web::Json<Transaction>,
) -> impl Responder {
    let transaction = transaction_json.into_inner();
    node.add_transaction(transaction);

    HttpResponse::Ok()
}
