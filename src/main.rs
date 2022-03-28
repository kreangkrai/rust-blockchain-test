mod blocks;

use crate::blocks::blockchain::Blockchain;

fn main(){
    let mut blockchain = Blockchain::genesis();
    let difficulty:usize = 2;
    //println!("{:#?}",blockchain);

    let previous_block = blockchain.get_previous_block();
   // let previous_nonce = previous_block.nonce;
    let nonce = blockchain.proof_of_work(difficulty);
    let previous_hash = blockchain.hash(previous_block);
    blockchain.create_block(nonce, previous_hash, 1000);

    //println!("{:#?}",blockchain);

    let previous_block = blockchain.get_previous_block();
   // let previous_nonce = previous_block.nonce;
    let nonce = blockchain.proof_of_work(difficulty);
    let previous_hash = blockchain.hash(previous_block);
    blockchain.create_block(nonce, previous_hash, 3000);

    println!("{:#?}",blockchain);

    let status = blockchain.is_chain_valid(difficulty);
    println!("{:?}",status);
}