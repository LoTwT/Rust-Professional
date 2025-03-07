use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let mut unique_elements = HashSet::new();

    // Split by comma and add each element to the HashSet
    for element in input_str.split(',') {
        unique_elements.insert(element);
    }

    // Return the count of unique elements
    unique_elements.len()
}
