use std::collections::HashMap;

fn day1a(input: &str) -> isize {
    let a: Vec<(String, String)> = input
        .lines()
        .map(|line| {
            let mut s = line.split_ascii_whitespace();
            (s.next().unwrap().to_string(), s.next().unwrap().to_string())
        })
        .collect();

    let (mut left, mut right): (Vec<isize>, Vec<isize>) = a
        .into_iter()
        .map(|(a, b)| (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap()))
        .unzip();

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn day1b(input: &str) -> isize {
    let a: Vec<(String, String)> = input
        .lines()
        .map(|line| {
            let mut s = line.split_ascii_whitespace();
            (s.next().unwrap().to_string(), s.next().unwrap().to_string())
        })
        .collect();

    let (left, right): (Vec<isize>, Vec<isize>) = a
        .into_iter()
        .map(|(a, b)| (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap()))
        .unzip();

    let right_occurrences: HashMap<isize, isize> =
        right.into_iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    left.into_iter().fold(0, |acc, c| {
        acc + (right_occurrences.get(&c).unwrap_or(&0) * c)
    })
}

#[cfg(test)]
mod tests {
    use super::day1a;
    use crate::{day1::day1b, read_input};

    #[test]
    fn test_day1a() {
        let input = r#"3   4
        4   3
        2   5
        1   3
        3   9
        3   3"#;

        assert_eq!(11, day1a(input));

        println!("day1 a result {}", day1a(&read_input("day1")))
    }

    #[test]
    fn test_day1b() {
        let input = r#"3   4
        4   3
        2   5
        1   3
        3   9
        3   3"#;

        assert_eq!(31, day1b(input));

        println!("day1 b result {}", day1b(&read_input("day1")))
    }
}
