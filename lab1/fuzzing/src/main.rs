use rand::Rng;
use std::cmp::Ordering;

const CASES: usize = 3000;

fn shortlex_cmp(a: &str, b: &str) -> Ordering {
    match a.len().cmp(&b.len()) {
        Ordering::Equal => a.cmp(b),
        o => o, // сокращенная запись: оставшиеся случаи
    }
}

fn random_word(len_min: usize, len_max: usize) -> String {
    let mut rng = rand::rng();

    let len = rng.random_range(len_min..=len_max);

    (0..len)
        .map(|_| if rng.random_bool(0.5) { 'a' } else { 'b' })
        .collect()
}

fn lcs_string(a: &str, b: &str) -> String {
    let aa: Vec<char> = a.chars().collect();
    let bb: Vec<char> = b.chars().collect();
    let n = aa.len();
    let m = bb.len();
    let mut dp = vec![vec![0usize; m + 1]; n + 1];

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if aa[i] == bb[j] {
                dp[i][j] = 1 + dp[i + 1][j + 1]
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1])
            }
        }
    }

    let mut i = 0;
    let mut j = 0;
    let mut out = String::new();
    while i < n && j < m {
        if aa[i] == bb[j] { 
            out.push(aa[i]); 
            i += 1; 
            j += 1; 
        }
        else if dp[i + 1][j] >= dp[i][j + 1] { 
            i += 1; 
        }
        else { 
            j += 1; 
        }
    }
    out
}

fn deletion_script(src: &str, target: &str) -> Vec<String> {
    let s: Vec<char> = src.chars().collect();
    let t: Vec<char> = target.chars().collect();

    let mut keep = vec![false; s.len()];
    let mut p = 0usize;
    for i in 0..s.len() { 
        if p < t.len() && s[i] == t[p] { 
            keep[i] = true; 
            p += 1; 
        } 
    }

    let mut cur: Vec<char> = s.clone();
    let mut steps = Vec::new();
    let mut removed = 0usize;
    for i in 0..s.len() {
        if !keep[i] {
            let pos = i - removed;
            cur.remove(pos);
            steps.push(cur.iter().collect());
            removed += 1;
        }
    }
    steps
}

fn reduce_to_eps_steps(src: &str) -> Vec<String> {
    let mut cur: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    while !cur.is_empty() { 
        cur.pop(); 
        out.push(cur.iter().collect()); 
    }
    out
}

fn run_case(w1: &str, w2: &str) {
    let m = lcs_string(w1, w2);
    let n1 = w1.len() - m.len();
    let n2 = w2.len() - m.len();

    let rel = match shortlex_cmp(w1, w2) {
        Ordering::Less => "<",
        Ordering::Equal => "=",
        Ordering::Greater => ">",
    };

    println!("w1: {w1}");
    println!("w2: {w2}  (w1 {rel} w2)");
    println!("meet: {m}  n1={n1} n2={n2}");

    let s1 = deletion_script(w1, &m);
    let s2 = deletion_script(w2, &m);

    print!("w1 -> m : {w1}");
    for s in &s1 { 
        print!(" -> {s}"); 
    }
    println!();

    print!("w2 -> m : {w2}");
    for s in &s2 { 
        print!(" -> {s}"); 
    }
    println!();

    let tail = reduce_to_eps_steps(&m);
    print!("m  -> ε : {m}");
    for s in &tail { 
        print!(" -> {s}"); 
    }
    println!(" -> ε");
}

fn main() {
    for i in 0..CASES {
        let mut w1 = random_word(3, 10);
        let mut w2 = random_word(3, 10);

        if shortlex_cmp(&w1, &w2).is_gt() { 
            std::mem::swap(&mut w1, &mut w2); 
        }

        println!("=== case {} ===", i);
        run_case(&w1, &w2);
        println!();
    }
}
