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
            echo "pub fn generator(input: &str) -> &str {"
            echo "    todo!();"
            echo "}"
            echo ""
            echo "pub fn part_1(input: &str) -> u32 {"
            echo "    todo!();"
            echo "}"
            echo ""
            echo "pub fn part_2(input: &str) -> u32 {"
            echo "    todo!();"
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
