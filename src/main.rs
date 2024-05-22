#![allow(unused_variables)]
#![allow(unused_imports)]

use std::thread;
use std::sync::mpsc::channel;
use schnorrkel::{ vrf::{VRFInOut, VRFPreOut, VRFProof},Keypair, PublicKey,};

const CARDS_CON: u16 = 52;
const DRAWS_CON: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DrawAction {
	value: u8,
    public_key: PublicKey,
	signature: [u8; 97],
    thread_id: u8,
}


fn main() {
    let seed_for_vrf = &[0u8; 32];

    let mut thread_winning: i32 = 0;
	let mut thread_winning_score: u16 = 0;

    // Create a shared channel that can be sent along from many threads
    // where tx is the sending half (tx for transmission), and rx is the receiving
    // half (rx for receiving).
    let (tx, rx) = channel();

    //We use 5 threads (the number of members for group 6)
    for i in 0..5 {
		let cloned_transmission = tx.clone();

		thread::spawn(move || {
            // We use random number generator that retrieves randomness from the operating system to create a keypair with schnorrkel
			let mut csprng = rand_core::OsRng;

			let keypair = Keypair::generate_with(&mut csprng);

			let generated_draw = draws_cards(&keypair, seed_for_vrf);

			let (card_available, signature_available) = generated_draw[0];
			let public_key = keypair.public;
			cloned_transmission.send(DrawAction { value: card_available as u8,public_key, signature: signature_available, thread_id: i as u8 }).unwrap();
		});
	}

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
	(0..DRAWS_CON).filter_map(|i| draw_card(keypair, seed, i)).collect()
}
