use std::collections::{HashMap, HashSet};

fn day8a(input: &str) -> isize {
    // split into 2d array of characters
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let m = map.len();
    let n = map[0].len();

    let mut antenats: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if *v != '.' {
                antenats.entry(*v).or_insert_with(Vec::new).push((i, j));
            }
        }
    }

    let mut locations = HashSet::new();

    for (ch, positions) in antenats {
        for (inx, (i, j)) in positions.iter().enumerate() {
            for (inx2, (x, y)) in positions.iter().enumerate() {
                if inx == inx2 { continue; }

                let x = *x as isize;
                let y = *y as isize;


                let di = x - (*i as isize);
                let dj = y - (*j as isize);

                let ni = x + di;
                let nj = y + dj;

                let ni: usize = match ni.try_into() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let nj: usize = match nj.try_into() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if ni < m && nj < n {
                    locations.insert((ni, nj));
                }
            }
        }
    }


    locations.len() as isize
}


fn day8b(input: &str) -> isize {
    // split into 2d array of characters
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let m = map.len();
    let n = map[0].len();

    let mut antenats: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if *v != '.' {
                antenats.entry(*v).or_insert_with(Vec::new).push((i, j));
            }
        }
    }

    let mut locations = HashSet::new();

    for (ch, positions) in antenats {
        for (inx, (i, j)) in positions.iter().enumerate() {
            for (inx2, (x, y)) in positions.iter().enumerate() {
                if inx == inx2 { continue; }
                locations.insert((*x, *y));

                let x = *x as isize;
                let y = *y as isize;


                let di = x - (*i as isize);
                let dj = y - (*j as isize);

                let mut mult = 1;
                loop {
                    let ni = x + di * mult;
                    let nj = y + dj * mult;

                    let ni: usize = match ni.try_into() {
                        Ok(v) => v,
                        Err(_) => break,
                    };
                    let nj: usize = match nj.try_into() {
                        Ok(v) => v,
                        Err(_) => break,
                    };
                    if ni < m && nj < n {
                        locations.insert((ni, nj));
                    } else {
                        break;
                    }
                    mult += 1;
                }
            }
        }
    }


    locations.len() as isize
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day8a() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        assert_eq!(14, day8a(input));

        println!("day8a result {}", day8a(&read_input("day8")))
    }

    #[test]
    fn test_day8b() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        assert_eq!(34, day8b(input));

        println!("day8b result {}", day8b(&read_input("day8")))
    }
}
