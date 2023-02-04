use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read input file.");
    let lines: Vec<&str> = contents.split("\n").collect();
}

fn create_calorie_lists(lines: Vec<&str>) -> Vec<Vec<i32>> {
    let mut calorie_list: Vec<Vec<i32>> = Vec::new();
    let mut calorie_group: Vec<i32> = Vec::new();

    for line in lines {
        match line {
            "" => {
                calorie_list.push(calorie_group);
                calorie_group = Vec::new();
            }
            _ => {
                let calories = line
                    .parse::<i32>()
                    .expect("Calorie value must be an integer.");
                calorie_group.push(calories);
            }
        }
    }

    if !calorie_group.is_empty() {
        calorie_list.push(calorie_group);
    }

    calorie_list
}

fn sum_calorie_groups(calorie_list: Vec<Vec<i32>>) -> Vec<i32> {
    let mut calorie_sums: Vec<i32> = Vec::new();
    for group in calorie_list {
        let sum: i32 = group.iter().sum();
        calorie_sums.push(sum);
    }

    calorie_sums
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_calorie_list() {
        let input = vec!["1", "2", "3", "", "3", "4", "5"];
        let expected = vec![vec![1, 2, 3], vec![3, 4, 5]];
        assert_eq!(super::create_calorie_lists(input), expected);
    }

    #[test]
    fn test_sum_calorie_groups() {
        let input = vec![vec![1, 2, 3], vec![3, 4, 5]];
        let expected = vec![6, 12];
        assert_eq!(super::sum_calorie_groups(input), expected);
    }
}
