pub fn read_file(name: &str) -> std::io::Result<String> {
    let content = std::fs::read_to_string(name)?;
    Ok(content)
}

pub fn amount_of_lines(file: String) -> u64 {
    let vec_str: Vec<&str> = file.split("\r").collect();
    let mut number_of_lines: u64 = 0;
    
    for string in vec_str {
        let mut single_line = from_string_to_numbers(string);

        let original_len = single_line.len();
        sort_and_remove_dups(&mut single_line); 
        let dedup_len = single_line.len();

        if original_len == dedup_len {
            if min_max_sum_times_3_is_not_greater_than_sum_of_others_times_2(single_line) {
                number_of_lines += 1;
            }
        }
    };
    number_of_lines
}

pub fn from_string_to_numbers(string: &str) -> Vec<i64> {
        string
            .split_whitespace()
            .map(|s|
                s.parse::<i64>()
                .expect("Failed to parse string into number.")
            )
            .collect()
}

pub fn sort_and_remove_dups(vector: &mut Vec<i64>) {
    vector.sort();
    vector.dedup();
}

pub fn min_max_sum_times_3_is_not_greater_than_sum_of_others_times_2(
    numbers: Vec<i64>
) -> bool {
    let first_number = numbers[0];
    let last_number = numbers[numbers.len() - 1];

    let min_max_times_3 = min_max_sum_times_3(first_number, last_number);
    let sum_others_times_2 = sum_others_times_2(first_number, last_number, numbers);

    if min_max_times_3 <= sum_others_times_2 {
        return true;
    } else {
        return false;
    }
}

fn min_max_sum_times_3(first: i64, last: i64) -> i64 {
    (first + last) * 3
}

fn sum_others_times_2(first: i64, last: i64, vec: Vec<i64>) -> i64 {
    (vec.iter().sum::<i64>() -  first - last) * 2
}