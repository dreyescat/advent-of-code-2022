// https://en.wikipedia.org/wiki/A*_search_algorithm
use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    let mut heightmap: Vec<Vec<u8>> = vec![];
    let mut start = Square(0, 0);
    let mut end = Square(0, 0);
    for (x, line) in io::stdin().lines().enumerate() {
        let mut row = vec![];
        for (y, byte) in line.unwrap().bytes().enumerate() {
            match byte {
                b'S' => {
                    start = Square(x, y);
                    row.push(b'a');
                }
                b'E' => {
                    end = Square(x, y);
                    row.push(b'z');
                }
                _ => row.push(byte),
            }
        }
        heightmap.push(row);
    }

    let path = climbing_path(&heightmap, start, end);

    println!("{:?}", path.len() - 1);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Square(usize, usize);

fn climbing_path(heightmap: &Vec<Vec<u8>>, start: Square, end: Square) -> Vec<Square> {
    // g = distance from initial Square
    let mut g: HashMap<Square, u32> = HashMap::new();
    g.insert(start, 0);
    // from = which square comes from
    let mut from: HashMap<Square, Square> = HashMap::new();
    let mut open = HashSet::from([start]);

    // h = manhattan distance (heuristic)
    // f = g + h
    let mut f: HashMap<Square, u32> = HashMap::new();
    loop {
        let current = min_distance(&mut open, &f);
        //dbg!(&current, &open, &f);
        if current == end {
            break;
        }

        // frontier squares
        for square in frontier(&heightmap, &current) {
            let score = g.get(&current).unwrap() + 1;
            if score < *g.get(&square).unwrap_or(&u32::MAX) {
                from.insert(square, current);
                g.insert(square, score);
                f.insert(square, score + manhattan_distance(&square, &end));
                open.insert(square);
            }
        }

        if open.is_empty() {
            break;
        }
    }

    to_path(from, end)
}

fn to_path(from: HashMap<Square, Square>, to: Square) -> Vec<Square> {
    let mut path = vec![to];
    let mut current = &to;
    while let Some(next) = from.get(current) {
        path.push(*next);
        current = next;
    }
    path
}

fn frontier(heightmap: &Vec<Vec<u8>>, square: &Square) -> Vec<Square> {
    let (width, height) = (heightmap[0].len(), heightmap.len());
    let mut f = vec![];
    if square.0 > 0 {
        f.push(Square(square.0 - 1, square.1));
    }
    if square.0 < height - 1 {
        f.push(Square(square.0 + 1, square.1));
    }
    if square.1 > 0 {
        f.push(Square(square.0, square.1 - 1));
    }
    if square.1 < width - 1 {
        f.push(Square(square.0, square.1 + 1));
    }

    f.into_iter()
        .filter(|Square(x, y)| heightmap[*x][*y] <= heightmap[square.0][square.1] + 1)
        .collect()
}

fn min_distance(open: &mut HashSet<Square>, g: &HashMap<Square, u32>) -> Square {
    let min = open
        .iter()
        .min_by(|a, b| g.get(a).unwrap().partial_cmp(g.get(b).unwrap()).unwrap())
        .cloned()
        .unwrap();

    open.remove(&min);

    min
}

fn manhattan_distance(from: &Square, to: &Square) -> u32 {
    (from.0.abs_diff(to.0) + from.1.abs_diff(to.1)) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn manhattan_distance_same() {
        assert_eq!(manhattan_distance(&Square(3, 4), &Square(3, 4)), 0)
    }

    #[test]
    fn manhattan_distance_diagonal() {
        assert_eq!(manhattan_distance(&Square(1, 1), &Square(4, 4)), 6)
    }
}
