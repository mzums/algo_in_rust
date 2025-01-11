use std::io::{self, BufRead};

fn compute_prefix_function(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let mut prefix = vec![0; m];
    let mut k = 0;

    for q in 1..m {
        while k > 0 && pattern[k] != pattern[q] {
            k = prefix[k - 1];
        }
        if pattern[k] == pattern[q] {
            k += 1;
        }
        prefix[q] = k;
    }

    prefix
}

fn kmp_search(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    let n = text.len();
    let m = pattern.len();
    let prefix = compute_prefix_function(pattern);

    let mut occurrences = Vec::new();
    let mut q = 0;

    for i in 0..n {
        while q > 0 && pattern[q] != text[i] {
            q = prefix[q - 1];
        }
        if pattern[q] == text[i] {
            q += 1;
        }
        if q == m {
            occurrences.push(i + 1 - m);
            q = prefix[q - 1];
        }
    }

    occurrences
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let _n: usize = parts.next().unwrap().parse().unwrap();
    let _m: usize = parts.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap().unwrap();
    let mut parts = second_line.split_whitespace();
    let pattern = parts.next().unwrap().as_bytes();
    let text = parts.next().unwrap().as_bytes();

    let occurrences = kmp_search(text, pattern);

    for index in occurrences {
        println!("{}", index + 1);
    }
}

