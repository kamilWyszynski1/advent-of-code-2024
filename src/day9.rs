use std::cmp::PartialEq;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
enum Value {
    Empty(isize),
    Val {
        inx: isize,
        amount: isize,
    },
}

impl Value {
    fn is_empty(&self) -> bool {
        match self {
            Value::Empty(_) => { true }
            Value::Val { .. } => { false }
        }
    }
}


fn day9a(input: &str) -> isize {
    let input = input.chars().map(|c| c as isize - '0' as isize).collect::<Vec<isize>>();

    let mut input: Vec<Value> = input.iter().enumerate().map(|(i, v)| {
        if i % 2 == 0 {
            Value::Val {
                inx: (i / 2) as isize,
                amount: *v,
            }
        } else {
            Value::Empty(*v)
        }
    }).collect();


    let mut left = input[1].clone();
    let mut left_index = 1;
    let (mut right, mut right_index) = if !input[input.len() - 1].is_empty() {
        (input[input.len() - 1].clone(), input.len() - 1)
    } else {
        (input[input.len() - 2].clone(), input.len() - 2)
    };


    while left_index < right_index {
        match (&left, &right) {
            (Value::Empty(empty_size), Value::Val { inx, amount }) => {
                if *amount > *empty_size {
                    input[left_index] = Value::Val {
                        inx: *inx,
                        amount: *empty_size,
                    };
                    input[right_index] = Value::Val {
                        inx: *inx,
                        amount: *amount - *empty_size,
                    };
                    left_index += 2;
                    right = input[right_index].clone();
                    left = input[left_index].clone();
                } else if *amount < *empty_size {
                    input[right_index] = Value::Empty(0);
                    input.insert(left_index, Value::Val {
                        inx: *inx,
                        amount: *amount,
                    });
                    left_index += 1;
                    input[left_index] = Value::Empty(*empty_size - *amount);
                    right_index -= 1;
                    right = input[right_index].clone();
                    left = input[left_index].clone()
                } else {
                    input[left_index] = Value::Val {
                        inx: *inx,
                        amount: *amount,
                    };
                    input[right_index] = Value::Empty(0);
                    right_index -= 2;
                    right = input[right_index].clone();
                    left_index += 2;
                    left = input[left_index].clone();
                }
            }
            _ => {
                println!("invalid: {:?}, {:?}", left, right);
                panic!("invalid")
            }
        }
    }


    let mut ans = 0;
    let mut i = 0;
    let mut curr_inx = 0;
    while curr_inx < input.len() {
        let curr = &input[curr_inx];
        match curr {
            Value::Empty(_) => { continue }
            Value::Val { inx, amount } => {
                for _ in 0..*amount {
                    ans += inx * i;
                    i += 1;
                }
            }
        }
        curr_inx += 1;
    }


    ans
}


fn find_empty(v: &Vec<Value>, max: usize) -> Option<(usize, Value)> {
    let mut inx = 0;
    while inx < max {
        if v[inx].is_empty() {
            return Some((inx, v[inx].clone()));
        }
        inx += 1;
    }
    None
}

fn find_next_empty(v: &Vec<Value>, curr: usize, max: usize, amount: isize) -> Option<(usize, &Value)> {
    for i in curr + 1..max {
        if let Value::Empty(empty) = v[i] {
            if empty >= amount {
                return Some((i, &v[i]));
            }
        }
    }
    None
}

fn find_last_non_empty(v: &Vec<Value>, x: &HashSet<isize>) -> Option<(usize, Value)> {
    for i in (0..v.len()).rev() {
        if let Value::Val { inx, amount } = v[i] {
            if !x.contains(&inx) {
                return Some((i, v[i].clone()));
            }
        }
    }
    None
}

fn print_arr(v: &Vec<Value>) {
    for i in v {
        match i {
            Value::Empty(size) => {
                for _ in 0..*size {
                    print!(".")
                }
            }
            Value::Val { inx, amount } => {
                for _ in 0..*amount {
                    print!("{}", *inx)
                }
            }
        }
    }
    println!();
}


fn day9b(input: &str) -> isize {
    let input = input.chars().map(|c| c as isize - '0' as isize).collect::<Vec<isize>>();

    let mut input: Vec<Value> = input.iter().enumerate().map(|(i, v)| {
        if i % 2 == 0 {
            Value::Val {
                inx: (i / 2) as isize,
                amount: *v,
            }
        } else {
            Value::Empty(*v)
        }
    }).collect();


    let mut curr_empty_index = 0;
    let mut moved = HashSet::new();
    loop {
        let (right_index, right) = match find_last_non_empty(&input, &moved) {
            None => { break }
            Some(v) => { v }
        };
        if let Value::Val { inx, amount } = right {
            // find first on the left
            match find_next_empty(&input, curr_empty_index, right_index, amount) {
                None => {
                    curr_empty_index = 0;
                    moved.insert(inx);
                }
                Some((left_index, left)) => {
                    if let Value::Empty(empty_space) = left {
                        if *empty_space < amount {
                            // move to the next one
                            curr_empty_index = left_index;
                            continue;
                        } else if *empty_space > amount {
                            input[left_index] = Value::Empty(empty_space - amount);
                            input[right_index] = Value::Empty(amount);
                            input.insert(left_index, Value::Val {
                                inx,
                                amount,
                            });
                            curr_empty_index = 0;
                            moved.insert(inx);
                        } else {
                            // empty_space == amount
                            input[left_index] = Value::Val {
                                inx,
                                amount,
                            };
                            input[right_index] = Value::Empty(amount);
                            curr_empty_index = 0;
                            moved.insert(inx);
                        }
                    } else {
                        panic!("should be empty")
                    }
                }
            }
        } else {
            panic!("should be Val")
        }
    }

    println!("done");

    let mut ans = 0;
    let mut i = 0;
    let mut curr_inx = 0;
    loop {
        if curr_inx >= input.len() { break; }
        let curr = &input[curr_inx];
        if i == 40 {
            println!("{curr_inx}, {i}, {:?}", curr);
        }
        match curr {
            Value::Empty(amount) => { i += amount }
            Value::Val { inx, amount } => {
                for _ in 0..*amount {
                    ans += inx * i;
                    i += 1;
                }
            }
        }
        curr_inx += 1;
    }


    ans
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day9a() {
        let input = r#"2333133121414131402"#;

        assert_eq!(1928, day9a(input));

        println!("day9a result {}", day9a(&read_input("day9")))
    }

    #[test]
    fn test_day9b() {
        let input = r#"2333133121414131402"#;

        assert_eq!(2858, day9b(input));

        println!("day9b result {}", day9b(&read_input("day9")))
    }
}
