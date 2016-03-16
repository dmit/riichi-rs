extern crate rand;

mod tile;

fn main() {
    let mut tiles = tile::Set::new();
    tiles.shuffle();

    let mut hand = tiles.take_hand().unwrap();

    println!("Initial hand ({}-shanten): {}", hand.shanten(), hand);

    let tile = tiles.take().unwrap();
    hand.take(5);
    hand.put(tile);

    println!("Updated hand ({}-shanten): {}", hand.shanten(), hand);
}
