use std::collections::HashSet;

use crate::read_lines;

pub fn day_6_part_1() {
    if let Ok(lines) = read_lines("./inputs/day_6_part_1.txt") {
        for line in lines {
            let mut cur_packet = Vec::<char>::with_capacity(4);
            let mut packet_start_index = 0;

            if let Ok(item) = line {
                for c in item.chars() {
                    if cur_packet.len() == 4 {
                        cur_packet.remove(0);
                    }
                    cur_packet.push(c);

                    let set = HashSet::<&char>::from_iter(cur_packet.iter());

                    packet_start_index += 1;
                    if set.len() == 4 {
                        break;
                    }
                }
            }

            println!(
                "The first start-of-packet marker is: {}",
                packet_start_index
            );
        }
    }
}

pub fn day_6_part_2() {
    if let Ok(lines) = read_lines("./inputs/day_6_part_2.txt") {
        for line in lines {
            let mut cur_packet = Vec::<char>::with_capacity(14);
            let mut packet_start_index = 0;

            if let Ok(item) = line {
                for c in item.chars() {
                    if cur_packet.len() == 14 {
                        cur_packet.remove(0);
                    }
                    cur_packet.push(c);

                    let set = HashSet::<&char>::from_iter(cur_packet.iter());

                    packet_start_index += 1;
                    if set.len() == 14 {
                        break;
                    }
                }
            }

            println!(
                "The first start-of-packet marker is: {}",
                packet_start_index
            );
        }
    }
}
