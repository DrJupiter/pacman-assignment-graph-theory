use std::{io, str::FromStr};

pub fn read_value<T: FromStr>() -> Result<T, T::Err> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("could not read from stdin");
    s.trim().parse()
}



pub fn read_values<T: FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("could not read from stdin");
    s.trim().split_whitespace().map(|word| word.parse()).collect()
}

pub fn adjacency_matrix(n: usize) {

    let v: Vec<Vec<bool>> = Vec::with_capacity(n);

    // loop
    // false -> edge, true -> no edge
    // we maintain a stack

    // realistically will you iterate through this graph more than once?
    for i in (0..n).into_iter() {
        let mut forward_feed: bool = false;


    }

}
// T must implement the trait Color;
// T should just be able to corse into Item
pub fn bfs<T, D, F>(_source: T, graph: D, c_func: F) -> usize where D: IntoIterator<Item = T>, F: Fn(T) -> bool{

    let mut counter = 0;

    for i in graph.into_iter() {
        if c_func(i) {
            counter += 1;
        }
    }

    return counter
}

// N S W E

// fn parse_tile(c: char) {
//     match c {
//         '#' => 0,
//         ' ' => 1,
//     }
// }

/// Takes an integer n and reads n lines from stdin
/// and counts the number of ghost occurences
pub fn count_ghosts(n: u32) -> u32 {
    let mut counter = 0; 

    for _ in (0..n).into_iter() {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("could not read from stdin");

        for c in s.chars() {
            match c {
                'G' => counter += 1,
                _ => (),
    
    } } } 

    counter

}

// ``` 
// Vec::with_capacity(n^2*4)
// 
// n=2
//  0       1       2       3
// [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
// let mut v : Vec<Tile> = Vec::with_capacity(n^2)
// // NESW
// // WENS
// // NEWS
// // EWSN
// 
// for tile in v.iter() {
//     match tile {
//         Tile::Wall => (),
//         Tile::Pacman((N, S, E, W)) =>, 
//         Tile::Ghost((N, S, E, W))  =>, 
//         Tile::Blank((N, S, E, W))  =>, 
//     }
// }
// 
// struct Map {
//     adj_l: Vec<Tile>,
//     pac_pos: Vec<usize>,
// 
// }
// 
// enum Tile {
//     Wall,
//     Pacman([bool;4]),
//     Ghost([bool;4]),
//     Blank([bool;4]),
// }
// 
// " #  "
// "  # "
// " #  "
// ``` 


