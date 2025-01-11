use std::io::{self, BufRead};

const P: i64 = 199;
const MOD: i64 = 1_000_000_007;

fn get_hash(l: usize, r: usize, n: usize, h: &[i64], pow_p: &[i64]) -> i64 {
    if l > r || r >= n {
        return 0;
    }
    let res = (h[r + 1] - h[l] + MOD) % MOD;
    (res * pow_p[n - l]) % MOD
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    
    let a = input.next().unwrap().unwrap();
    let b = input.next().unwrap().unwrap();

    let n = b.len();
    let m = a.len();

    if m > n {
        println!("0");
        return;
    }

    let mut pow_p = vec![0; n + 1];
    let mut h = vec![0; n + 1];

    pow_p[0] = 1;
    for i in 1..=n {
        pow_p[i] = (pow_p[i - 1] * P) % MOD;
    }

    let mut pattern = 0;
    for i in 0..m {
        pattern = (pattern + pow_p[i] * (a.as_bytes()[i] as i64)) % MOD;
    }
    pattern = (pattern * pow_p[n]) % MOD;

    for i in 0..n {
        h[i + 1] = (h[i] + pow_p[i] * (b.as_bytes()[i] as i64)) % MOD;
    }

    let mut score = 0;
    let mut positions = Vec::new();
    for i in m - 1..n {
        if i + 1 >= m && get_hash(i + 1 - m, i, n, &h, &pow_p) == pattern {
            score += 1;
            positions.push(i + 2 - m);
        }
    }

    println!("{}", score);
    for pos in positions {
        print!("{} ", pos);
    }
    println!();
}

