pub fn generator(input: &str) -> Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> {
    input
        .lines()
        .map(|l| {
            let (input, output) = l
                .split_once(" | ")
                .expect("Wrong format. Could not find '|'");
            let input_elements: Vec<Vec<char>> = input
                .split(' ')
                .map(|chars| chars.chars().collect())
                .collect();
            let output_elements: Vec<Vec<char>> = output
                .split(' ')
                .map(|chars| chars.chars().collect())
                .collect();
            (input_elements, output_elements)
        })
        .collect()
}

pub fn part_1(input: &[(Vec<Vec<char>>, Vec<Vec<char>>)]) -> u32 {
    let mut count = 0;
    for data in input {
        let (_, output) = data;
        output.iter().for_each(|o| match o.len() {
            2 | 3 | 4 | 7 => {
                count += 1;
            }
            _ => {}
        });
    }
    count
}

pub fn part_2(input: &[(Vec<Vec<char>>, Vec<Vec<char>>)]) -> u32 {
    let mut sum = 0;
    for data in input {
        let (input, output) = data;

        let mut nine_six_or_zero = input
            .iter()
            .filter(|i| i.len() == 6)
            .cloned()
            .collect::<Vec<Vec<char>>>();

        let mut five_two_or_three = input
            .iter()
            .filter(|i| i.len() == 5)
            .cloned()
            .collect::<Vec<Vec<char>>>();

        // Number one letters
        let one_letters = input.iter().find(|i| i.len() == 2).unwrap();

        // Number four letters
        let four_letters = input.iter().find(|i| i.len() == 4).unwrap();

        // Number six letters
        let mut six_letters = Vec::<char>::with_capacity(6);
        let mut six_index = 0;
        for (i, number) in nine_six_or_zero.iter().enumerate() {
            let matches: Vec<char> = one_letters
                .iter()
                .filter(|&&c| number.contains(&c))
                .cloned()
                .collect();
            if matches.len() == 1 {
                six_letters = number.to_vec();
                six_letters.sort();
                six_index = i;
            }
        }
        nine_six_or_zero.remove(six_index);

        // Number nine letters
        let mut nine_letters = Vec::<char>::with_capacity(6);
        let mut nine_index = 0;
        for (i, number) in nine_six_or_zero.iter().enumerate() {
            let matches: Vec<char> = number
                .iter()
                .filter(|&&c| four_letters.contains(&c))
                .cloned()
                .collect();
            if matches.len() == 4 {
                nine_letters = number.to_vec();
                nine_letters.sort();
                nine_index = i;
            }
        }
        nine_six_or_zero.remove(nine_index);

        // Number two letters
        let mut two_letters = Vec::<char>::with_capacity(5);
        let mut two_index = 0;
        for (i, number) in five_two_or_three.iter().enumerate() {
            let matches: Vec<char> = number
                .iter()
                .filter(|&&c| four_letters.contains(&c))
                .cloned()
                .collect();
            if matches.len() == 2 {
                two_letters = number.to_vec();
                two_letters.sort();
                two_index = i;
            }
        }
        five_two_or_three.remove(two_index);

        // Number five letters
        let mut five_letters = Vec::<char>::with_capacity(5);
        let mut five_index = 0;
        for (i, number) in five_two_or_three.iter().enumerate() {
            let matches: Vec<char> = number
                .iter()
                .filter(|&&c| six_letters.contains(&c))
                .cloned()
                .collect();
            if matches.len() == 5 {
                five_letters = number.to_vec();
                five_letters.sort();
                five_index = i;
            }
        }
        five_two_or_three.remove(five_index);

        let mut number = 0;
        output.iter().for_each(|o| match o.len() {
            7 => number = number * 10 + 8,
            2 => number = number * 10 + 1,
            3 => number = number * 10 + 7,
            4 => number = number * 10 + 4,
            5 => {
                let mut letters = o.clone();
                letters.sort();

                if letters.eq(&five_letters) {
                    number = number * 10 + 5;
                } else if letters.eq(&two_letters) {
                    number = number * 10 + 2;
                } else {
                    number = number * 10 + 3;
                }
            }
            6 => {
                let mut letters = o.clone();
                letters.sort();

                if letters.eq(&nine_letters) {
                    number = number * 10 + 9;
                } else if letters.eq(&six_letters) {
                    number = number * 10 + 6;
                } else {
                    number *= 10;
                }
            }
            _ => panic!(),
        });
        sum += number;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 26);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 61229);
    }
}
