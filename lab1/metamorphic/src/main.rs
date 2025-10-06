use rand::prelude::*;
use std::collections::HashSet;

type Rule = (&'static str, &'static str);

const T: &[Rule] = &[
    ("aaa",        ""),
    ("bbb",        "bb"),
    ("baab",       "baa"),
    ("babab",      "aabab"),
    ("bbbab",      "bbaab"),
    ("abaaab",     "baba"),
    ("ababbab",    "ababaab"),
    ("bbabbaa",    "bbaaa"),
    ("aabbabba",   "bbaa"),
    ("aabaababb",  "bbb"),
    ("bbabbabba",  "baababbaa"),
];


const TP: &[Rule] = &[
    ("aaa", ""),
    ("ab",  "b"),
    ("ba",  "b"),
    ("bb",  "b"),
];

fn counts(s: &str) -> (usize, usize) {
    let mut a = 0usize;
    let mut b = 0usize;
    for c in s.bytes() {
        if c == b'a' { a += 1; }
        if c == b'b' { b += 1; }
    }
    (a, b)
}


#[derive(PartialEq, Debug)]
struct Inv {
    has_b: u8,        
    a_mod3_mask: u8,  
    dfa_state: u8,   
    matrix_code: u8, 
}

fn inv_has_b(s: &str) -> u8 {
    let (_, b) = counts(s);
    (b > 0) as u8
}

fn inv_a_mod3_mask(s: &str) -> u8 {
    let (a, b) = counts(s);

    if b > 0 { 
        0
    } 
    else { 
        (a % 3) as u8 
    }
}

fn inv_dfa(s: &str) -> u8 {
    // states: 0=E,1=A,2=AA,3=B
    let mut st = 0u8;

    for c in s.bytes() {
        if st == 3 { 
            continue; 
        }
        if c == b'b' { 
            st = 3; 
        }
        else if c == b'a' {
            st = match st { 
                0 => 1, 
                1 => 2, 
                _ => 0,
            };
        }
    }
    st
}

fn inv_matrix_code(s: &str) -> u8 {
    let (a, b) = counts(s);
    
    if b > 0 { 
        return 3; 
    }
    match a % 3 { 
        0 => 0, 
        1 => 1, 
        _ => 2,
    }
}

fn invariants(s: &str) -> Inv {
    Inv {
        has_b:       inv_has_b(s),
        a_mod3_mask: inv_a_mod3_mask(s),
        dfa_state:   inv_dfa(s),
        matrix_code: inv_matrix_code(s),
    }
}

fn neighbors(word: &str, rules: &[Rule]) -> Vec<String> {
    let mut outs: HashSet<String> = HashSet::new();
    let n = word.len();

    for (lhs, rhs) in rules {
        
        let mut start = 0usize;
        while start <= n {
            if let Some(i) = word[start..].find(lhs) {
                let pos = start + i;
                let mut w = String::new();
                w.push_str(&word[..pos]);
                w.push_str(rhs);
                w.push_str(&word[pos + lhs.len()..]);
                outs.insert(w);
                start = pos + 1;
            } else { break; }
        }


        if rhs.is_empty() {
            // ε -> lhs: вставка lhs в любую позицию 0..=n
            for pos in 0..=n {
                let mut w = String::new();
                w.push_str(&word[..pos]);
                w.push_str(lhs);
                w.push_str(&word[pos..]);
                outs.insert(w);
            }
        } else {
            let mut start2 = 0usize;
            while start2 <= n {
                if let Some(i) = word[start2..].find(rhs) {
                    let pos = start2 + i;
                    let mut w = String::new();
                    w.push_str(&word[..pos]);
                    w.push_str(lhs);
                    w.push_str(&word[pos + rhs.len()..]);
                    outs.insert(w);
                    start2 = pos + 1;
                } else { break; }
            }
        }
    }

    outs.remove(word);
    outs.into_iter().collect()
}

fn random_word(len_min: usize, len_max: usize) -> String {
    let mut rng = rand::rng();

    let len = rng.random_range(len_min..=len_max);

    (0..len)
        .map(|_| if rng.random_bool(0.5) { 'a' } else { 'b' })
        .collect()
}


fn random_chain(
    rng: &mut StdRng,
    w0: &str,
    rules: &[Rule],
    min_steps: usize,
    max_steps: usize,
) -> Vec<String> {
    let mut w = w0.to_string();
    let mut chain = vec![w.clone()];
    let steps = rng.random_range(min_steps..=max_steps);

    for _ in 0..steps {
        let ns = neighbors(&w, rules);
        if ns.is_empty() { 
            break; 
        }

        let pick = rng.random_range(0..ns.len());
        w = ns[pick].clone();
        chain.push(w.clone());
    }
    chain
}

fn check_chain(chain: &[String]) -> (bool, Inv, Option<(Inv, String)>) {
    let base = invariants(&chain[0]);
    for w in &chain[1..] {
        let inv = invariants(w);
        if inv != base {
            return (false, base, Some((inv, w.clone())));
        }
    }
    (true, base, None)
}

fn unit_check_rules(rules: &[Rule]) -> (bool, Vec<(String, String, Inv, Inv)>) {
    let mut ok = true;
    let mut bad = Vec::new();
    for (l, r) in rules {
        let il = invariants(l);
        let ir = invariants(r);
        if il != ir {
            ok = false;
            bad.push(((*l).to_string(), (*r).to_string(), il, ir));
        }
    }
    (ok, bad)
}

fn fuzz_test(
    rng: &mut StdRng,
    rules: &[Rule],
    trials: usize,
    min_len: usize,
    max_len: usize,
    min_steps: usize,
    max_steps: usize,
) -> (usize, usize, Vec<String>) {
    let mut ok = 0usize;
    let mut fail = 0usize;
    let mut examples = Vec::new();

    for _ in 0..trials {
        let w0 = random_word(min_len, max_len);
        let chain = random_chain(rng, &w0, rules, min_steps, max_steps);
        let (good, base, bad) = check_chain(&chain);
        if good {
            ok += 1;
        } else {
            fail += 1;
            let (inv_bad, wbad) = bad.unwrap();
            
            examples.push(format!(
                "FAIL: start='{}'  base={:?}  bad='{}'  inv_bad={:?}",
                w0, base, wbad, inv_bad
            ));
        }
    }
    (ok, fail, examples)
}

fn main() {
    let (ok_t,  bad_t)  = unit_check_rules(T);
    let (ok_tp, bad_tp) = unit_check_rules(TP);
    println!("Unit check:");

    println!("  T  : {}", if ok_t  { "OK" } else { "FAIL" });
    if !ok_t  { 
        for (l,r,il,ir) in bad_t  { 
            println!("    {} -> {} | {:?} vs {:?}", l, r, il, ir); 
    }
    }

    println!("  T' : {}", if ok_tp { "OK" } else { "FAIL" });
    if !ok_tp { 
        for (l,r,il,ir) in bad_tp { 
            println!("    {} -> {} | {:?} vs {:?}", l, r, il,ir); 
        } 
    }

    let mut rng = StdRng::seed_from_u64(12345);

    let (ok_a, fail_a, ex_a) = fuzz_test(&mut rng, T,  1000, 6, 24, 1, 12);
    println!("\nFuzz T : ok={}  fail={}", ok_a, fail_a);
    for s in ex_a.iter().take(5) {
        println!("  {}", s); 
     }

    let (ok_b, fail_b, ex_b) = fuzz_test(&mut rng, TP, 1000, 6, 24, 1, 12);
    println!("\nFuzz T': ok={}  fail={}", ok_b, fail_b);
    for s in ex_b.iter().take(5) { 
        println!("  {}", s); 
    }
}
