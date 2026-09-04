// use std::collections::BTreeSet;
use std::{collections::BTreeSet, io::Read};

fn get_input(it: &mut std::str::SplitWhitespace<'_>) -> (Vec<(u64, u64)>, Vec<(u64, u64)>) {
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let frogs: Vec<(u64, u64)> = (0..n)
        .map(|_| {
            let pos: u64 = it.next().unwrap().parse().unwrap();
            let tongue: u64 = it.next().unwrap().parse().unwrap();
            (pos, tongue)
        })
        .collect();

    let mosquitoes: Vec<(u64, u64)> = (0..m)
        .map(|_| {
            let pos: u64 = it.next().unwrap().parse().unwrap();
            let size: u64 = it.next().unwrap().parse().unwrap();
            (pos, size)
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

        for val in sol {
            println!("{} {}", val.0, val.1);
        }
    }
}

struct Solution {}

impl Solution {
    pub fn frogs_and_mosquitoes(
        frogs: Vec<(u64, u64)>,
        mosquitoes: Vec<(u64, u64)>,
    ) -> Vec<(u64, u64)> {
        let mut answers: Vec<(u64, u64)> = frogs.iter().map(|&(_, tongue)| (0, tongue)).collect();
        // store the right enpoints of the eating range for each frog
        let mut frog_ranges: Vec<(u64, Option<usize>)> = frogs
            .iter()
            .enumerate()
            .map(|(idx, &(pos, tongue))| (pos + tongue, Some(idx)))
            .collect();
        // sort the right endpoints by increasing left endpoints
        frog_ranges.sort_by_key(|&(_, idx)| frogs[idx.unwrap()]);
        // remove the right endpoints of the dominated frogs
        let mut max_range = u64::MIN;
        frog_ranges.retain(|&(range, _)| {
            if range <= max_range {
                false
            } else {
                max_range = range;
                true
            }
        });
        // convert frog ranges to ordered set
        let mut frog_ranges: BTreeSet<(u64, Option<usize>)> = frog_ranges.into_iter().collect();
        /* SEEMS WORKING TILL HERE */
        // store yet to be eaten mosquitoes in an ordered set
        let mut landed_mosquitoes: BTreeSet<(u64, Option<usize>)> = BTreeSet::new();

        for (curr_idx, &(curr_pos, curr_size)) in mosquitoes.iter().enumerate() {
            let frog_query = frog_ranges
                .range((curr_pos, None)..)
                .next()
                .clone();

            let mut ate_mosquito = false;

            if let Some(&(mut curr_range, idx)) = frog_query {
                let idx = idx.unwrap();

                if frogs[idx].0 <= curr_pos {
                    frog_ranges.remove(&(curr_range, Some(idx)));
                    ate_mosquito = true;
                    let old_range = curr_range;
                    // eat current mosquito
                    answers[idx].0 += 1;
                    answers[idx].1 += curr_size;
                    curr_range += curr_size;

                    let frog_pos = frogs[idx].0;
                    // keep eating landed mosquitoes in reach
                    while let Some(new_mosq) = landed_mosquitoes
                        .range((frog_pos, None)..(curr_range + 1, None))
                        .next()
                        .copied()
                    {
                        landed_mosquitoes.remove(&new_mosq);
                        let new_mosq_idx = new_mosq.1.unwrap();
                        // eat new mosquito
                        answers[idx].0 += 1;
                        answers[idx].1 += mosquitoes[new_mosq_idx].1;
                        curr_range += mosquitoes[new_mosq_idx].1;
                    }

                    // remove newly dominated frogs
                    while let Some(sub_frog) = frog_ranges
                        .range((old_range, None)..(curr_range + 1, None))
                        .next()
                        .copied()
                    {
                        frog_ranges.remove(&sub_frog);
                    }

                    // add back the eating frog
                    frog_ranges.insert((frog_pos + answers[idx].1, Some(idx)));
                }
            }

            // If no frog could eat it, add it to landed mosquitoes
            if !ate_mosquito {
                landed_mosquitoes.insert((curr_pos, Some(curr_idx)));
            }
        }

        answers
    }
}
