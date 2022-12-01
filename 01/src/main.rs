use std::{
    collections::BinaryHeap,
    fs::File,
    io::{self, BufRead, BufReader},
};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};

const CALORIES_PATH: &str = "calories.txt";

fn main() -> Result<()> {
    color_eyre::install()?;

    let file =
        File::open(CALORIES_PATH).wrap_err_with(|| format!("couldn't open `{}`", CALORIES_PATH))?;
    let reader = BufReader::new(file);

    let mut calories =
        count_calories(reader).wrap_err_with(|| format!("error reading `{}`", CALORIES_PATH))?;

    let Some(most_calories) = calories.pop() else {
        bail!("No calories in `{}`", CALORIES_PATH);
    };

    eprintln!("Most calories");
    println!("{}", most_calories);

    let next_top_2_calories: u32 = std::iter::from_fn(|| calories.pop()).take(2).sum();
    eprintln!("Combined top-3 calories");
    println!("{}", most_calories + next_top_2_calories);

    Ok(())
}

fn count_calories(reader: impl BufRead) -> io::Result<BinaryHeap<u32>> {
    let mut calories = BinaryHeap::new();
    let mut current_calories = 0;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            if current_calories > 0 {
                calories.push(current_calories);
                current_calories = 0;
            }
        } else {
            let calories: u32 = line.parse().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("invalid calorie value `{}`", line),
                )
            })?;
            current_calories += calories;
        }
    }
    if current_calories > 0 {
        calories.push(current_calories);
    }

    Ok(calories)
}

#[cfg(test)]
mod tests {
    use super::count_calories;

    #[test]
    fn test_count_calories() {
        let input = r#"

12
156

75
900

4535

342
12
67

4535
1

1234
0

"#;
        let mut result = count_calories(input.as_bytes()).unwrap();
        let list: Vec<_> = std::iter::from_fn(|| result.pop()).collect();
        assert_eq!(&list, &[4536, 4535, 1234, 975, 421, 168]);
    }
}
