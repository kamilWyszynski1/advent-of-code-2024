fn day4a(input: &str) -> isize {
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (m, n) = (matrix.len(), matrix[0].len());


    // left -> right
    let mut ans = 0;
    for i in 0..m {
        for j in 0..=n - 4 {
            if check(&matrix[i][j..j + 4]) { ans += 1 }
        }
    }

    // up -> down
    for i in 0..=m - 4 {
        for j in 0..n {
            if check(&[matrix[i][j], matrix[i + 1][j], matrix[i + 2][j], matrix[i + 3][j]]) { ans += 1 }
        }
    }

    // top left -> bottom right
    for i in 0..=m - 4 {
        for j in 0..=n - 4 {
            if check(&[matrix[i][j], matrix[i + 1][j + 1], matrix[i + 2][j + 2], matrix[i + 3][j + 3]]) { ans += 1 }
        }
    }

    // bottom left -> top right
    for i in 0..=m - 4 {
        for j in 0..=n - 4 {
            if check(&[matrix[i + 3][j], matrix[i + 2][j + 1], matrix[i + 1][j + 2], matrix[i][j + 3]]) { ans += 1 }
        }
    }

    ans
}

fn check(subv: &[char]) -> bool {
    assert_eq!(subv.len(), 4);
    (subv[0] == 'X' && subv[1] == 'M' && subv[2] == 'A' && subv[3] == 'S') || (subv[0] == 'S' && subv[1] == 'A' && subv[2] == 'M' && subv[3] == 'X')
}

fn day4b(input: &str) -> isize {
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (m, n) = (matrix.len(), matrix[0].len());

    let mut ans = 0;
    for i in 0..=m - 3 {
        for j in 0..=n - 3 {
            let mut subm = Vec::new();
            subm.push(matrix[i][j..j + 3].to_vec());
            subm.push(matrix[i + 1][j..j + 3].to_vec());
            subm.push(matrix[i + 2][j..j + 3].to_vec());

            if check_b(subm) {
                ans += 1;
            }
        }
    }
    ans
}

fn check_b(subm: Vec<Vec<char>>) -> bool {
    fn ch(subv: &[char]) -> bool {
        assert_eq!(subv.len(), 3);
        (subv[0] == 'M' && subv[1] == 'A' && subv[2] == 'S') || (subv[0] == 'S' && subv[1] == 'A' && subv[2] == 'M')
    }

    assert_eq!(subm.len(), 3);
    assert_eq!(subm[0].len(), 3);

    ch(&[subm[0][0], subm[1][1], subm[2][2]]) && ch(&[subm[2][0], subm[1][1], subm[0][2]])
}

#[cfg(test)]
mod tests {
    use crate::day4::{day4a, day4b};
    use crate::read_input;

    #[test]
    fn test_day4a() {
        assert_eq!(18, day4a(r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#));

        println!("day4 a result {}", day4a(&read_input("day4")))
    }

    #[test]
    fn test_day4b() {
        assert_eq!(9, day4b(r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#));

        println!("day4 b result {}", day4b(&read_input("day4")))
    }
}
