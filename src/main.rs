use ege_2::*;
use std::{env, process};
fn main() -> std::io::Result<()> {
    // Should be a better way but can't find it atm,
    // beacuse it gets os error 123 if taken from stdin()
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let contents = read_file(config.file_path)?;
    let vec_str: Vec<&str> = contents.split("\r").collect();

    let mut number_of_lines: u64 = 0;
    
    for string in vec_str {
        let mut single_line = from_string_to_numbers(string);

        let original_len = single_line.len();
        sort_and_remove_dups(&mut single_line); 
        let dedup_len = single_line.len();

        if original_len == dedup_len {
            min_max_sum_times_3_is_not_greater_than_sum_of_others_times_2(single_line, &mut number_of_lines);
        }
    };
    
    println!("{number_of_lines}");
    Ok(())
}

