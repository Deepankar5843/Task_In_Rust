use std::cmp;

fn main() {
    // Task 1: Check if a given string is a palindrome
    let palindrome_string = "racecar";
    println!("Is '{}' a palindrome? {}", palindrome_string, is_palindrome(palindrome_string));

    // Task 2: Find the index of the first occurrence of a given number in a sorted array
    let sorted_array = [1, 3, 5, 7, 9];
    let target_number = 5;
    println!("Index of first occurrence of {}: {:?}", target_number, find_first_occurrence(&sorted_array, target_number));

    // Task 3: Return the shortest word in a string of words
    let words_string = "The quick brown fox jumps over the lazy dog";
    println!("Shortest word in '{}': {:?}", words_string, shortest_word(words_string));

    // Task 4: Check if a given number is prime
    let number_to_check = 17;
    println!("Is {} prime? {}", number_to_check, is_prime(number_to_check));

    // Task 5: Return the median of a sorted array of integers
    let sorted_int_array = [1, 2, 3, 4, 5];
    println!("Median of {:?}: {}", sorted_int_array, median(&sorted_int_array));

    // Task 6: Find the longest common prefix of a given set of strings
    let string_set = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("Longest common prefix: {}", longest_common_prefix(&string_set));

    // Task 7: Return the kth smallest element in a given array
    let kth_array = [4, 3, 1, 7, 5];
    let k = 3;
    println!("{}th smallest element in {:?}: {:?}", k, kth_array, kth_smallest(&kth_array, k));

    // Task 8: Return the maximum depth of a binary tree
    // Not implemented as it requires a binary tree structure.

    // Task 9: Reverse a string
    let string_to_reverse = "hello";
    println!("Reverse of '{}': {}", string_to_reverse, reverse_string(string_to_reverse));

    // Task 10: Check if a number is prime
    let number_to_check_rust = 23;
    println!("Is {} prime? {}", number_to_check_rust, is_prime_rust(number_to_check_rust));

    // Task 11: Merge two sorted arrays
    let arr1 = [1, 3, 5];
    let arr2 = [2, 4, 6];
    println!("Merged array: {:?}", merge_sorted_arrays(&arr1, &arr2));

    // Task 12: Find the maximum subarray sum
    let subarray = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum subarray sum: {}", max_subarray_sum(&subarray));
}

// Task 1: Check if a given string is a palindrome
fn is_palindrome(input: &str) -> bool {
    input.chars().eq(input.chars().rev())
}

// Task 2: Find the index of the first occurrence of a given number in a sorted array
fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// Task 3: Return the shortest word in a string of words
fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|&word| word.len())
}

// Task 4: Check if a given number is prime
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Task 5: Return the median of a sorted array of integers
fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// Task 6: Find the longest common prefix of a given set of strings
fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let min_len = strings.iter().map(|s| s.len()).min().unwrap();
    for i in 0..min_len {
        let c = strings[0].as_bytes()[i];
        if strings.iter().any(|s| s.as_bytes()[i] != c) {
            return strings[0][..i].to_string();
        }
    }
    strings[0][..min_len].to_string()
}

// Task 7: Return the kth smallest element in a given array
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr.get(k - 1).cloned()
}

// Task 9: Reverse a string
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// Task 10: Check if a number is prime
fn is_prime_rust(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Task 11: Merge two sorted arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);

    merged
}

// Task 12: Find the maximum subarray sum
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = arr[0];
    let mut max_so_far = arr[0];

    for &num in arr.iter().skip(1) {
        max_ending_here = cmp::max(num, max_ending_here + num);
        max_so_far = cmp::max(max_so_far, max_ending_here);
    }

    max_so_far
}
