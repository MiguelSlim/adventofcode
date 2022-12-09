use crate::read_lines;

fn day_4() {
    let mut result = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(item) = line {
                let mut ranges = item.split(",");
                let (r_1_1, r_1_2) = get_range(ranges.nth(0).unwrap());
                let (r_2_1, r_2_2) = get_range(ranges.nth(0).unwrap());

                if (r_1_1 >= r_2_1 && r_1_1 <= r_2_2) || (r_2_1 >= r_1_1 && r_2_1 <= r_1_2) {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
}

fn get_range(range_str: &str) -> (i32, i32) {
    let nums = range_str
        .split("-")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    (nums[0], nums[1])
}
