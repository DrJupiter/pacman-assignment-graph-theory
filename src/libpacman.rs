use std::fmt::{Display, Formatter};
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

pub fn print_pretty_slice<T: Display>(slice: &[T]) {
    let _str = slice
        .iter()
        .map(|num| format!("{} ", num.to_string()))
        .collect::<String>();
    println!("{}", _str.trim_end());
}

pub struct Step((usize, usize), Tile, usize);

impl Step {
    pub fn new(vertex: (usize, usize), tile: Tile, distance: usize) -> Self {
        Self(vertex, tile, distance)
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Vertex: {} {}, Tile: {}, Distance: {}",
            self.0 .0, self.0 .1, self.1, self.2
        )
    }
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
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

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            &Tile::Blank => "Blank",
            &Tile::Ghost => "Ghost",
            &Tile::Pacman => "Pacman",
            &Tile::Wall => "Wall",
        };
        write!(f, "{}", t)
    }
}

#[derive(Debug)]
pub struct Map {
    pub graph: Vec<Vec<Tile>>,
    pub pacmen: Vec<(usize, usize)>,
    pub ghosts: Vec<(usize, usize)>,
}

impl Map {
    pub fn new(n: usize) -> Self {
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

            for (col_idx, char) in s.trim_end_matches("\n").char_indices() {
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

    pub fn from_strings<'a>(string_vec: Vec<&'a str>) -> Self {
        let mut graph: Vec<Vec<Tile>> = Vec::with_capacity(string_vec.len());
        let mut pacmen: Vec<(usize, usize)> = Vec::new();
        let mut ghosts: Vec<(usize, usize)> = Vec::new();

        let mut s_iter = string_vec.iter();

        for row_idx in (0..string_vec.len()).into_iter() {
            let mut row: Vec<Tile> = Vec::with_capacity(string_vec.len());

            if let Some(s) = s_iter.next() {
                for (col_idx, char) in s.trim_end_matches("\n").char_indices() {
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

    fn neighbours_numbers(&self, vertex: &(usize, usize)) -> [Option<usize>; 4] {
        // (r-1, r+1, c-1, c+1)
        return [
            range_check(vertex.0, 1, self.len(), |x| x - 1),
            range_check(vertex.0, 0, self.len() - 2, |x| x + 1),
            range_check(vertex.1, 1, self.len(), |x| x - 1),
            range_check(vertex.1, 0, self.len() - 2, |x| x + 1),
        ];
    }

    fn neighbours(&self, vertex: &(usize, usize)) -> [Option<(usize, usize)>; 4] {
        // If only they didn't use 1.43
        // let mut n_numbers = std::array::IntoIter::new(self.neighbours_numbers(vertex));

        let _n_numbers = self.neighbours_numbers(vertex);

        let mut n_numbers = _n_numbers.into_iter();

        let mut neighbours: [Option<(usize, usize)>; 4] = [None; 4];

        for i in 0..2 {
            if let Some(n_row) = n_numbers.next().unwrap() {
                neighbours[i] = Some((*n_row, vertex.1))
            }
        }

        for i in 2..4 {
            if let Some(n_col) = n_numbers.next().unwrap() {
                neighbours[i] = Some((vertex.0, *n_col))
            }
        }

        neighbours
    }

    fn node_type_checked(&self, vertex: &(usize, usize)) -> Option<Tile> {
        match self.graph.get(vertex.0) {
            Some(tile_vec) => match tile_vec.get(vertex.1) {
                Some(tile) => Some(*tile),
                None => None,
            },
            None => None,
        }
    }

    fn node_type(&self, vertex: &(usize, usize)) -> Tile {
        self.graph[vertex.0][vertex.1]
    }

    fn adjacent(vertex1: &(usize, usize), vertex2: &(usize, usize)) -> bool {
        let r_distance = vertex2.0 as isize - vertex1.0 as isize;
        let c_distance = vertex2.1 as isize - vertex1.1 as isize;
        r_distance.abs() <= 1 && c_distance.abs() <= 1 
    }

    fn adjacent_delta(vertex1: &(usize, usize), vertex2: &(usize, usize)) -> (bool, (isize, isize)) {
        let r_distance = vertex2.0 as isize - vertex1.0 as isize;
        let c_distance = vertex2.1 as isize - vertex1.1 as isize;
        (r_distance.abs() <= 1 && c_distance.abs() <= 1, (r_distance, c_distance))
    }

}

// This could also be done with HashSet, we can test performance later
pub struct BreadthFirst<'a> {
    pub marker: ColorMarker,
    // Here it is implicit that the colorgrid and Map Follow the same lookup scheme
    pub queue: VecDeque<((usize, usize), Tile, usize)>,
    graph: &'a Map,
}

impl<'a> BreadthFirst<'a> {
    pub fn new(graph: &'a Map, root: &'a (usize, usize)) -> Self {
        //        let mut marker: ColorMarker = ColorMarker::new(graph.len(), graph.len());
        let marker: ColorMarker = ColorMarker::new(graph.len(), graph.len());
        let mut queue: VecDeque<((usize, usize), Tile, usize)> = VecDeque::new();

        //        let neighbours = graph.neighbours(root);
        //
        //        for neighbour in neighbours.iter() {
        //            if let Some(n) = neighbour {
        //                queue.push_front((*n, graph.node_type(n), 1))
        //            }
        //        }
        //        marker.mark(*root);
        queue.push_front((*root, graph.node_type(&root), 0));

        Self {
            marker,
            queue,
            graph,
        }
    }

   pub fn backtrace_path(search: &[((usize, usize), Tile, usize)], root_idx: usize, target_idx: usize) -> Vec<char> {



   let search_interval: &[((usize, usize), Tile, usize)]   = &search[root_idx..target_idx];

   let (mut current_vertex, mut current_distace) = (&search[target_idx].0, &search[target_idx].2);

   //let mut direction_string: String = String::with_capacity(search_interval.len());
   let mut direction_string: Vec<char> = Vec::with_capacity(search_interval.len());

   for (vertex, tile, distance) in search_interval.iter().rev() {
        if distance == current_distace {
            continue
        }  
        else {
            //let delta: (isize, isize) = (vertex.0 as isize - current_vertex.0 as isize, vertex.1 as isize - current_vertex.1 as isize);
            match Map::adjacent_delta(current_vertex, vertex) {
                (false, _) => (),
                (true, delta) => {direction_string.push(Self::backtrace_direction(delta)); current_vertex = vertex; current_distace = distance;}, 
            }
        }
   }

   direction_string.reverse();
   direction_string

   }

   fn forwardtrace_direction(delta: (isize, isize)) -> char {
       match delta {
           (-1, 0) => 'N',
           (0, 1) => 'E',
           (0, -1) => 'W',
           (1, 0) => 'S',
           _ => unreachable!(),
       }
   }


   fn backtrace_direction(delta: (isize, isize)) -> char {
       match delta {
           (-1, 0) => 'S',
           (0, 1) => 'W',
           (0, -1) => 'E',
           (1, 0) => 'N',
           _ => unreachable!(),
       }
   }

}

impl<'a> Iterator for BreadthFirst<'a> {
    //      Vertex      Vertex Type  Distance
    type Item = ((usize, usize), Tile, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_back() {
            None => None,
            Some((vertex, tile, distance)) => {
                for neighbour in self.graph.neighbours(&vertex).iter() {
                    if let Some(neighbour) = neighbour {
                        if self.marker.marked_or_mark(*neighbour) {
                            continue;
                        } else {
                            let tile = self.graph.node_type(neighbour);
                            if tile == Tile::Wall {
                                continue;
                            } else {
                                self.queue.push_front((*neighbour, tile, distance + 1))
                            }
                        }
                    }
                }
                //                self.marker.mark(vertex);
                Some((vertex, tile, distance))
            }
        }
    }
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
pub struct ColorMarker {
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
        } else {
            self.mark(idx);
            false
        }
    }
}

// T must implement the trait Color;
// T should just be able to corse into Item
pub fn counter<T, D, F>(graph: D, c_func: F) -> usize
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_from_strings() {
        let s_vec = vec!["  P", "  #", "G #"];

        let map = Map::from_strings(s_vec);

        dbg!(map);
    }
    #[test]
    fn breadth_first() {
        let map: Map = Map::new(3);

        //        let v = Vec::from(map.neighbours(&map.pacmen[0]));

        //       dbg!(v);

        let bfs: BreadthFirst = BreadthFirst::new(&map, &(0, 0));
        println!("Queue: {:?}", bfs.queue);
        //map.graph[0][2];
        let v: Vec<_> = bfs.into_iter().collect();

        println!("{:?}", v);

        //for s in bfs.into_iter() {
        //    println!("{:?}", s);
        //}
    }
}
