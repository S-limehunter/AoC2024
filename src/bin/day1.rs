#![feature(extract_if)]
use std::{collections::HashMap, fs}; 
use itertools::Itertools;

fn split_pairs_and_parse(pair: &str) -> (isize, isize) {
    pair.split_ascii_whitespace()
        .map(
            |num| 
            num.parse::<isize>()
            .expect(format!("Failed to parse element: {num}").as_str())
        )
        .collect_tuple::<(isize, isize)>()
        .expect(format!("Badly formatted list element: {pair}").as_str())
}

// Part 1 solution
fn calculate_total_distance(mut left_list: Vec<isize>, mut right_list: Vec<isize>) -> usize {
    let sorted_pairs = {
        left_list.sort();
        right_list.sort();
        left_list.iter().zip(right_list.iter())
    };
    let sum: usize = sorted_pairs.map(|(a, b)| isize::abs_diff(*a, *b)).sum();
    sum
}

// Part 2 solution
fn calculate_similarity_score(left_list: Vec<isize>, right_list: Vec<isize>) -> isize {
    let mut cached_totals: HashMap<isize, isize> = HashMap::new();
    let mut uncounted: Vec<isize> = right_list;

    let sum: isize = left_list.iter().map(
        |left_element| {   
        let count: &isize = cached_totals
            .entry(*left_element)
            .or_insert_with(
                || 
                uncounted
                    .extract_if(|right_element| right_element == left_element)
                    .count()
                    .try_into()
                    .expect("failed converting from unsigned to signed")
                );
        count * left_element
    }).sum();
    sum
}

fn main() {
    let input_string: String = fs::read_to_string("inputs/day1.txt").unwrap();
    let (mut left_list, mut right_list): (Vec<isize>, Vec<isize>) = 
        input_string.as_str()
            .trim()
            .split('\n')
            .map(split_pairs_and_parse)
            .unzip();
    let sum = calculate_similarity_score(left_list, right_list);
    println!("Calculated similarity score: {}", sum)
}
