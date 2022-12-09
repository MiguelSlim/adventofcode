use crate::read_lines;

fn day_1() {
    let mut elfs = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        let mut cur = 0;
        for line in lines {
            if let Ok(item) = line {
                if item != "" {
                    cur += item.parse::<i32>().unwrap();
                } else {
                    elfs.push(cur);
                    cur = 0;
                }
            }
        }
    }

    elfs.sort();
    println!("{:?}", &elfs[elfs.len() - 3..].iter().sum::<i32>());
}
