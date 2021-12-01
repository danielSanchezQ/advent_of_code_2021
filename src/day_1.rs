// As the submarine drops below the surface of the ocean, it automatically performs a sonar sweep of the nearby sea floor. On a small screen, the sonar sweep report (your puzzle input) appears: each line is a measurement of the sea floor depth as the sweep looks further and further away from the submarine.
//
// For example, suppose you had the following report:
//
// 199
// 200
// 208
// 210
// 200
// 207
// 240
// 269
// 260
// 263
// This report indicates that, scanning outward from the submarine, the sonar sweep found depths of 199, 200, 208, 210, and so on.
//
// The first order of business is to figure out how quickly the depth increases, just so you know what you're dealing with - you never know if the keys will get carried into deeper water by an ocean current or a fish or something.
//
// To do this, count the number of times a depth measurement increases from the previous measurement. (There is no measurement before the first measurement.) In the example above, the changes are as follows:
//
// 199 (N/A - no previous measurement)
// 200 (increased)
// 208 (increased)
// 210 (increased)
// 200 (decreased)
// 207 (increased)
// 240 (increased)
// 269 (increased)
// 260 (decreased)
// 263 (increased)
// In this example, there are 7 measurements that are larger than the previous measurement.
//
// How many measurements are larger than the previous measurement?

fn count_increased_measures<'a>(
    measures: impl Iterator<Item = &'a usize> + Clone,
    skip: usize,
) -> usize {
    let iter1 = measures.clone();
    let iter2 = measures.skip(skip);
    iter1
        .zip(iter2)
        .map(|(a, b)| a.lt(b).then_some(1usize).unwrap_or(0))
        .sum()
}

fn count_increased_measure_sliding_windows(measures: &[usize], windows_size: usize) -> usize {
    count_increased_measures(
        measures
            .windows(windows_size)
            .map(|window| window.iter().sum::<usize>())
            .collect::<Vec<_>>()
            .iter(),
        1,
    )
}

#[cfg(test)]
mod test {
    use crate::day_1::{count_increased_measure_sliding_windows, count_increased_measures};
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn example_1() {
        let input = vec![199usize, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increased_measures(input.iter(), 1), 7);
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let data: Vec<usize> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_1.txt").unwrap())?;
        assert_ne!(data.len(), 0);

        let result = count_increased_measures(data.iter(), 1);
        print!("Day 1, part 1 result: {}", result);
        Ok(())
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let data: Vec<usize> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_1.txt").unwrap())?;
        assert_ne!(data.len(), 0);

        let result = count_increased_measure_sliding_windows(&data, 3);
        print!("Day 1, part 2 result: {}", result);
        Ok(())
    }

    #[test]
    fn part_2_v2() -> std::io::Result<()> {
        let data: Vec<usize> =
            io::read_vec_from_file(&PathBuf::from_str("./inputs/day_1.txt").unwrap())?;
        assert_ne!(data.len(), 0);

        let result = count_increased_measures(data.iter(), 3);
        assert_eq!(result, count_increased_measure_sliding_windows(&data, 3));
        print!("Day 1, part 2 result: {}", result);
        Ok(())
    }
}
