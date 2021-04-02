use std::{collections::VecDeque, io, str::FromStr};

pub fn read_value<T: FromStr>() -> Result<T, T::Err> {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("could not read from stdin");
    s.trim().parse()
}

pub fn read_values<T: FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("could not read from stdin");
    s.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}

fn range_check<T, C, U, F>(arg: T, low: C, high: C, func: F) -> Option<U>
where
    T: PartialOrd<C>,
    C: PartialOrd<T>,
    F: Fn(T) -> U,
{
    if low <= arg && arg <= high {
        Some(func(arg))
    } else {
        None
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
            _ => unreachable!(),
        }
    }
}

pub struct Map {
    graph: Vec<Vec<Tile>>,
    pacmen: Vec<(usize, usize)>,
    ghosts: Vec<(usize, usize)>,
}

impl Map {
    fn new(n: usize) -> Self {
        // capacity = n, because the graph is an n x n matrix.
        let mut graph: Vec<Vec<Tile>> = Vec::with_capacity(n);
        let mut pacmen: Vec<(usize, usize)> = Vec::new();
        let mut ghosts: Vec<(usize, usize)> = Vec::new();

        for row_idx in (0..n).into_iter() {
            // We read a new row of the map.
            let mut s = String::with_capacity(n);
            io::stdin()
                .read_line(&mut s)
                .expect("could not read from stdin");

            // If only we didn't have to be optimal
            //            let row: Vec<Tile> = s.trim().chars().map(|c| Tile::new(c)).collect();
            //            graph.push(row);

            let mut row: Vec<Tile> = Vec::with_capacity(n);

            for (col_idx, char) in s.char_indices() {
                match char {
                    ' ' => row.push(Tile::Blank),
                    '#' => row.push(Tile::Wall),
                    'G' => {
                        row.push(Tile::Ghost);
                        ghosts.push((row_idx, col_idx));
                    }
                    'P' => {
                        row.push(Tile::Pacman);
                        pacmen.push((row_idx, col_idx));
                    }
                    _ => panic!(),
                }
            }
            graph.push(row)
        }

        Self {
            graph,
            pacmen,
            ghosts,
        }
    }

    fn len(&self) -> usize {
        return self.graph.len();
    }

    fn neighbours(&self, vertex: &(usize, usize)) -> [Option<usize>; 4] {
        // (r-1, r+1, c-1, c+1)
        return [
            range_check(vertex.0, 1, self.len(), |x| x - 1),
            range_check(vertex.0, 0, self.len() - 1, |x| x + 1),
            range_check(vertex.1, 1, self.len(), |x| x - 1),
            range_check(vertex.1, 0, self.len() - 1, |x| x + 1),
        ];
    }
}

// This could also be done with HashSet, we can test performance later
pub struct BreadthFirst<'a> {
    visited: ColorMarker,
    // Here it is implicit that the colorgrid and Map Follow the same lookup scheme
    queue: VecDeque<(usize,usize)>,
    graph: &'a Map,
}

impl<'a> BreadthFirst<'a> {

    pub fn new(graph: &'a Map, root: (usize,usize)) -> Self {
        let mut nodes: ColorMarker = ColorMarker::new(graph.len(), graph.len());
        let mut queue: VecDeque<(usize,usize)> = VecDeque::new();

        let _neighbours: [Option<usize>; 4] = graph.neighbours(&root);
        let mut neighbours = _neighbours.iter();

        for _ in 0..2 {
            if let Some(neighbour) = neighbours.next().unwrap() {
                queue.push_front((*neighbour,root.1))
            }
        } 

        for _ in 0..2 {
            if let Some(neighbour) = neighbours.next().unwrap() {
                queue.push_front((root.0, *neighbour))
            }
        } 

        nodes.mark(root);

        Self{
            visited: nodes,
            queue,
            graph,
        }

    }

}

impl<'a> Iterator for BreadthFirst<'a> {
    type Item = 
}

pub trait Marker {
    type Index;

    fn marked(&self, idx: Self::Index) -> bool;

    fn marked_or_mark(&mut self, idx: Self::Index) -> bool;

    fn mark(&mut self, idx: Self::Index);
}

#[derive(Debug, PartialEq)]
enum Color {
    White,
    Black,
}

impl Color {
    fn new() -> Self {
        Color::White
    }

    fn flip(&mut self) {
        match self {
            &mut Color::White => *self = Color::Black,
            &mut Color::Black => *self = Color::White,
        }
    }
}

#[derive(Debug)]
struct ColorMarker {
    color_grid: Vec<Vec<Color>>,
}

impl ColorMarker {
    fn new(rows: usize, columns: usize) -> Self {
        let color_grid: Vec<Vec<Color>> = (0..rows)
            .map(|_r| (0..columns).map(|_c| Color::White).collect())
            .collect();
        Self { color_grid }
    }
}

impl Marker for ColorMarker {

    type Index = (usize, usize);

    fn mark(&mut self, idx: Self::Index) {
       self.color_grid[idx.0][idx.1] = Color::Black; 
    }

    fn marked(&self, idx: Self::Index) -> bool {
        self.color_grid[idx.0][idx.1] == Color::Black
    }

    fn marked_or_mark(&mut self, idx: Self::Index) -> bool {
        if self.marked(idx) {
            true
        }
        else {
            self.mark(idx);
            false
        }
    }

}

// T must implement the trait Color;
// T should just be able to corse into Item
pub fn bfs<T, D, F>(_source: T, graph: D, c_func: F) -> usize
where
    D: IntoIterator<Item = T>,
    F: Fn(T) -> bool,
{
    let mut counter = 0;

    for i in graph.into_iter() {
        if c_func(i) {
            counter += 1;
        }
    }

    return counter;
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
        io::stdin()
            .read_line(&mut s)
            .expect("could not read from stdin");

        for c in s.chars() {
            match c {
                'G' => counter += 1,
                _ => (),
            }
        }
    }

    counter
}
