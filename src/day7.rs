fn base(input: &str, check: fn(isize, isize, Vec<isize>) -> bool) -> isize {
    let data: Vec<(isize, Vec<isize>)> = input.lines().map(|line| { line.split_once(':').unwrap() }).map(|(result, s)| (result.parse::<isize>().unwrap(), s.trim().split(' ').map(|v| v.parse::<isize>().unwrap()).collect())).collect();

    let mut ans = 0;
    for (result, numbers) in data {
        if check(result, numbers[0], numbers[1..].to_vec()) {
            ans += result
        }
    }

    ans
}

fn day7a(input: &str) -> isize {
    base(input, check)
}

fn check(result: isize, curr: isize, numbers: Vec<isize>) -> bool {
    if numbers.len() == 0 {
        return result == curr;
    }


    if check(result, curr + numbers[0], numbers[1..].to_owned()) { return true; }
    check(result, curr * numbers[0], numbers[1..].to_owned())
}


fn day7b(input: &str) -> isize {
    base(input, checkb)
}


fn checkb(result: isize, curr: isize, numbers: Vec<isize>) -> bool {
    if numbers.len() == 0 {
        return result == curr;
    }

    if checkb(result, curr + numbers[0], numbers[1..].to_owned()) { return true; }
    if checkb(result, curr * numbers[0], numbers[1..].to_owned()) { return true; }
    checkb(result, concatenation(curr, numbers[0]), numbers[1..].to_owned())
}

fn concatenation(left: isize, right: isize) -> isize {
    let mut s = left.to_string();
    s.push_str(&right.to_string());
    s.parse::<isize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day7a() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        assert_eq!(3749, day7a(input));

        println!("day7a result {}", day7a(&read_input("day7")))
    }

    #[test]
    fn test_day7b() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        assert_eq!(11387, day7b(input));

        println!("day7b result {}", day7b(&read_input("day7")))
    }
}
