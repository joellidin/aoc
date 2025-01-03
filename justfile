set shell := ["sh", "-cu"]

create-day YEAR DAY:
    #!/usr/bin/env bash
    set -euo pipefail
    FILENAME="aoc{{YEAR}}/src/day{{DAY}}.rs"
    MAINFILE="aoc{{YEAR}}/src/main.rs"
    echo "Creating Rust file for year {{YEAR}}, day {{DAY}} at $FILENAME"
    
    if [ ! -f $FILENAME ]; then
        mkdir -p aoc{{YEAR}}/src
        {
            echo "use aoc_utils::prelude::*;"
            echo ""
            echo "pub fn generator(input: &str) -> Vec<Vec2<i32>> {"
            echo "    input"
            echo "        .lines()"
            echo "        .map(|line| {"
            echo "            let &[x, y, ..] = extract_integers::<i32>(line).as_slice() else {"
            echo "                panic!(\"Could not parse integers\")"
            echo "            };"
            echo "            (x, y).into()"
            echo "        })"
            echo "        .collect()"
            echo "}"
            echo ""
            echo "pub fn part_1(input: &[Vec2<i32>]) -> u32 {"
            echo "    todo!();"
            echo "}"
            echo ""
            echo "pub fn part_2(input: &[Vec2<i32>]) -> u32 {"
            echo "    todo!();"
            echo "}"
            echo ""
            echo "#[cfg(test)]"
            echo "mod tests {"
            echo "    use super::*;"
            echo ""
            echo "    const INPUT: &str = r#\""
            echo "\"#;"
            echo ""
            echo "    #[test]"
            echo "    fn part_1_test() {"
            echo "        let generator_output = generator(INPUT);"
            echo "        let result = part_1(&generator_output);"
            echo "        assert_eq!(result, 7);"
            echo "    }"
            echo ""
            echo "    #[test]"
            echo "    fn part_2_test() {"
            echo "        let generator_output = generator(INPUT);"
            echo "        let result = part_2(&generator_output);"
            echo "        assert_eq!(result, 5);"
            echo "    }"
            echo "}"

        } > $FILENAME
        echo "Created $FILENAME"

        # Insert 'mod dayXX;' after the second empty line
        awk 'BEGIN{empty=0} /^$/{empty++} empty==2 && !printed && NF==0{print "mod day{{DAY}};"; printed=1} {print}' aoc{{YEAR}}/src/main.rs > tmpfile && mv -f tmpfile aoc{{YEAR}}/src/main.rs

    
        # Insert 'dayXX : generator => part_1, part_2;' before the last line of the macro in main.rs
        sed -i '$i \ \ \ \ day{{DAY}} : generator => part_1, part_2;' $MAINFILE
    
        echo "Updated $MAINFILE with day {{DAY}}."
    else
        echo "$FILENAME already exists."
    fi
