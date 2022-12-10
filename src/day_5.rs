use std::collections::{HashMap, VecDeque};

use crate::read_lines;
use regex::Regex;

pub fn day_5_part_2() {
    let mut stacks = get_stacks();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(item) = line {
                let re = Regex::new(r"move [0-9]*").unwrap();
                let mut quantity = if let Some(quantity) = re.captures(&item) {
                    Some((&quantity[0][5..].parse::<i32>().unwrap()).to_owned())
                } else {
                    None
                };
                let re = Regex::new(r"from [0-9]*").unwrap();
                let from_idx = if let Some(from) = re.captures(&item) {
                    Some((&from[0][5..].parse::<i32>().unwrap()).to_owned())
                } else {
                    None
                };
                let re = Regex::new(r"to [0-9]*").unwrap();
                let to_idx = if let Some(to) = re.captures(&item) {
                    Some((&to[0][3..].parse::<i32>().unwrap()).to_owned())
                } else {
                    None
                };

                if quantity.is_some() && from_idx.is_some() && to_idx.is_some() {
                    let mut quantity = quantity.unwrap();
                    let from_idx = from_idx.unwrap() as usize;
                    let to_idx = to_idx.unwrap() as usize;

                    let mut temp = VecDeque::new();
                    while quantity > 0 {
                        let item = stacks.get_mut(&from_idx).unwrap().pop_back().unwrap();
                        temp.push_front(item);

                        quantity -= 1;
                    }

                    for item in temp.into_iter() {
                        stacks.get_mut(&to_idx).unwrap().push_back(item);
                    }
                }
            }
        }
    }

    let mut as_vec = stacks.iter().map(|a| a).collect::<Vec<_>>();
    as_vec.sort_by(|(i_a, _), (i_b, _)| i_a.cmp(&i_b));
    let result = as_vec
        .iter()
        .map(|(_, item)| *item.iter().last().unwrap())
        .collect::<String>();
    println!("{:?}", result);
}

type Stack = VecDeque<char>;
type Stacks = HashMap<usize, Stack>;

fn get_stacks() -> Stacks {
    let mut stacks = HashMap::<usize, Stack>::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(item) = line {
                let chars = item.chars();
                for (i, c) in chars.enumerate() {
                    if ('A'..='Z').any(|x| x == c) {
                        let stack_num = ((i - 1) / 4) + 1;
                        stacks
                            .entry(stack_num)
                            .and_modify(|stack| stack.push_front(c))
                            .or_insert({
                                let mut vecd = VecDeque::new();
                                vecd.push_front(c);
                                vecd
                            });
                    }
                }

                if item == "" {
                    break;
                }
            }
        }
    }

    stacks
}
