use std::collections::BinaryHeap;
use std::io;

fn main() -> io::Result<()> {
    let mut top_calories = BinaryHeap::new();
    let mut current_calories = 0;
    for line in io::stdin().lines() {
        match line?.parse::<u32>() {
            Ok(calories) => current_calories += calories,
            Err(_) => {
                top_calories.push(current_calories);
                current_calories = 0;
            }
        }
    }
    top_calories.push(current_calories);
    println!("Top one: {}", top_calories.peek().unwrap());
    println!(
        "Top three {}",
        top_calories
            .into_sorted_vec()
            .iter()
            .rev()
            .take(3)
            .sum::<u32>()
    );

    Ok(())
}
