use std::collections::HashMap;

fn day11a(input: &str) -> isize {
    let mut input: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();

    let mut ans = 0;

    for i in 0..25 {
        blink(&mut input);
    }

    input.len() as isize
}

fn blink(input: &mut Vec<String>) {
    let mut to_insert = vec![];
    for (inx, s) in input.iter_mut().enumerate() {
        if *s == "0" {
            *s = "1".to_string();
        } else if s.len() % 2 == 0 {
            to_insert.push((inx, s[0..s.len() / 2].to_string()));

            let mut trimmed: String = s[s.len() / 2..s.len()].trim_start_matches(|c| c == '0').to_string();
            if trimmed.len() == 0 {
                trimmed = "0".to_string();
            }
            *s = trimmed;
        } else {
            let mut v: isize = s.parse().unwrap();
            v *= 2024;
            *s = v.to_string();
        }
    }


    for (inx, s) in to_insert.iter().rev() {
        input.insert(*inx, s.to_string());
    }
}

fn blinkb(input: String, max: isize, i: isize, memo: &mut HashMap<String, HashMap<isize, isize>>) -> isize {
    if let Some(m) = memo.get(&input) {
        if let Some(v) = m.get(&(max - i)) {
            return *v;
        }
    }
    if max == i {
        return 1;
    }
    if input == "0" {
        let ans = blinkb("1".to_string(), max, i + 1, memo);
        memo.entry(input).or_insert(HashMap::new()).insert(max - i, ans);
        ans
    } else if input.len() % 2 == 0 {
        let (left, mut right) = input.split_at(input.len() / 2);
        right = right.trim_start_matches(|c| c == '0');
        if right.len() == 0 {
            right = "0";
        }

        let ansl = blinkb(left.to_string(), max, i + 1, memo);
        let ansr = blinkb(right.to_string(), max, i + 1, memo);

        memo.entry(input).or_insert(HashMap::new()).insert(max - i, ansl + ansr);

        return ansl + ansr;
    } else {
        let mut v: isize = input.parse().unwrap();
        v *= 2024;

        let ans = blinkb(v.to_string(), max, i + 1, memo);
        memo.entry(input).or_insert(HashMap::new()).insert(max - i, ans);

        return ans;
    }
}


fn day11b(input: &str) -> isize {
    let mut input: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();

    let mut ans = 0;
    let mut memo: HashMap<String, HashMap<isize, isize>> = HashMap::new(); // str -> (iteration, number of elements)

    let max = 75;

    for el in input {
        ans += blinkb(el, max, 0, &mut memo);
    }

    ans
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;


    #[test]
    fn test_day11a() {
        let input = r#"125 17"#;

        assert_eq!(55312, day11a(input));

        println!("day11 result {}", day11a(&read_input("day11")))
    }

    #[test]
    fn test_day11b() {
        let input = r#"125 17"#;

        assert_eq!(65601038650482, day11b(input));

        println!("day11 result {}", day11b(&read_input("day11")))
    }
}
