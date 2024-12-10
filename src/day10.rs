use std::collections::HashSet;

fn day10(input: &str, a: bool) -> isize {
    let input: Vec<Vec<usize>> = input.lines().map(|l| l.chars().map(|c| c as usize - '0' as usize).collect()).collect();

    let mut ans = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 0 {
                let score = walk(&input, i as isize, j as isize, true, 0, &mut HashSet::new(), a);
                ans += score;
            }
        }
    }
    ans
}

fn walk(input: &Vec<Vec<usize>>, i: isize, j: isize, start: bool, prev: usize, reached: &mut HashSet<(isize, isize)>, a: bool) -> isize {
    if i < 0 || i > input.len() as isize - 1 || j < 0 || j > input[0].len() as isize - 1 { return 0; }
    let curr = input[i as usize][j as usize];

    if !start && curr != prev + 1 { return 0; }
    if curr == 9 {
        if !a {
            return 1;
        }
        if reached.contains(&(i, j)) { return 0; }
        reached.insert((i, j));

        return 1;
    }

    walk(&input, i + 1, j, false, curr, reached, a) +
        walk(&input, i - 1, j, false, curr, reached, a) +
        walk(&input, i, j + 1, false, curr, reached, a) +
        walk(&input, i, j - 1, false, curr, reached, a)
}

fn day10b(input: &str) -> isize {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day10a() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

        assert_eq!(36, day10(input, true));

        println!("day10 result {}", day10(&read_input("day10"), true))
    }

    #[test]
    fn test_day10b() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

        assert_eq!(81, day10(input, false));

        println!("day10 result {}", day10(&read_input("day10"), false))
    }
}
