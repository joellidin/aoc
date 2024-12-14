/// Extracts integers from a string.
///
/// This function scans the input string for integer-like sequences and returns
/// them as a vector of type `T`, which must implement `TryFrom<i64>`.
/// Negative values are parsed if supported by `T`. If `T` is an unsigned type
/// and can't represent a negative value, it tries to convert the number to its
/// absolute value.
///
/// # Type Constraints
/// - `T` must implement `TryFrom<i64>` and `Debug`.
///
/// # Behavior with Unsigned Types
/// For unsigned types (e.g., `u32`), negative numbers are converted to their
/// absolute values. If the conversion fails (e.g., the number is too large),
/// the number is skipped.
///
/// # Examples
/// ```
/// # use aoc_utils::parsing::extract_integers;
/// let result: Vec<i32> = extract_integers("hello12, -12, world 32");
/// assert_eq!(result, vec![12, -12, 32]);
///
/// let result: Vec<u32> = extract_integers("hello12, -12, world 32");
/// assert_eq!(result, vec![12, 12, 32]);
///
/// let result: Vec<isize> = extract_integers("1:2,-12,world 32ListOfnew3");
/// assert_eq!(result, vec![1, 2, -12, 32, 3]);
/// ```
pub fn extract_integers<T>(input: &str) -> Vec<T>
where
    T: TryFrom<i64> + std::fmt::Debug,
    <T as TryFrom<i64>>::Error: std::fmt::Debug,
{
    let mut numbers = Vec::new();
    let mut current_number = String::new();
    let mut is_negative = false;

    // A closure to process and push the accumulated number to the vector
    let mut push_number = |current_number: &mut String, is_negative: &mut bool| {
        if current_number.is_empty() {
            return;
        }

        if let Ok(parsed_num) = current_number.parse::<i64>() {
            let final_num = if *is_negative {
                -parsed_num
            } else {
                parsed_num
            };
            // Try direct conversion
            match T::try_from(final_num) {
                Ok(value) => numbers.push(value),
                Err(_) => {
                    // If direct conversion fails, try absolute value
                    if let Ok(value) = T::try_from(final_num.abs()) {
                        numbers.push(value);
                    }
                }
            }
        }

        current_number.clear();
        *is_negative = false;
    };

    for c in input.chars() {
        if c == '-' && current_number.is_empty() {
            // Handle potential negative sign
            is_negative = true;
        } else if c.is_ascii_digit() {
            // Accumulate digits
            current_number.push(c);
        } else {
            // Non-digit encountered, finalize the number if any
            push_number(&mut current_number, &mut is_negative);
        }
    }

    // Push the last number if it exists
    push_number(&mut current_number, &mut is_negative);

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_integers_i32() {
        let result = extract_integers::<i32>("hello12, -12, world 32");
        assert_eq!(result, vec![12, -12, 32]);
    }

    #[test]
    fn test_extract_integers_u32() {
        let result = extract_integers::<u32>("hello12, -12, world 32");
        assert_eq!(result, vec![12, 12, 32]);
    }

    #[test]
    fn test_extract_integers_isize() {
        let result = extract_integers::<isize>("1:2,-12,world 32ListOfnew3");
        assert_eq!(result, vec![1, 2, -12, 32, 3]);
    }

    #[test]
    fn test_empty_string() {
        let result = extract_integers::<i32>("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_only_negative_signs() {
        let result = extract_integers::<i32>("----");
        assert!(result.is_empty());
    }

    #[test]
    fn test_mixed_with_symbols() {
        let result = extract_integers::<i32>("abc -42! 100? -200#300");
        assert_eq!(result, vec![-42, 100, -200, 300]);
    }

    #[test]
    fn test_large_numbers() {
        let result = extract_integers::<i64>("9999999999 -1000000000 0");
        assert_eq!(result, vec![9999999999, -1000000000, 0]);
    }

    #[test]
    fn test_all_negative_i64() {
        let result = extract_integers::<i64>("-1 -2 -3 -4");
        assert_eq!(result, vec![-1, -2, -3, -4]);
    }
}
