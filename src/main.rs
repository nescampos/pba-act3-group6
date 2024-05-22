#![allow(unused_variables)]
#![allow(unused_imports)]

use std::thread;
use schnorrkel::{ vrf::{VRFInOut, VRFPreOut, VRFProof},Keypair, PublicKey,};

fn main() {
    println!("Hello, world!");
}





fn compute_card(io: &VRFInOut) -> Option<u16> {
    todo!()
}

fn draw_card(keypair: &Keypair, seed: &[u8; 32], draw_num: u8) -> Option<(u16, [u8; 97])> {
    todo!()
}



fn verify_card(public: &PublicKey, vrf_signature: &[u8; 97], seed: &[u8; 32]) -> Option<u16> {
    todo!()
}

fn draws_cards(keypair: &Keypair, seed: &[u8; 32]) -> Vec<(u16, [u8; 97])> {
	todo!()
}
