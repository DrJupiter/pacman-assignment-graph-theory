mod libpacman;
use libpacman::{counter, count_ghosts, read_value, Map, BreadthFirst, Tile};

//mod libgraph;

// Q1
//fn main() {
//    let n = read_value::<u32>().unwrap();
//    println!("{}", count_ghosts(n));
//}

// Q2
fn main() {
    let n = read_value::<usize>().unwrap();

    let map: Map = Map::new(n);

    // Hvis der ikke er nogen pacmen, så er der ikke noget at gøre. -- Hvis I vil have yderligere forklaring, så spørg.
    if map.pacmen.len() == 0 {
        println!("0");
        return ()
    }

    // Hvis der kun er et felt, så er der ikke nogen spøgelser. -- Hvis I vil have yderligere forklaring, så spørg.
    else if n == 1 {
        println!("0");
        return ()
    }

    let bfs: BreadthFirst = BreadthFirst::new(&map, &map.pacmen[0]);

    let counter = counter(bfs, |(_x, t , _d)| t == Tile::Ghost);

    println!("{}", counter);

}
