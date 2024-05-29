extern crate bn;
extern crate rand_core;
extern crate rand;

use bn::{pairing, Fr, G1, G2};
use crate::bn::Group;

use rand_core::OsRng;





fn main() {
    let rng = &mut rand::thread_rng();

    let a = Fr::random(rng);
    let b = Fr::random(rng);
    let c = Fr::random(rng);


    let (pa,qa) = (G1::one()*a,G2::one()*a);
    let (pb,qb) = (G1::one()*b,G2::one()*b);
    let (pc,qc) = (G1::one()*c,G2::one()*c);


    let alice_ss = pairing(pb,qc).pow(a);
    let bob_ss = pairing(pb, qb).pow(b);
    let ron_ss = pairing(pc, qc).pow(c);


    if alice_ss == bob_ss && bob_ss == ron_ss {
        println!("\nSharing has worked!");
    }
    else {
       println!("\nSharing has failed!");
    }
   }
    

