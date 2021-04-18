mod libpacman;

use libpacman::{BreadthFirst, Map, Marker, Step, Tile, count_ghosts, counter, print_pretty_slice, read_value};

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

    // Hvis der kun er et felt, så kan der ikke både være en pacman og et spøgelse.
    if n == 1 {
        println!("0");
        return ()
    }
    
    let bfs: BreadthFirst = BreadthFirst::new(&map, &map.pacmen[0]);

    let counter = counter(bfs, |(_x, t , _d)| t == Tile::Ghost);

    println!("{}", counter);
}

// Q3
//fn main() {
//
//    let n = read_value::<usize>().unwrap();
//    let map: Map = Map::new(n);
//
//    // Hvis der ikke er nogen pacmen, så er der ikke noget at gøre.
////        if map.pacmen.len() == 0 {
////            println!("");
////            return ()
////        }
//
//    // Hvis der kun er et felt, så er der ikke nogen spøgelser.
//        //else if n == 1 {
////        if n == 1 {
////            println!("");
////            return ()
////        }
//
//
//    let bfs: BreadthFirst = BreadthFirst::new(&map, &map.pacmen[0]);
//
//    let mut bfs_iter = bfs.into_iter();
//
//    let mut paths = Vec::with_capacity(n ^ 2);
//
//    'outer: loop {
//        if let Some((vertex, tile, distance)) = bfs_iter.next() {
//            paths.push((vertex, tile, distance));
//            if tile == Tile::Ghost {
//                break 'outer;
//            }
//        }
//        else {
//            break 'outer;
//        }
//    }
//    // In doing this we assume that the last tile encountered is a ghost, so at least one ghost was reachable.
//    // If this is undesirable behaviour this can be curcumvented with a simple check of the form
//    // if paths[paths.len() - 1].1 != Tile::Ghost {println!("")}
//    // else {print_pretty_slice(&BreadthFirst::backtrace_path(&paths, 0, paths.len()-1));}
//    // note that this would still fail if no tiles were reachable from the source, since paths.len() - 1 would underflow: (0 - 1) => underflow.
//    print_pretty_slice(&BreadthFirst::backtrace_path(&paths, 0, paths.len()-1));
//}

// Q4 Naive
//fn main() {
//
//    let n = read_value::<usize>().unwrap();
//    let map: Map = Map::new(n);
//
//    // Hvis der ikke er nogen pacmen, så er der ikke noget at gøre. -- Hvis I vil have yderligere forklaring, så spørg.
//        if map.pacmen.len() == 0 || map.ghosts.len() == 0 {
//            println!("0");
//            return ()
//        }
//
//    // Hvis der kun er et felt, så er der ikke nogen spøgelser. -- Hvis I vil have yderligere forklaring, så spørg.
//        else if n == 1 {
//            println!("0");
//            return ()
//        }
//
//    if map.ghosts.len() <= map.pacmen.len() {
//    let mut distances: Vec<usize> = Vec::with_capacity(map.ghosts.len());
//
//    for ghost in map.ghosts.iter() {
//
//    let bfs: BreadthFirst = BreadthFirst::new(&map, ghost);
//
//    for (_vertex, tile, distance) in bfs.into_iter() {
//            if tile == Tile::Pacman {
//                distances.push(distance);
//            }
//    }
//    }
//
//    match distances.iter().min(){
//        Some(min) => println!("{}", min),
//        None => println!("0"),
//    }
//}
//else {
//
//    let mut distances: Vec<usize> = Vec::with_capacity(map.pacmen.len());
//
//    for pacman in map.pacmen.iter() {
//
//    let mut bfs: BreadthFirst = BreadthFirst::new(&map, pacman);
//
//    let bfs_iter = &mut bfs;
//
//    for (_vertex, tile, distance) in bfs_iter{
//            if tile == Tile::Ghost {
//                distances.push(distance);
//            }
//    }
//
//    }
//
//    match distances.iter().min(){
//        Some(min) => println!("{}", min),
//        None => println!("0"),
//    }
//}
//
//}

// Q4, but faster 
//fn main() {
//
//    let n = read_value::<usize>().unwrap();
//    let map: Map = Map::new(n);
//
//
//    // Hvis der kun er et felt, så kan der ikke både være en pacman og et spøgelse.
//        if n == 1 {
//            println!("0");
//            return ()
//        }
//
//    if map.ghosts.len() <= map.pacmen.len() {
//
//    let mut ghost_iter = map.ghosts.iter();
//
//    if let Some(ghost) = ghost_iter.next() {
//
//    let mut bfs: BreadthFirst = BreadthFirst::new(&map, ghost);
//
//    // This will always be a ghost, since we just enqued a ghost.
//    // The same holds true in the following loop.
//    bfs.next();
//
//    for ghost in ghost_iter {
//        bfs.queue.push_back((*ghost, Tile::Ghost, 0));
//        bfs.marker.mark(*ghost);
//        bfs.next();
//    }
//
//    loop {
//        match bfs.next() {
//            Some((_vertex, tile, distance)) => {
//                if tile == Tile::Pacman {
//                    println!("{}", distance);
//                    break
//                }
//            },
//            None => break,
//
//            }
//        }
//    }
//
//}
//else {
//
//    let mut pacman_iter = map.pacmen.iter();
//
//    if let Some(pacman) = pacman_iter.next() {
//
//    let mut bfs: BreadthFirst = BreadthFirst::new(&map, pacman);
//
//    // This will always be a pacman, since we just enqued a pacman.
//    // The same holds true in the following loop.
//    bfs.next();
//
//    for pacman in pacman_iter {
//        bfs.queue.push_back((*pacman, Tile::Pacman, 0));
//        bfs.marker.mark(*pacman);
//        bfs.next();
//    }
//
//    loop {
//        match bfs.next() {
//            Some((_vertex, tile, distance)) => {
//                if tile == Tile::Ghost {
//                    println!("{}", distance);
//                    break
//                }
//            },
//            None => break,
//
//            }
//        }
//    }
//
//}
//
//}
