
pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config{file_path})
    }
    
}
pub fn read_file(name: String) -> std::io::Result<String> {
    let content = std::fs::read_to_string(name)?;
    Ok(content)
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

pub fn min_max_sum_times_3_is_not_greater_than_sum_of_others_times_2(numbers: Vec<i64>, lines: &mut u64) {
    let first_number = numbers[0];
    let last_number = numbers[numbers.len() - 1];

    let min_max_times_3 = min_max_times_3(first_number, last_number);
    let sum_others_times_2 = sum_others_times_2(first_number, last_number, numbers);

    if min_max_times_3 <= sum_others_times_2 {
        *lines += 1;
    }
}

fn min_max_times_3(first: i64, last: i64) -> i64 {
    (first + last) * 3
}

fn sum_others_times_2(first: i64, last: i64, vec: Vec<i64>) -> i64 {
    (vec.iter().sum::<i64>() -  first - last) * 2
}