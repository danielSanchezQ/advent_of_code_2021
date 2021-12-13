use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct Pathways(HashMap<String, HashSet<String>>);

impl FromStr for Pathways {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();
        for l in s.lines() {
            let (init, end): (String, String) = sscanf::scanf!(l, "{}-{}", String, String).unwrap();
            map.entry(init.clone()).or_default().insert(end.clone());
            map.entry(end).or_default().insert(init);
        }
        map.remove("end");
        Ok(Self(map))
    }
}

impl Pathways {
    fn generate_path_from(
        &self,
        from: String,
        mut visited: HashSet<String>,
        mut visited_counts: HashMap<String, usize>,
        mut previous: Vec<String>,
    ) -> Vec<Vec<String>> {
        if from.to_ascii_lowercase() == from {
            if let Some(&counter) = visited_counts.get(&from) {
                if counter == 0 {
                    visited.insert(from.to_string());
                } else if counter == 1 {
                    if visited_counts.values().any(|&v| v >= 2) {
                        visited_counts.values_mut().for_each(|v| *v -= 1);
                    } else {
                        *visited_counts.get_mut(&from).unwrap() -= 1;
                    }
                    for (k, &v) in &visited_counts {
                        if v == 0 {
                            visited.insert(k.to_string());
                        }
                    }
                } else if counter == 2 {
                    *visited_counts.get_mut(&from).unwrap() -= 1;
                }
            }
        }
        previous.push(from.to_string());
        if let Some(paths) = self.0.get(&from) {
            paths
                .iter()
                .filter(|e| !visited.contains(*e))
                .flat_map(|e| {
                    let visited = visited.clone();
                    let previous = previous.clone();
                    let visited_counts = visited_counts.clone();
                    self.generate_path_from(e.clone(), visited, visited_counts, previous)
                })
                .collect()
        } else {
            vec![previous]
        }
    }

    fn generate_paths(&self, visited_count_default: usize) -> Vec<Vec<String>> {
        let visited: HashSet<String> = ["start".to_string()].into_iter().collect();
        let mut visited_counts: HashMap<String, usize> = self
            .0
            .keys()
            .map(|k| (k.clone(), visited_count_default))
            .collect();
        visited_counts.remove("start");
        visited_counts.remove("end");
        self.0
            .get("start")
            .unwrap()
            .iter()
            .flat_map(move |e| {
                self.generate_path_from(
                    e.clone(),
                    visited.clone(),
                    visited_counts.clone(),
                    vec!["start".to_string()],
                )
            })
            .collect()
    }
}

fn solve_part_1(paths: Pathways) -> usize {
    let generated = paths.generate_paths(0);
    generated
        .iter()
        .filter(|v| v.last().unwrap() == "end")
        .count()
}

fn solve_part_2(paths: Pathways) -> usize {
    let generated = paths.generate_paths(2);
    generated
        .iter()
        .filter(|v| v.last().unwrap() == "end")
        .count()
}

#[cfg(test)]
mod test {
    use crate::day_12::{solve_part_1, solve_part_2, Pathways};
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn example_1() {
        let input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let pathways: Pathways = input.parse().unwrap();
        assert_eq!(solve_part_1(pathways), 10);
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let pathways: Pathways =
            io::read_object_from_file(&PathBuf::from_str("./inputs/day_12.txt").unwrap())?;
        println!("Day 12 part 1 solution: {}", solve_part_1(pathways));
        Ok(())
    }

    #[test]
    fn example_2() {
        let input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let pathways: Pathways = input.parse().unwrap();
        assert_eq!(solve_part_2(pathways), 36);
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let pathways: Pathways =
            io::read_object_from_file(&PathBuf::from_str("./inputs/day_12.txt").unwrap())?;
        println!("Day 12 part 2 solution: {}", solve_part_2(pathways));
        Ok(())
    }
}
