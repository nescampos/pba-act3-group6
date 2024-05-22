#![allow(unused_variables)]
#![allow(unused_imports)]

use std::thread;
use std::sync::mpsc::channel;
use schnorrkel::{ vrf::{VRFInOut, VRFPreOut, VRFProof},Keypair, PublicKey,};
use merlin::Transcript;

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

    //We use 5 threads (the number of members for group 6) to draw a card for every member
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

    //Now we receive the transactions from the shared channel to verify the winner
    for i in 0..5 {
        // We get the response from the channel
        let ans = rx.recv().unwrap();

        //Verify the card
        

        //Verify the winner
	}

    // print the output
	println!("The winning thread is: {:#?}", thread_winning);
	println!("The winning thread score is: {}", thread_winning_score);
}




// From the VRF we generate the current draw card
fn compute_card(io: &VRFInOut) -> Option<u16> {
    let b: [u8; 8] = io.make_bytes(b"new_card");
	Some((u64::from_le_bytes(b) % (CARDS_CON as u64)) as u16)
}



fn draw_card(keypair: &Keypair, seed: &[u8; 32], draw_num: u8) -> Option<(u16, [u8; 97])> {
    
	let game_try_transcript = validate_try(seed, draw_num)?;
	let (io, proof, _) = keypair.vrf_sign(game_try_transcript);
	let computed_card = compute_card(&io)?;
	let mut vrf_signature = [0u8; 97];
	// the first 32 bytes are io
	vrf_signature[..32].copy_from_slice(&io.to_preout().to_bytes()[..]);
	// the next 64 bytes are the proof
	vrf_signature[32..96].copy_from_slice(&proof.to_bytes()[..]);
	// the final byte is the draw number
	vrf_signature[96] = draw_num;
	Some((computed_card, vrf_signature))
}


fn draws_cards(keypair: &Keypair, seed: &[u8; 32]) -> Vec<(u16, [u8; 97])> {
    // Take out the cards we have, according to the seed that is given to you.
	(0..DRAWS_CON).filter_map(|i| draw_card(keypair, seed, i)).collect()
}

fn validate_try(seed: &[u8; 32], draw_num: u8) -> Option<Transcript> {
    // Validate the card number
    if draw_num > DRAWS_CON {
		return None;
	}
    //Checking the validation  of the number of draws
	let mut game_try = Transcript::new(b"Deck Poker");
	game_try.append_message(b"seed_game", seed);
	game_try.append_u64(b"draw_game", draw_num as u64);
    Some(game_try)
}