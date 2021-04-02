mod libpacman;
use libpacman::{bfs, count_ghosts, read_value};

mod libgraph;

// Q1
//fn main() {
//    let n = read_value::<u32>().unwrap();
//    println!("{}", count_ghosts(n));
//}

fn main() {
    let n = read_value::<u32>().unwrap();

    //    let map = adjacency_matrix(n);
    //    let counter = bfs(map.pacmanpos, map.graph, |x| x == Tile::Ghost);
    let counter = bfs(false, std::array::IntoIter::new([false; 4]), |b| b == false);
    dbg!(counter);
}
