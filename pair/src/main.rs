extern crate bn;
extern crate rand_core;
extern crate rand;

use bn::{pairing, Fr, G1, G2};
use crate::bn::Group;






fn main() {
    let rng = &mut  rand::thread_rng();
  //generate private key 
    let a = Fr::random(rng);
    let b = Fr::random(rng);
    let c = Fr::random(rng);

 //public key
    let (pa,qa) = (G1::one()*a,G2::one()*a);
    let (pb,qb) = (G1::one()*b,G2::one()*b);
    let (pc,qc) = (G1::one()*c,G2::one()*c);

    //get shared secret 

    let alice_ss = pairing(pb,qc).pow(a);
    let bob_ss = pairing(pc, qa).pow(b);
    let ron_ss = pairing(pa, qb).pow(c);


    if alice_ss == bob_ss && bob_ss == ron_ss {
        println!("sharing has worked!");
    }
    else {
       println!("sharing has failed!");
    }
   }
    

