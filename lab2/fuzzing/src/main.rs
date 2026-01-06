use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use regex::Regex;
use std::collections::HashSet;


fn accepts_regex(re: &Regex, word: &str) -> bool {
    re.is_match(word)
}

// (?=[abc]*$) в Rust отсутсвует - использую is_abc_only
fn is_abc_only(word: &str) -> bool {
    word.chars().all(|c| c == 'a' || c == 'b' || c == 'c')
}

fn accepts_ext_regex(re: &Regex, word: &str) -> bool {
    if !is_abc_only(word) {
        return false;
    }
    re.is_match(word)
}



const NFA_START: usize = 0;
const NFA_ACCEPT: [usize; 3] = [5, 9, 14];

fn nfa_eps(state: usize) -> &'static [usize] {
    match state {
        0 => &[1],
        _ => &[],
    }
}

fn nfa_delta(state: usize, ch: char) -> &'static [usize] {
    match (state, ch) {
        (0, 'a') => &[12], 
        (12, 'b') => &[13],
        (13, 'c') => &[14],

        (1, 'a') => &[1, 2],
        (1, 'b') => &[1, 6],
        (1, 'c') => &[1],

        (2, 'b') => &[3],       

        (3, 'a') => &[3],
        (3, 'c') => &[3],
        (3, 'b') => &[3, 4],    

        (4, 'c') => &[5],      

        (5, 'a') | (5, 'b') | (5, 'c') => &[5],

        (6, 'c') => &[7],       

        (7, 'b') => &[7],
        (7, 'c') => &[7],
        (7, 'a') => &[7, 8],   

        (8, 'b') => &[9],      

        (9, 'a') => &[9],      
        (9, 'b') => &[10],      
        (9, 'c') => &[11],     

        (10, 'b') => &[9],      
        (10, 'c') => &[9],      
        (11, 'c') => &[9],     

        _ => &[],
    }
}

fn epsilon_closure(states: &HashSet<usize>) -> HashSet<usize> {
    let mut closure = states.clone();
    let mut stack: Vec<usize> = states.iter().copied().collect();

    while let Some(s) = stack.pop() {
        for &t in nfa_eps(s) {
            if closure.insert(t) {
                stack.push(t);
            }
        }
    }

    closure
}

fn accepts_nfa(word: &str) -> bool {
    let mut start = HashSet::new();
    start.insert(NFA_START);
    let mut current = epsilon_closure(&start);

    for ch in word.chars() {
        let mut next = HashSet::new();
        for &s in &current {
            for &t in nfa_delta(s, ch) {
                next.insert(t);
            }
        }
        if next.is_empty() {
            current.clear();
            break;
        }
        current = epsilon_closure(&next);
    }

    current.iter().any(|s| NFA_ACCEPT.contains(s))
}

fn dfa_delta(state: usize, ch: char) -> usize {
    match (state, ch) {
        (0, 'a') => 1,
        (0, 'b') => 2,
        (0, 'c') => 3,

        (1, 'a') => 4,
        (1, 'b') => 5,
        (1, 'c') => 3,

        (2, 'a') => 4,
        (2, 'b') => 2,
        (2, 'c') => 6,

        (3, 'a') => 4,
        (3, 'b') => 2,
        (3, 'c') => 3,

        (4, 'a') => 4,
        (4, 'b') => 7,
        (4, 'c') => 3,

        (5, 'a') => 8,
        (5, 'b') => 9,
        (5, 'c') => 10,

        (6, 'a') => 11,
        (6, 'b') => 6,
        (6, 'c') => 6,

        (7, 'a') => 8,
        (7, 'b') => 9,
        (7, 'c') => 12,

        (8, 'a') => 8,
        (8, 'b') => 9,
        (8, 'c') => 8,

        (9, 'a') => 8,
        (9, 'b') => 9,
        (9, 'c') => 13,

        (10, 'a') => 14,
        (10, 'b') => 15,
        (10, 'c') => 12,

        (11, 'a') => 11,
        (11, 'b') => 16,
        (11, 'c') => 6,

        (12, 'a') => 14,
        (12, 'b') => 15,
        (12, 'c') => 12,

        (13, 'a') => 13,
        (13, 'b') => 13,
        (13, 'c') => 13,

        (14, 'a') => 14,
        (14, 'b') => 17,
        (14, 'c') => 12,

        (15, 'a') => 14,
        (15, 'b') => 15,
        (15, 'c') => 13,

        (16, 'a') => 18,
        (16, 'b') => 19,
        (16, 'c') => 20,

        (17, 'a') => 18,
        (17, 'b') => 19,
        (17, 'c') => 13,

        (18, 'a') => 18,
        (18, 'b') => 21,
        (18, 'c') => 20,

        (19, 'a') => 14,
        (19, 'b') => 17,
        (19, 'c') => 13,

        (20, 'a') => 14,
        (20, 'b') => 15,
        (20, 'c') => 16,

        (21, 'a') => 18,
        (21, 'b') => 21,
        (21, 'c') => 13,

        _ => panic!("нет перехода для состояния {} и символа {}", state, ch),
    }
}

fn accepts_dfa(word: &str) -> bool {
    let mut state: usize = 0;

    for ch in word.chars() {
        state = dfa_delta(state, ch);
    }

    matches!(state, 10 | 13 | 16 | 17 | 18 | 21)
}

fn accepts_afa(word: &str) -> bool {
    accepts_nfa(word) && word.contains("ab")
}

fn random_word(rng: &mut StdRng, max_len: usize) -> String {
    let len = rng.random_range(0..=max_len);
    let alphabet = ['a', 'b', 'c'];

    (0..len)
        .map(|_| {
            let i = rng.random_range(0..alphabet.len());
            alphabet[i]
        })
        .collect()
}


fn main() {
    let re_base = Regex::new(
        r"^((a*b*c*)*ab(a*b*c*)*bc(a|b|c)*|(a|b|c)*bc(a*b*c*)*ab(a|bc|cc|bb)*|abc)$",
    )
    .unwrap();

    // расширенная — без lookahead, он вынесен в is_abc_only
    let re_ext = Regex::new(r"^(.*ab.*bc.*|.*bc.*ab(a|bc|cc|bb)*|abc)$").unwrap();

    let num_tests = 20_000;
    let max_len = 8;

    let mut rng = StdRng::seed_from_u64(0);

    for i in 0..num_tests {
        let w = random_word(&mut rng, max_len);

        let r1 = accepts_regex(&re_base, &w);
        let r2 = accepts_ext_regex(&re_ext, &w);
        let r3 = accepts_nfa(&w);
        let r4 = accepts_dfa(&w);
        let r5 = accepts_afa(&w);

        if !(r1 == r2 && r2 == r3 && r3 == r4 && r4 == r5) {
            println!("Расхождение на слове {:?} (тест #{})", w, i);
            println!("регулярка     : {}", r1);
            println!("расширенная регулярка : {}", r2);
            println!("НКА                   : {}", r3);
            println!("ДКА                   : {}", r4);
            println!("ПКА                   : {}", r5);
            return;
        }
    }

    println!(
        "{} случайных слов, расхождений не найдено.",
        num_tests
    );
}
