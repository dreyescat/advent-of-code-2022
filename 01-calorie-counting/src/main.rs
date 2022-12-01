use std::{cmp, io};

fn main() -> io::Result<()> {
    let mut max_calories = 0;
    let mut current_calories = 0;
    for line in io::stdin().lines() {
        match line?.parse::<u32>() {
            Ok(calories) => current_calories += calories,
            Err(_) => {
                max_calories = cmp::max(current_calories, max_calories);
                current_calories = 0;
            }
        }
    }
    println!("{}", max_calories);

    Ok(())
}
