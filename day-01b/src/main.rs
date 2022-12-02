/*
--- Day1: Part Two ---

By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories
of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves
carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories),
then the fifth Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("../data/day-01.txt").unwrap());
    let mut item_count: u32 = 1;
    let mut elf_count: u32 = 1;
    let mut total: u32 = 0;
    let mut elves = Vec::<(u32, u32)>::new();

    for (_index, raw_line) in reader.lines().enumerate() {
        let line = raw_line.unwrap();

        if line.is_empty() {
            println!("Elf {:03} total: {}\n", elf_count, total);
            elves.push((elf_count, total));
            elf_count += 1;
            total = 0;
            item_count = 1;
            continue;
        }

        match line.parse::<u32>() {
            Ok(calories) => {
                total += calories;
                println!("\tItem {:02}: {}", item_count, calories);
                item_count += 1;
            }
            Err(e) => panic!("{:?}", e),
        }
    }
    println!("Elf {:03} total: {}\n", elf_count, total);
    elves.push((elf_count, total));

    elves.sort_by(|a, b| {
        let a_cal = a.1;
        let b_cal = b.1;
        a_cal.cmp(&b_cal)
    });

    let mut grand_total = 0;
    println!("The top 3 elves are:");
    ((elves.len() - 3)..elves.len()).for_each(|i| {
        println!("\tElf {:03} has {} calories", elves[i].0, elves[i].1);
        grand_total += elves[i].1;
    });
    println!("With a total of {} calories", grand_total);
}
