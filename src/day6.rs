use std::collections::HashSet;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn get_move(&self) -> (isize, isize) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
        }
    }

    fn rotate_right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down,
            Dir::Left => Dir::Up,
        }
    }
}


fn day6a(input: &str) -> isize {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut ans = 1; // include starting position

    let mut position = find_start(&map);
    map[position.0 as usize][position.1 as usize] = 'X';
    let mut dir = Dir::Up;

    loop {
        let m = dir.get_move();

        loop {
            let next = (position.0 + m.0, position.1 + m.1);
            if next.0 < 0 || next.1 < 0 || next.0 >= map.len() as isize || next.1 >= map[0].len() as isize {
                print_map(&map);
                return ans;
            }
            if map[next.0 as usize][next.1 as usize] == '#' {
                dir = dir.rotate_right();
                break;
            }
            if map[next.0 as usize][next.1 as usize] != 'X' {
                ans += 1;
                map[next.0 as usize][next.1 as usize] = 'X';
            }
            position = next;
        }
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

fn find_start(map: &Vec<Vec<char>>) -> (isize, isize) {
    for (i, row) in map.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if *v == '^' {
                return (i as isize, j as isize);
            }
        }
    }
    unreachable!()
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Been {
    pos: (isize, isize),
    dir: Dir,
}

fn day6b(input: &str) -> isize {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut ans = 0;

    let traversed = traverse(&map);
    let mut included = HashSet::new();
    let inclen = traversed.len();

    let (n, m) = (map.len() as isize, map[0].len() as isize);
    for (inx, point) in traversed.iter().enumerate() {
        for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)] {
            let (nx, ny) = (point.0 + dx, point.1 + dy);
            if 0 <= nx && nx < n && 0 <= ny && ny < m {
                if map[nx as usize][ny as usize] != '^' {
                    let tmp = map[nx as usize][ny as usize];
                    map[nx as usize][ny as usize] = '#';
                    if is_infinite_loop(&map) && !included.contains(&(nx as usize, ny as usize)) {
                        ans += 1;
                        included.insert((nx as usize, ny as usize));
                    }
                    map[nx as usize][ny as usize] = tmp;
                }
            }
        }
    }

    ans
}

fn is_infinite_loop(map: &Vec<Vec<char>>) -> bool {
    let mut beens = HashSet::new();

    let mut position = find_start(&map);
    let mut dir = Dir::Up;
    beens.insert(Been { pos: position, dir });

    loop {
        let m = dir.get_move();

        loop {
            let next = (position.0 + m.0, position.1 + m.1);
            if next.0 < 0 || next.1 < 0 || next.0 >= map.len() as isize || next.1 >= map[0].len() as isize {
                return false;
            }
            if beens.contains(&Been { pos: next, dir }) {
                return true;
            }
            if map[next.0 as usize][next.1 as usize] == '#' {
                dir = dir.rotate_right();
                break;
            }
            beens.insert(Been { pos: next, dir });
            position = next;
        }
    }
    false
}

fn traverse(map: &Vec<Vec<char>>) -> HashSet<(isize, isize)> {
    let mut beens = HashSet::new();

    let mut position = find_start(&map);
    let mut dir = Dir::Up;
    beens.insert(position);

    loop {
        let m = dir.get_move();

        loop {
            let next = (position.0 + m.0, position.1 + m.1);
            if next.0 < 0 || next.1 < 0 || next.0 >= map.len() as isize || next.1 >= map[0].len() as isize {
                return beens;
            }
            if map[next.0 as usize][next.1 as usize] == '#' {
                dir = dir.rotate_right();
                break;
            }

            beens.insert(position);
            position = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::{day6a, day6b};
    use crate::read_input;

    #[test]
    fn test_day6a() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        assert_eq!(41, day6a(input));

        println!("daya b result {}", day6a(&read_input("day6")))
    }

    #[test]
    fn test_day6b() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        assert_eq!(6, day6b(input));

        println!("day6 b result {}", day6b(&read_input("day6")))
    }
}
