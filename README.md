# Introduction til krav

Vi skal aflevere både på gradescope og på [CodeJudge](https://dtu.codejudge.net/02105-f21/collection/3068/view), [GradeScope](https://www.gradescope.com/courses/226765).

## Requirements for the written propotion

Vi skal skrive vores svar i blokkene på pdferne til hver respektiv afleverings spørgsmål.


 - Vi skal have så tight asymptotic bounds som muligt $O(n^2) \rightarrow O(n^2 + n)$, derfor er $\Theta$ nok også bedst at bruge.

- Når vi beskriver en algoritme, så skal det gøres ved brug af naturligt sprog. __Ved beskrivelsen af en ny algortime så skal den altid analyseres i forhold til space og time complexity__. Og mådan analysen er foretaget skal detaljeres.

- Når vi skal argumentere for at en algortime er korrekt skal det være kort og præcist fx ved loop invariance eller bare rent sprogligt, hvis det er forsvarligt at gøre det sådan.

# Introduktion til pacman problemet


## Map definitions
- `# = wall`, `G = ghost`, `P= pacman`, ` blank_space = nothing`

# Problem 1 Counting Ghosts

## Description
Given a map calculate the number of ghosts present on the map.

## Implementation

Read each line and add 1 to a counter each time a ghost, `G`, is encountered.
This will take O(n^2) time.
This will consume O(n^2) space.



# Problem 2 Reachable ghosts

## Description
Given a map calculate the number of ghosts that can reach pacman.

## Implementation

We construct a graph, then perform exploration with BFS eller DFS with the root vertex being pacman and then each time we encounter a vertex with a new ghost we add 1 to a counter.

BFS takes O(V + E) time, in this problem it will therefore take O(n^2) time.


# Problem 3 Closest Ghost

## Description
Given a map calculate the shortest path between any ghost and the pacman.

## Implementation

We construct a graph and perform BFS and pick one of the paths that minimizes the distance.
Beak when the first ghost is encountered. There can be no path shorter than this, thus all subsequent paths are irrelevant

BFS takes O(V + E) time, in this problem it will therefore take O(n^2) time.

# Problem 4 Multiplayer Closest Ghost

## Description
Given a map with multiple pacmans, calculate the shortest distance between any pacman and a ghost.

## Implementation

We create a graph and perform BFS from each pacperson, except that the algorithm terminates when the first ghost is encountered, since no shorter paths to ghost can exist due to BFS being layer by layer.

BFS takes O(V + E) time, in this problem it will therefore take O(n^2) time.

# Psuedo Code
```rust
enum Tile {
    Pacman((usize,usize)),
    Wall((usize,usize)),
    Ghost((usize,usize)),
    Blank((usize,usize)),
}

struct Map {
    pacman: Tile,
    ghost: Vec<Tile>,
    adj_m: Vec<Vec<bool>>,
}
    1 = x=0, y=0
    2 = x=1, y=0
    3

///   ADJECENCY MATRIX
//    ================      // 
//   1 2 3 4 5 6 7 8 9 
// 1 0 1 0 0 1 0 0 1 0
// 2 1 0 1
// 3 0 1 0
// 4 0 1 0
// 5 0 1 0
// 6 0 1 0


//    ================      // 
///   N S W E
// 1: 0 1 0 0 
// 2: 1 0 1 1
// 3: 0 1 0 1
// 4: 0 1 0 0
// 5: 0 1 0 0
// 6: 0 1 0 0

// (1,1) -> (1,2) -> (2,1)

// NxN 1, 1 + 1, 1 + 3, 1 - 1, 1-3

fn check_neighbouhrs(&self, idx) {
    idx + 1, idx+n, idx-1, idx-n
}
/*
for dx in (-1..=1).into_iter() {
    for dy in (-1..=1).into_iter() {
        let newX = x + dx;
        let newY = x + dy;

        // OUT OF BOUNDS
        if (newX < 0 || newX => n || newY < 0 || newY => n) 
            continue;
*/

enum Option<T> {
    Some(T),
    None,
}

match matrix.get(idx) {
    Some(Tile) => ,
    None => Fuck that,
}

123
456
789

1234
5678
9(10)(11)(12)

// V -> N -> S -> E -> W


// 5 0
// 6 0
// 7 0
// 8 0
// 9 0

// 




fn GetValidNeighbors (x : i32, y :i32) -> Vec<Tile> {

    Vec<Tile> result;
    match matrix.get(idx) {
        Some(Tile) => match Tile {
            Tile::Wall => ,
            Tile::Pacman => /*Found pacman, send info to algo and end program*/ FoundPacman(Tile);
            ,
            _ => result.push(Tile);
        }
        None => (),
    }
}

match Tile {
    Tile::Pacman => ,
    Tile::Ghost => ,
    Tile::Blank => result.push(Tile),
    Tile::Wall => (),
}

/*
OLD VERSION 
fn GetValidNeighbors (x : i32, y :i32) -> Vec<Tile> {
    Vec<Tile> result;
    for dx in (-1..=1).into_iter() {
        for dy in (-1..=1).into_iter() {
            let newX = x + dx;
            let newY = y + dy;

            // OUT OF BOUNDS
            if (newX < 0 || newX => n || newY < 0 || newY => n) 
                continue;

            // ADD IF NOT WALL OR SMTHNG
            if (!adj_m[newX, newY])
                result.add(Tile(newX, newY))

            // ADD IF NOT WALL OR SMTHNG 
            if (!IsWall(newX, newY)) 
                result.add(Tile(newX, newY))
        }
    }
    return result;
}
*/


```