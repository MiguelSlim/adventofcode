use crate::read_lines;

pub fn day_2_part_2() {
    let arr_opp = ["A", "B", "C"];
    let arr_me = ["X", "Y", "Z"];

    let mut total_score = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(item) = line {
                if item == "" {
                    continue;
                }
                let vals = item.split(" ").collect::<Vec<&str>>();
                let opp = *vals.get(0).unwrap();
                let outcome = *vals.get(1).unwrap();

                let idx_opp = arr_opp.iter().position(|x| *x == opp).unwrap();

                let idx_me = if outcome == "X" {
                    (idx_opp + 2) % 3
                } else if outcome == "Z" {
                    (idx_opp + 1) % 3
                } else {
                    idx_opp
                };

                total_score += if idx_me == (idx_opp + 1) % 3 {
                    6
                } else if idx_me == (idx_opp + 2) % 3 {
                    0
                } else {
                    3
                } + (idx_me + 1);
            }
        }
    }

    println!("{:?}", total_score);
}
