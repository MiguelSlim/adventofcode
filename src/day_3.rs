use std::collections::{HashMap, HashSet};

use crate::read_lines;

pub fn day_3_part_2() {
    let mut result = 0;
    if let Ok(lines) = read_lines("./input2.txt") {
        let mut char_map = HashMap::<char, i32>::new();
        let mut bla = &mut char_map;

        for (i, line) in lines.enumerate() {
            if let Ok(item) = line {
                let mut set = HashSet::<char>::new();

                for c in item.chars() {
                    set.insert(c);
                }

                for c in set {
                    bla.entry(c)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }

                if (i + 1) % 3 == 0 {
                    let badge = bla.into_iter().find(|(c, a)| **a == 3).unwrap();
                    result += get_char_val(*badge.0);
                    bla.clear();
                    continue;
                }
            }
        }
    }

    println!("{:?}", result);
}

fn get_char_val(c: char) -> i32 {
    let is_upper = c.is_uppercase();
    let val = c.to_lowercase().to_string().chars().nth(0).unwrap() as u8 - 96;
    if is_upper {
        (val + 26) as i32
    } else {
        val as i32
    }
}
