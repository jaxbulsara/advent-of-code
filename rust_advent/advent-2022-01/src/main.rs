use std::cmp;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read input file.");
    let lines: Vec<&str> = contents.split("\n").collect();
    let calorie_groups = create_calorie_groups(lines);
    let calorie_sums: Vec<i32> = calorie_groups
        .iter()
        .map(|group| group.iter().sum())
        .collect();
    let max_calorie_sum = calorie_sums
        .iter()
        .reduce(|acc, e| cmp::max(acc, e))
        .unwrap();
    println!("{:?}", max_calorie_sum);
}

fn create_calorie_groups(lines: Vec<&str>) -> Vec<Vec<i32>> {
    let mut calorie_groups: Vec<Vec<i32>> = Vec::new();
    let mut group: Vec<i32> = Vec::new();

    for line in lines {
        match line {
            "" => {
                calorie_groups.push(group);
                group = Vec::new();
            }
            _ => {
                let calories = line
                    .parse::<i32>()
                    .expect("Calorie value must be an integer.");
                group.push(calories);
            }
        }
    }

    if !group.is_empty() {
        calorie_groups.push(group);
    }

    calorie_groups
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_calorie_groups() {
        let input = vec!["1", "2", "3", "", "3", "4", "5"];
        let expected = vec![vec![1, 2, 3], vec![3, 4, 5]];
        assert_eq!(super::create_calorie_groups(input), expected);
    }
}
