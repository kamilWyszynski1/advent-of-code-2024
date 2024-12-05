use std::collections::HashMap;

fn get_update_and_rules(input: &str) -> (Vec<Vec<isize>>, HashMap<isize, Vec<isize>>) {
    let (rules, updates) = input.split_once("

").unwrap();
    let updates: Vec<Vec<isize>> = updates.lines().map(|line| line.split(",").map(|v| v.parse::<isize>().unwrap()).collect()).collect();

    let rls: Vec<(isize, isize)> = rules.lines().map(|rule| rule.split_once("|").unwrap()).map(|(l, r)| (l.parse::<isize>().unwrap(), r.parse::<isize>().unwrap())).collect();
    let r = rls.into_iter().fold(HashMap::new(), |mut acc, (from, to)| {
        acc.entry(from).or_insert(vec![]).push(to);
        acc
    });
    (updates, r)
}

fn day5a(input: &str) -> isize {
    let (updates, r) = get_update_and_rules(input);

    let mut ans = 0;
    for update in updates {
        let mut invalid = false;
        for i in 1..update.len() {
            match r.get(&update[i]) {
                None => { continue }
                Some(value_rules) => {
                    for prev in &update.clone()[0..=i] {
                        if value_rules.contains(prev) {
                            invalid = true;
                            break;
                        }
                    }
                    if invalid { break; }
                }
            }
        }
        if invalid { continue; }
        ans += update[update.len() / 2]
    }

    ans
}

fn day5b(input: &str) -> isize {
    let (updates, r) = get_update_and_rules(input);

    let mut ans = 0;
    for mut update in updates {
        let mut swapped = false;
        for i in 1..update.len() {
            match r.get(&update[i]) {
                None => { continue }
                Some(value_rules) => {
                    for (prev_inx, prev) in update.clone()[0..=i].iter().enumerate() {
                        if value_rules.contains(prev) {
                            update.swap(i, prev_inx);
                            swapped = true
                        }
                    }
                }
            }
        }
        if swapped {
            ans += update[update.len() / 2]
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use crate::day5::{day5a, day5b};
    use crate::read_input;

    #[test]
    fn test_day5a() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        assert_eq!(143, day5a(input));

        println!("day5 b result {}", day5a(&read_input("day5")))
    }

    #[test]
    fn test_day5b() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        assert_eq!(123, day5b(input));

        println!("day5 b result {}", day5b(&read_input("day5")))
    }
}
