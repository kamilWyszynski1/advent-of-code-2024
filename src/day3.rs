use regex::Regex;

fn day3a(input: &str) -> isize {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut ans = 0;
    for (full, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        let (l, r): (isize, isize) = (left.parse().unwrap(), right.parse().unwrap());
        ans += l * r
    }
    ans
}

fn day3b(input: &str) -> isize {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|(don\'t)|(do)").unwrap();
    let mut ans = 0;
    let mut should_do = true;
    for f in re.find_iter(input) {
        if f.as_str() == "don't" {
            should_do = false;
            continue;
        } else if f.as_str() == "do" {
            should_do = true;
            continue;
        }
        let mut bind = f.as_str().replace("mul", "").replace("(", "").replace(")", "");
        let (left, right) = bind.split_once(",").unwrap();
        let (l, r): (isize, isize) = (left.parse().unwrap(), right.parse().unwrap());

        if should_do {
            ans += l * r;
        }
    }

    ans
}


#[cfg(test)]
mod tests {
    use crate::day3::{day3a, day3b};
    use crate::read_input;

    #[test]
    fn test_day3a() {
        assert_eq!(161, day3a("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"));

        println!("day3 a result {}", day3a(&read_input("day3")))
    }

    #[test]
    fn test_day3b() {
        assert_eq!(48, day3b("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"));

        println!("day3 b result {}", day3b(&read_input("day3")))
    }
}
