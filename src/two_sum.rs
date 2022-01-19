#![allow(unused)]

use std::{collections::HashMap, ops::Sub};

pub fn two_sum(array: &[i32], target: i32) -> Vec<i32> {
    // Build hashmap.
    let mut hs: HashMap<i32, usize> = HashMap::new();
    for (i,v) in array.iter().enumerate() {
        hs.insert(v-target, i);
    }

    // Build vector to return
    for v in array.iter() {
        match hs.get(v) {
            Some(i) => {
                let other = array.get(*i).expect("");
                return vec![*v, *other];
            },
            _ => {
                continue
            },
        }
    };

    vec![]
}
