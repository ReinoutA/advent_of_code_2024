use std::{cmp::Reverse, collections::HashMap};

fn filter_page_updates<'a>(
    page_updates: &'a Vec<Vec<i64>>,
    page_orderings: &'a HashMap<i64, Vec<i64>>,
) -> impl Iterator<Item = &'a [i64]> {
    page_updates.iter().filter_map(move |update_nums| {
        let valid = update_nums.iter().rev().fold(
            (true, Vec::with_capacity(update_nums.len())),
            |(valid, mut viewed_nums), &num| {
                let is_valid = viewed_nums.iter().all(|&viewed_num| {
                    page_orderings.get(&viewed_num).map_or(true, |ordering| {
                        !ordering.contains(&num)
                    })
                });

                viewed_nums.push(num);
                (valid && is_valid, viewed_nums)
            },
        ).0;

        if !valid {
            Some(update_nums.as_slice())
        } else {
            None
        }
    })
}

fn fix(page_orderings: &HashMap<i64, Vec<i64>>, page_updates: &Vec<Vec<i64>>) -> i64 {
    filter_page_updates(page_updates, page_orderings)
        .map(|update_nums| {
            let mut filtered_nums: Vec<(i64, Vec<i64>)> = update_nums.iter().map(|&num| {
                let after_nums = page_orderings.get(&num)
                    .into_iter()
                    .flatten()
                    .filter(|&after_num| update_nums.contains(&after_num))
                    .copied()
                    .collect::<Vec<i64>>();
                (num, after_nums)
            }).collect();

            filtered_nums.sort_by_key(|(_, after_nums)| Reverse(after_nums.len()));

            let middle_index = filtered_nums.len() / 2;
            filtered_nums[middle_index].0
        })
        .sum()
}

fn main() {
    let reader = std::fs::read_to_string("assets/day5.txt").expect("Failed to read file");

    let mut page_orderings = HashMap::new();
    let mut page_updates = Vec::new();
    let mut is_subsequent_section = false;

    for line in reader.lines() {
        let line = line.trim();
        if line.is_empty() {
            is_subsequent_section = true;
            continue;
        }

        if is_subsequent_section {
            let update_nums: Vec<i64> = line.split(',')
                .filter_map(|num| num.trim().parse().ok())
                .collect();
            page_updates.push(update_nums);
        } else {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let before = parts[0].trim().parse::<i64>().unwrap();
                let after = parts[1].trim().parse::<i64>().unwrap();
                page_orderings.entry(before)
                    .or_insert_with(Vec::new)
                    .push(after);
            }
        }
    }

    let result = fix(&page_orderings, &page_updates);
    println!("Result: {}", result);
}
