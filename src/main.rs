use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

use day_1::day_1_part_2;
use day_2::day_2_part_2;
use day_3::day_3_part_2;
use day_4::day_4_part_2;
use day_5::day_5_part_2;
use day_6::{day_6_part_1, day_6_part_2};

fn main() {}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
