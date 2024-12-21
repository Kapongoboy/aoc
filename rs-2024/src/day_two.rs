fn part_one(input: &str) -> i32 {
    let mut num_safe = 0;

    for line in input.lines() {
        let mut safe = true;
        let mut increasing = true;

        println!("line = {}", line);

        let numbers = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());

        let mut prev = 0;

        for (i, number) in numbers.enumerate() {
            println!("i = {} prev = {} num = {}", i, prev, number);

            if i == 0 {
                prev = number;
                continue;
            }

            if prev > number && i == 1 {
                increasing = false;
                println!("decreasing");
            }

            if !increasing && prev < number {
                println!("increasing when decreasing, set to unsafe");
                safe = false;
                break;
            }

            let sign = if increasing { 1 } else { -1 };
            let diff = (number - prev) * sign;
            if diff > 3 || diff < 1 {
                println!("not within barrier, value = {}, set to unsafe", diff);
                safe = false;
                break;
            }

            prev = number;
        }

        if safe {
            num_safe += 1;
        }
    }

    num_safe
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn test_part_one_example() {
        let input = "7 6 4 2 1
                    1 2 7 8 9
                    9 7 6 2 1
                    1 3 2 4 5
                    8 6 4 4 1
                    1 3 6 7 9";

        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn test_part_one() {
        let input = common::read_file("data/day_two.txt").unwrap();
        assert_eq!(part_one(&input), 202);
    }
}
