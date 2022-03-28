use serde::{Serialize};
use sha2::{Digest,Sha256};
use std::fmt::Write;
use chrono::prelude::*;

#[derive(Debug,Clone,Serialize)]
pub struct Block{
    pub index:usize,
    pub timestamp:i64,
    pub nonce:u32,
    pub amonut: u32,
    pub previous_hash:String,
}
#[derive(Debug,Clone,Serialize)]
pub struct Blockchain{
    pub chain:Vec<Block>,
}

impl Blockchain {
    pub fn genesis()->Self{
        let block = Block{
            index: 1,
            timestamp: Utc::now().timestamp_millis(),
            nonce: 1,
            amonut: 0, 
            previous_hash: String::from_utf8(vec![48;64]).unwrap(),
        };
        let chains = vec![block];
        Blockchain { chain: chains}
    }
    pub fn create_block(&mut self,nonce:u32,previous_hash:String,amount:u32){
        let block = Block { index: self.chain.len()+1 ,
                            timestamp: Utc::now().timestamp_millis(),
                            nonce: nonce,
                            amonut: amount, 
                            previous_hash: previous_hash
                          };
        self.chain.push(block);           
    }
    pub fn get_previous_block(&self)->&Block{
        let last: &Block = self.chain.last().unwrap();
        last
    }
    pub fn hash<T: serde::Serialize>(&self,item:T)->String{
        let json = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();
        Blockchain::hex_to_string(vec_res.as_slice())
    }
    pub fn hex_to_string(vec_res:&[u8])->String{
        let mut s = String::new();
        for v in vec_res{
            write!(&mut s,"{:?}",v).expect("unable to write");
        }
        s
    }
    pub fn proof_of_work(&self,difficulty:usize)->u32{
        let mut new_nonce:u32 = 1;
        loop{
            let hash = self.hash(new_nonce);
            let slice = &hash[..difficulty];
            match slice.parse::<u32>(){
                Ok(val) => {
                    if val != 0{
                        new_nonce += 1;
                    }else{  
                        break;
                    }
                }
                Err(_)=>{
                    new_nonce += 1;
                    continue;
                }
            }            
        }
        new_nonce
    }
    pub fn is_chain_valid(&self,difficulty:usize)->bool{
        let mut previous_block = self.chain.first().unwrap();
        let mut block_index = 1;
        while block_index < self.chain.len() {
            let block = &self.chain[block_index];
            if block.previous_hash != self.hash(previous_block){
                return false;
            }

           // let previous_nonce = previous_block.nonce;
            let nonce = block.nonce;
            let hash = self.hash(nonce);
            let slice = &hash[..difficulty];
            match slice.parse::<u32>(){
                Ok(val) => {
                    if val == 0{                                         
                        break;
                    }
                }
                Err(_)=>{
                    continue;
                }
            }      
            previous_block = block;
            block_index += 1;
        }
        true
    }
}
