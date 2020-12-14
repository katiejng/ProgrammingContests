struct InputRule {
    timestamp: usize,
    bus_times: Vec<BusTime>,
}

#[derive(PartialEq)]
enum BusTime {
    Bus(usize),
    X,
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> InputRule {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse().unwrap();
    let bus_times = lines
        .next()
        .unwrap()
        .split(',')
        .map(|time| {
            if time == "x" {
                return BusTime::X;
            } else {
                return BusTime::Bus(time.parse().unwrap());
            }
        })
        .collect();

    return InputRule {
        bus_times: bus_times,
        timestamp: timestamp,
    };
}

#[aoc(day13, part1)]
fn solve_part1(input: &InputRule) -> i32 {
    let closest_buses = input
        .bus_times
        .iter()
        .filter_map(|bus_time| match bus_time {
            BusTime::X => None,
            BusTime::Bus(b) => Some(b),
        })
        .map(|&bus_time| {
            (
                bus_time,
                ((input.timestamp as f64 / bus_time as f64).ceil() * bus_time as f64) as i32,
            )
        });

    let mut help: Vec<(usize, i32)> = closest_buses.collect();
    help.sort_by(|a, b| a.1.cmp(&b.1));

    let result = help[0];

    return result.0 as i32 * (result.1 as i32 - input.timestamp as i32);
}
impl BusTime {
    fn get_time(&self) -> usize {
        match self {
            BusTime::X => panic!("oops"),
            BusTime::Bus(value) => *value,
        }
    }
}

// #[aoc(day13, part2, alt1)]
// fn solve_part2(input: &InputRule) -> usize {
//     let mut t = 0;
//     let first_time = input.bus_times[0].get_time();
//     loop {
//         let result = input.bus_times.iter().enumerate().all(|(index, bus_time)| {
//             if bus_time.is_x() {
//                 return true;
//             }
//             (t + index) % bus_time.get_time() == 0
//         });

//         if result {
//             return t;
//         }
//         t += first_time;
//     }
// }

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[derive(Debug)]
struct Item {
    time: usize,
    lcm: usize,
}

#[aoc(day13, part2)]
fn solve_part2(input: &InputRule) -> usize {
    let delay = input.bus_times.len() - 1;
    let initial_value = Item {
        time: input.bus_times[0].get_time(),
        lcm: input.bus_times[0].get_time(),
    };
    let result = input.bus_times[1..]
        .iter()
        .fold(initial_value, |acc, bus_time| match bus_time {
            BusTime::X => Item {
                time: acc.time + 1,
                lcm: acc.lcm,
            },
            BusTime::Bus(t) => {
                let new_lcm = lcm(*t, acc.lcm);
                let multiplier = (0..)
                    .skip_while(|&x| (acc.time + acc.lcm * x + 1) % t != 0)
                    .next()
                    .unwrap();
                Item {
                    time: acc.time + acc.lcm * multiplier + 1,
                    lcm: new_lcm,
                }
            }
        });
    result.time - delay
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"939
7,13,x,x,59,x,31,19";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 295;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let real_input = r"939
7,13,x,x,59,x,31,19";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 1068781;
        assert_eq!(result, expected);
    }
}
