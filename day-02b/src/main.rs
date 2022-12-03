/*
--- Day 02: Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to
end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as
 indicated. The example above now goes like this:

In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock.
This gives you a score of 1 + 3 = 4.

In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.

In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to
 your strategy guide?

Answer: 
*/

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Results {
    Won,
    Lost,
    Tied,
}

fn main() {
    let mut results = HashMap::new();
    results.insert("A X".to_string(), ("C", Results::Lost, 3)); // Rock     - Need to Lose - Scissors (0+3)
    results.insert("A Y".to_string(), ("A", Results::Tied, 4)); // Rock     - Need to Draw - Rock (3+1)
    results.insert("A Z".to_string(), ("B", Results::Won,  8)); // Rock     - Need to Win  - Paper (6+2)
    results.insert("B X".to_string(), ("A", Results::Lost, 1)); // Paper    - Need to Lose - Rock (0+1)
    results.insert("B Y".to_string(), ("B", Results::Tied, 5)); // Paper    - Need to Draw - Paper (3+2)
    results.insert("B Z".to_string(), ("C", Results::Won,  9)); // Paper    - Need to Win  - Scissors (6+3)
    results.insert("C X".to_string(), ("B", Results::Lost, 2)); // Scissors - Need to Lose - Paper(0+2)
    results.insert("C Y".to_string(), ("C", Results::Tied, 6)); // Scissors - Need to Draw - Scissors (3+3)
    results.insert("C Z".to_string(), ("A", Results::Won,  7)); // Scissors - Need to Win  - Rock (6+1)

    let mut total_score = 0;

    let reader = BufReader::new(File::open("../data/day-02.txt").unwrap());
    for (_index, raw_line) in reader.lines().enumerate() {
        let line = raw_line.unwrap();
        let scenario = results.get(&line).unwrap();
        let score = scenario.2;
        total_score += score;

        print!("{} - Played {} ", line, scenario.0);
        match scenario.1 {
            Results::Won => print!("won"),
            Results::Lost => print!("lost"),
            Results::Tied => print!("tied"),
        }
        println!(", scored {}", score);
    }
    println!("\nTotal score: {}", total_score)
}
