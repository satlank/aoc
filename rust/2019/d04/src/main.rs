fn is_valid(val: i32) -> bool {
    if val < 100_000 { return false; }
    if val > 999_999 { return false; }

    let mut tmp = val;
    let mut prev_digit = tmp % 10;
    let mut repeat_found = false;
    tmp /= 10;
    for _ in 0..5 {
        let digit = tmp % 10;
        if digit > prev_digit {
            return false;
        }
        if digit == prev_digit {
            repeat_found = true;
        }
        prev_digit = digit;

        tmp /= 10;
    }

    return repeat_found;
}

fn is_valid2(val: i32) -> bool {
    if val < 100_000 { return false; }
    if val > 999_999 { return false; }

    let mut tmp = val;
    let mut prev_digit = tmp % 10;
    let mut repeat_found = false;
    let mut repeat_count = 1;
    tmp /= 10;
    for _ in 0..5 {
        let digit = tmp % 10;
        if digit > prev_digit {
            return false;
        }
        if !repeat_found {
            if digit == prev_digit {
                repeat_count +=1;
            } else {
                if repeat_count == 2 {
                    repeat_found = true;
                } else {
                    repeat_count = 1;
                }
            }
        }
        prev_digit = digit;

        tmp /= 10;
    }

    return repeat_found || repeat_count == 2;
}

fn main() {
    let range_start = 264793;
    let range_end = 803935;
    let mut valid = Vec::new();

    for val in range_start..range_end {
        if is_valid(val) {
            valid.push(val);
        }
    }
    let valid2: Vec<i32> = valid.iter().filter(|&v| is_valid2(*v)).cloned().collect();
    println!("Number of valid guesses (part 1): {}", valid.len());
    println!("Number of valid guesses (part 2): {}", valid2.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_length() {
        assert!(is_valid(123455));
        assert!(!is_valid(12344));
        assert!(!is_valid(1234577));
    }

    #[test]
    fn test_part1_ex1() {
        assert!(is_valid(111111));
    }

    #[test]
    fn test_part1_ex2() {
        assert!(!is_valid(223450));
    }

    #[test]
    fn test_part1_ex3() {
        assert!(!is_valid(123789));
    }

    #[test]
    fn test_part2_ex1() {
        assert!(is_valid2(112233));
    }

    #[test]
    fn test_part2_ex2() {
        assert!(!is_valid2(123444));
    }

    #[test]
    fn test_part2_ex3() {
        assert!(is_valid2(111122));
    }

    #[test]
    fn test_part2_err1() {
        assert!(is_valid2(334444));
    }
}
