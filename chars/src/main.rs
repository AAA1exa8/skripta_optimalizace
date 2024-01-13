use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    println!("Hello, world!");
}

fn simple_solution(i: &[u8]) -> usize {
    return i
        .windows(14)
        .position(|w| {
            return w.iter().collect::<HashSet<_>>().len() == 14;
        })
        .map(|x| x + 14)
        .unwrap();
}

fn hash_faster(i: &[u8]) -> usize {
    return i
        .windows(14)
        .position(|w| {
            let mut hash = HashSet::new();
            for x in w {
                if !hash.insert(x) {
                    return false;
                }
            }
            true
        })
        .map(|x| x + 14)
        .unwrap();
}

fn vec_solution(i: &[u8]) -> usize {
    return i
        .windows(14)
        .position(|w| {
            let mut vec = Vec::with_capacity(14);
            for x in w {
                if vec.contains(x) {
                    return false;
                }
                vec.push(*x);
            }
            true
        })
        .map(|x| x + 14)
        .unwrap();
}

fn arr_solution(i: &[u8]) -> usize {
    return i
        .windows(14)
        .position(|w| {
            let mut arr = [0u8; 14];
            let mut idx = 0;
            for x in w {
                for i in 0..idx {
                    if arr[i] == *x {
                        return false;
                    }
                }
                arr[idx] = *x;
                idx += 1;
            }
            true
        })
        .map(|x| x + 14)
        .unwrap();
}

pub fn benny(input: &[u8]) -> Option<usize> {
    let mut filter = 0u32;
    input
        .iter()
        .take(14 - 1)
        .for_each(|c| filter ^= 1 << (c % 32));
    input.windows(14).position(|w| {
        let first = w[0];
        let last = w[w.len() - 1];
        filter ^= 1 << (last % 32);
        let res = filter.count_ones() == 14 as _;
        filter ^= 1 << (first % 32);
        res
    })
}

pub fn nerd_face(input: &[u8]) -> Option<usize> {
    let mut idx = 0;
    while let Some(slice) = input.get(idx..idx + 14) {
        let mut state = 0u32;

        if let Some(pos) = slice.iter().rposition(|byte| {
            let bit_idx = byte % 32;
            let ret = state & (1 << bit_idx) != 0;
            state |= 1 << bit_idx;
            ret
        }) {
            idx += pos + 1;
        } else {
            return Some(idx);
        }
    }
    return None;
}
