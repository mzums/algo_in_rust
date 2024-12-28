fn manacher(s: &str) -> (usize, usize) {
    let chars: Vec<char> = s.chars().collect();

    let mut t = vec!['#'];
    for &ch in &chars {
        t.push(ch);
        t.push('#');
    }

    let t_len = t.len();
    let mut p = vec![0; t_len];
    let mut center = 0;
    let mut right = 0;

    for i in 0..t_len {
        if i < right {
            p[i] = p[2 * center - i].min(right - i);
        }

        while i + p[i] + 1 < t_len && i as isize - p[i] as isize - 1 >= 0 && t[i + p[i] + 1] == t[i - p[i] - 1] {
            p[i] += 1;
        }

        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
    }

    let (max_center, &max_len) = p
        .iter()
        .enumerate()
        .max_by_key(|&(_, &length)| length)
        .unwrap();

    let start = if max_center >= max_len {
        (max_center - max_len) / 2
    } else {
        0
    };

    (start, max_len)
}


fn main() {
    let s = "abccabddabadd";
    let (start, length) = manacher(s);
    println!("Longest palindrom begins at index {} and has length {}.", start, length);
    println!("Palindrom: {}", &s[start..start + length]);
}
