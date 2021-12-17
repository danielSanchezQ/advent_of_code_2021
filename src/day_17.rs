use sscanf::const_format::pmr::Range;
use std::collections::HashSet;

type InitialVelocity = isize;
type MaxT = usize;

fn solve_x(
    start: isize,
    end: isize,
    min_pos: isize,
    max_pos: isize,
) -> Vec<(InitialVelocity, MaxT)> {
    (start..end)
        .flat_map(|v| {
            let mut ts = Vec::new();
            let mut pos = 0;
            let mut current_speed = v as usize;
            let mut t = 0usize;
            loop {
                if current_speed == 0 && pos >= min_pos && pos <= max_pos {
                    ts.push((v, t));
                    break;
                } else if current_speed == 0 {
                    break;
                } else if pos >= min_pos && pos <= max_pos {
                    ts.push((v, t));
                }
                pos += current_speed as isize;
                current_speed = current_speed.wrapping_sub(1);
                t += 1;
            }
            // while current_speed > 0 && pos <= max_pos {
            //     if pos >= min_pos && pos <= max_pos {
            //         ts.push((v, t));
            //     }
            //     pos += current_speed as isize;
            //     current_speed = current_speed.wrapping_sub(1);
            //     t += 1;
            // }
            ts
        })
        .collect()
}

fn initial_y_speed(initial_y: isize, final_y: isize, g: isize, t: usize) -> isize {
    (final_y - initial_y + (g * (t as isize * t as isize) / 2)) / t as isize
}

fn solve_y(
    ts: &[(InitialVelocity, MaxT)],
    starting_position: isize,
    y_range: usize,
    g: usize,
) -> Vec<isize> {
    ts.iter()
        .flat_map(|(_, t_max)| {
            (0..=y_range)
                .map(|final_y| {
                    let initial_speed =
                        initial_y_speed(starting_position, final_y as isize, g as isize, *t_max);
                    (0usize..=*t_max)
                        .map(|t| {
                            let heigh = starting_position + initial_speed * t as isize
                                - (t as isize * t as isize) / 2;
                            heigh
                        })
                        .max()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn exploit_y(
    min: isize,
    max: isize,
    target_min: isize,
    target_max: isize,
    max_t: usize,
) -> Vec<(isize, InitialVelocity, MaxT)> {
    (min..=max)
        .filter_map(|initial_velocity| {
            let mut current_pos = 0;
            let mut speed = initial_velocity;
            let mut t = 0;
            let mut max_height = 0;
            let mut found = false;
            loop {
                if current_pos <= target_min && current_pos >= target_max {
                    found = true;
                    break;
                }
                // if t > max_t {
                //     break;
                // }
                if current_pos < target_max {
                    break;
                }
                if current_pos > max_height {
                    max_height = current_pos;
                }
                current_pos += speed;
                t += 1;
                speed -= 1;
            }
            if found {
                Some((max_height, initial_velocity, t))
            } else {
                None
            }
        })
        .collect()
}

fn solve_part_1() -> isize {
    let x_data = solve_x(1, 171, 150, 171);
    let max_t = x_data.iter().map(|(_, t)| *t).max().unwrap();
    let y_data = exploit_y(1, 10000, -70, -130, max_t);
    println!("{:?}", x_data);
    println!("{:?}", y_data);
    y_data.iter().map(|(m, _, _)| *m).max().unwrap()
    // let diff = Y_RANGE.start.abs_diff(Y_RANGE.end);
    // let starting_position = Y_RANGE.start.abs();
    //
    // println!("Diff {}, starting position {}", diff, starting_position);
    // solve_y(&x_data, starting_position, diff, 1)
    //     .into_iter()
    //     .max()
    //     .unwrap()
    //     - starting_position
}

#[cfg(test)]
mod test {
    use crate::day_17::{solve_part_1, solve_x};

    // #[test]
    // fn example_part_1() {
    //     let example_range_x = 20..31;
    //     let example_range_y = -10..-5;
    //     let x_data = solve_x(1, example_range_x.end, example_range_x.start, example_range_x.end);
    //     let diff = example_range_y.start.abs_diff(example_range_y.end);
    //     let starting_position = example_range_y.start.abs();
    //     solve_y(&x_data, starting_position, diff, 1)
    //         .into_iter()
    //         .max()
    //         .unwrap()
    //     println!("Day 1 part 1 solution: {}", solve_part_1());
    // }

    #[test]
    fn part_1() {
        println!("Day 1 part 1 solution: {}", solve_part_1());
    }
}
