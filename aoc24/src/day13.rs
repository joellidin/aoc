use std::{error::Error, fmt, num::ParseFloatError, str::FromStr};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
pub enum ParseLineError {
    UnknownLineType,
    InvalidFormat(String),
    DuplicateDefinition(&'static str),
    MissingComponent(&'static str),
    ParseFloatError(ParseFloatError),
}

impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseLineError::UnknownLineType => write!(f, "Unknown line type"),
            ParseLineError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ParseLineError::DuplicateDefinition(comp) => {
                write!(f, "Duplicate definition: {}", comp)
            }
            ParseLineError::MissingComponent(comp) => write!(f, "Missing component: {}", comp),
            ParseLineError::ParseFloatError(e) => write!(f, "Parse float error: {}", e),
        }
    }
}

impl Error for ParseLineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseLineError::ParseFloatError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParseFloatError> for ParseLineError {
    fn from(error: ParseFloatError) -> Self {
        ParseLineError::ParseFloatError(error)
    }
}

#[derive(Debug)]
enum LineType {
    ButtonA(Point),
    ButtonB(Point),
    Prize(Point),
}

#[derive(Debug, Copy, Clone)]
pub struct SectionData {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

impl FromStr for LineType {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(button_a_str) = s.strip_prefix("Button A: X+") {
            let (x_str, y_str) =
                button_a_str
                    .split_once(", Y+")
                    .ok_or(ParseLineError::InvalidFormat(
                        "Invalid format for Button A".to_string(),
                    ))?;
            let x = x_str.trim().parse()?;
            let y = y_str.trim().parse()?;
            return Ok(LineType::ButtonA(Point { x, y }));
        }

        if let Some(button_b_str) = s.strip_prefix("Button B: X+") {
            let (x_str, y_str) =
                button_b_str
                    .split_once(", Y+")
                    .ok_or(ParseLineError::InvalidFormat(
                        "Invalid format for Button B".to_string(),
                    ))?;
            let x = x_str.trim().parse()?;
            let y = y_str.trim().parse()?;
            return Ok(LineType::ButtonB(Point { x, y }));
        }

        if let Some(prize_str) = s.strip_prefix("Prize: X=") {
            let (x_str, y_str) =
                prize_str
                    .split_once(", Y=")
                    .ok_or(ParseLineError::InvalidFormat(
                        "Invalid format for Prize".to_string(),
                    ))?;
            let x = x_str.trim().parse()?;
            let y = y_str.trim().parse()?;
            return Ok(LineType::Prize(Point { x, y }));
        }

        Err(ParseLineError::UnknownLineType)
    }
}

pub fn generator(input: &str) -> Result<Vec<SectionData>, ParseLineError> {
    input
        .split("\n\n")
        .map(|section| {
            let mut button_a = None;
            let mut button_b = None;
            let mut prize = None;
            for line in section.lines().take(3) {
                let parsed_line = line.parse()?;
                match parsed_line {
                    LineType::ButtonA(p) => {
                        if button_a.is_some() {
                            return Err(ParseLineError::DuplicateDefinition("Button A"));
                        }
                        button_a = Some(p);
                    }
                    LineType::ButtonB(p) => {
                        if button_b.is_some() {
                            return Err(ParseLineError::DuplicateDefinition("Button B"));
                        }
                        button_b = Some(p);
                    }
                    LineType::Prize(p) => {
                        if prize.is_some() {
                            return Err(ParseLineError::DuplicateDefinition("Prize"));
                        }
                        prize = Some(p);
                    }
                }
            }

            Ok(SectionData {
                button_a: button_a.ok_or(ParseLineError::MissingComponent("Button A"))?,
                button_b: button_b.ok_or(ParseLineError::MissingComponent("Button B"))?,
                prize: prize.ok_or(ParseLineError::MissingComponent("Prize"))?,
            })
        })
        .collect()
}

fn math(button_a: &Point, button_b: &Point, prize: &Point) -> Option<u64> {
    let m_numerator = prize.y - (prize.x * button_a.y) / button_a.x;
    let m_denominator = button_b.y - (button_b.x * button_a.y) / button_a.x;

    let m = m_numerator / m_denominator;
    if m < 0. || (m.round() - m).abs() > 1e-3 {
        return None;
    }
    let n = (prize.x - m * button_b.x) / button_a.x;
    if n < 0. || (m.round() - m).abs() > 1e-3 {
        return None;
    }
    Some(3 * n.round() as u64 + m.round() as u64)
}

pub fn part_1(input: &Result<Vec<SectionData>, ParseLineError>) -> u64 {
    input
        .as_ref()
        .expect("Invalid input")
        .iter()
        .filter_map(|play| math(&play.button_a, &play.button_b, &play.prize))
        .sum()
}

pub fn part_2(input: &Result<Vec<SectionData>, ParseLineError>) -> u64 {
    input
        .as_ref()
        .expect("Invalid input")
        .iter()
        .filter_map(move |play| {
            let conversion_error = 10_000_000_000_000.0;
            let section = SectionData {
                prize: Point {
                    x: play.prize.x + conversion_error,
                    y: play.prize.y + conversion_error,
                },
                ..*play
            };
            math(&section.button_a, &section.button_b, &section.prize)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 480);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 875318608908);
    }
}
