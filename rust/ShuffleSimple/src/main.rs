extern crate rand;

//use rand::prelude::*;
//use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;


fn main() {
    let cards = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];
    //let suits = vec![1,2,3,4];

    let mut rng = thread_rng();

    // choose one
    println!("{:?}", cards.choose(&mut rng));
//    assert_eq!(cards[..0].choose(&mut rng), None);


    // choose all
    let v: Vec<u8> = cards.choose_multiple(&mut rng, 13).cloned().collect();
    
//    // store in a buffer:
//    let mut buf = [0u8; 13];
//    for (b, slot) in cards.choose_multiple(&mut rng, buf.len()).zip(buf.iter_mut()) {
//        *slot = *b;
//    }

    println!("{:?}", v);

    println!("done!");
}
