fn day2a(input: &str) -> isize {
    solve(input, is_report_safe)
}

fn day2b(input: &str) -> isize {
    solve(input, is_report_safe_with_delete)
}

fn solve(input: &str, is_report_safe: fn(v: &Vec<isize>) -> bool) -> isize {
    let values = input.lines().map(|l| l.split(" ").map(|v| v.parse::<isize>().unwrap()).collect::<Vec<isize>>()).collect::<Vec<_>>();

    values.into_iter().map(|v| if is_report_safe(&v) { 1 } else { 0 }).sum()
}


fn is_report_safe(v: &Vec<isize>) -> bool {
    let increasing = if v[0] < v[1] { true } else { false };
    for i in 1..v.len() {
        let dis = (v[i] - v[i - 1]).abs();
        if dis > 3 || dis < 1 { return false; }
        if increasing {
            if v[i - 1] > v[i] { return false; }
        } else {
            if v[i] > v[i - 1] { return false; }
        }
    }
    true
}

fn is_report_safe_with_delete(v: &Vec<isize>) -> bool {
    if is_report_safe(v) { return true; }
    for i in 0..v.len() {
        let mut cloned = v.clone();
        cloned.remove(i);
        if is_report_safe(&cloned) { return true; }
    }
    false
}


#[cfg(test)]
mod tests {
    use crate::day2::{day2a, day2b};
    use crate::read_input;

    #[test]
    fn test_day2a() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(2, day2a(input));

        println!("day2 a result {}", day2a(&read_input("day2")))
    }

    #[test]
    fn test_day2b() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(4, day2b(input));

        println!("day2 b result {}", day2b(&read_input("day2")))
    }
}
