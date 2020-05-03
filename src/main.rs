mod api;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let blockchain = Blockchain::new();

    let port = 8000;
    api::run(port, blockchain).expect("could not start the API");
}