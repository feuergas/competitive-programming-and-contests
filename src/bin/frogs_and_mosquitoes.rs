// use std::collections::BTreeSet;
use std::{collections::BTreeSet, io::Read, ops::Bound::Included};

struct Solution {}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Frog {
    tongue: u64,
    pos: u64,
    eaten_mosquitoes: u64,
    index: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Mosquito {
    pos: u64,
    size: u64,
    index: usize,
}

fn get_input(it: &mut std::str::SplitWhitespace<'_>) -> (Vec<Frog>, Vec<Mosquito>) {
    let n: u64 = it.next().unwrap().parse().unwrap();
    let m: u64 = it.next().unwrap().parse().unwrap();

    let frogs: Vec<Frog> = (0..n)
        .map(|idx| {
            let pos: u64 = it.next().unwrap().parse().unwrap();
            let tongue: u64 = it.next().unwrap().parse().unwrap();
            Frog {
                pos,
                tongue: tongue + pos,
                eaten_mosquitoes: 0,
                index: idx as usize,
            }
        })
        .collect();

    let mosquitoes: Vec<Mosquito> = (0..m)
        .map(|idx| {
            let pos: u64 = it.next().unwrap().parse().unwrap();
            let size: u64 = it.next().unwrap().parse().unwrap();
            Mosquito {
                pos,
                size,
                index: idx as usize,
            }
        })
        .collect();

    (frogs, mosquitoes)
}

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it: std::str::SplitWhitespace<'_> = input.split_whitespace();

    // let t: u64 = it.next().unwrap().parse().unwrap();
    let t: u64 = 1;

    for _ in 0..t {
        let (frogs, mosquitoes) = get_input(&mut it);

        let sol: Vec<(u64, u64)> = Solution::frogs_and_mosquitoes(frogs, mosquitoes);

        for val in sol.iter() {
            println!("{} {}", val.0, val.1);
        }
    }
}

impl Solution {
    pub fn frogs_and_mosquitoes(frogs: Vec<Frog>, mosquitoes: Vec<Mosquito>) -> Vec<(u64, u64)> {
        let mut frogs: BTreeSet<Frog> = frogs.into_iter().collect();
        let mut landed_mosquitoes: BTreeSet<Mosquito> = BTreeSet::new();

        for mosq in mosquitoes {
            let frog: Option<Frog> = frogs
                .range(
                    Frog {
                        pos: 0,
                        tongue: mosq.pos,
                        eaten_mosquitoes: 0,
                        index: 0,
                    }..,
                )
                .next()
                .filter(|frog: &&Frog| frog.pos <= mosq.pos)
                .cloned();

            if let Some(frog) = frog {
                // Frog eats the mosquito
                frogs.remove(&frog);
                let mut updated_frog: Frog = frog.clone();
                updated_frog.tongue += mosq.size;
                updated_frog.eaten_mosquitoes += 1;

                // Frog keeps eating mosquitoes that are within its tongue range
                while let Some(l_mosq) = landed_mosquitoes
                    .range((
                        Included(Mosquito {
                            pos: updated_frog.pos,
                            size: 0,
                            index: 0,
                        }),
                        Included(Mosquito {
                            pos: updated_frog.tongue,
                            size: u64::MAX,
                            index: usize::MAX,
                        }),
                    ))
                    .next()
                    .cloned()
                {
                    landed_mosquitoes.remove(&l_mosq);
                    updated_frog.tongue += l_mosq.size;
                    updated_frog.eaten_mosquitoes += 1;
                }

                // Insert the updated frog back into the set
                frogs.insert(updated_frog);
            } else {
                landed_mosquitoes.insert(mosq);
            }
        }

        let mut frogs: Vec<Frog> = frogs.into_iter().collect();

        frogs.sort_by_key(|frog: &Frog| frog.index);

        frogs
            .into_iter()
            .map(|frog: Frog| (frog.eaten_mosquitoes, frog.tongue - frog.pos))
            .collect()
    }
}
