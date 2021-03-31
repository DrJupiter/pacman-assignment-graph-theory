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


// Potentially change to usize or just use on off mechanics for 
// multiplayer
enum Tile {
    Pacman,
    Ghost,
    Blank,
    Wall,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            ' ' => Tile::Blank,
            '#' => Tile::Wall,
            'G' => Tile::Ghost,
            'P' => Tile::Pacman,
            _ => panic!(),
        }
    }

//    fn flip(&mut self, direction: usize) {
//       match self {
//            &mut Tile::Wall => (),
//            &mut Tile::Blank(ref mut neighbours) => neighbours[direction] = !neighbours[direction],
//            &mut Tile::Ghost(ref mut neighbours) => neighbours[direction] = !neighbours[direction],
//            &mut Tile::Pacman(ref mut neighbours) => neighbours[direction] = !neighbours[direction],
//       } 
//    }
}

struct Map {
    graph: Vec<Vec<Tile>>,
    pacmen: Vec<(usize,usize)>,  
    ghosts: Vec<(usize,usize)>,
}

impl Map {

    fn new(n: usize) -> Self {

        // Allocate space for the vector. n^2 because the map will contain n^2 tiles.
        let mut graph: Vec<Vec<Tile>> = Vec::with_capacity(n);
        let mut pacmen: Vec<(usize, usize)> = Vec::new();
        let mut ghosts: Vec<(usize, usize)> = Vec::new();

        for row_idx in (0..n).into_iter() {

            // We read a new row of the map.
            let mut s = String::with_capacity(n);
            io::stdin().read_line(&mut s).expect("could not read from stdin");

            // If only we didn't have to be optimal
//            let row: Vec<Tile> = s.trim().chars().map(|c| Tile::new(c)).collect();
//            graph.push(row);            
        
        let row: Vec<Tile> = Vec::with_capacity(n);           

        for (col_idx, char ) in s.char_indices() {
            match c {
                ' ' => row.push(Tile::Blank),
                '#' => row.push(Tile::Wall),
                'G' => {row.push(Tile::Ghost); ghosts.push((row_idx,col_idx));},
                'P' => {row.push(Tile::Pacman); pacmen.push((row_idx,col_idx));},
                _ => panic!(),
            }
        }

        }
        
        Self {
            graph,
            pacmen,
            ghosts,
        }
    }

    const fn len(&self) -> usize {
        return self.graph.len()
    }

    fn neighbours(&self, vertex: (usize,usize)) -> [Option<usize>;4] {
    // (r-1, r+1, c-1, c+1)
    return [range_check(vertex.0, 1, self.len(), |x| x-1), range_check(vertex.0, 0, self.len()-1, |x| x + 1), range_check(vertex.1, 1, self.len(), |x| x-1), range_check(vertex.1, 0, self.len()-1, |x| x + 1)]
}

}

fn range_check<T,U,F>(arg: T, low: T, high: T, func: F) -> Option<U> where T: PartialOrd, F: Fn(T) -> U {
    if low <= arg && arg <= high {
        Some(func(arg))
    } 
    else {
        None
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


