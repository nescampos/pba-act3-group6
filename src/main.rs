#![allow(unused_variables)]
#![allow(unused_imports)]

use std::thread;
use schnorrkel::{ vrf::{VRFInOut, VRFPreOut, VRFProof},Keypair, PublicKey,};

const CARDS: u16 = 52;
const DRAWS: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DrawAction {
	value: u8,
	thread_id: u8,
    public_key: PublicKey,
	signature: [u8; 97],
}


fn main() {
    let seed_for_vrf = &[0u8; 32];

    let mut thread_winning: i32 = 0;
	let mut thread_winning_score: u16 = 0;

    // print the output
	println!("The winning thread is: {:#?}", thread_winning);
	println!("The winning thread score is: {}", thread_winning_score);
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
    // Take out the cards we have, according to the seed that is given to you.
	(0..DRAWS).filter_map(|i| try_draw(keypair, seed, i)).collect()
}
