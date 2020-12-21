#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<(Vec<&'a str>, Vec<&'a str>)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (ingr, allg) = line
                .split(" (contains ")
                .collect_tuple()
                .unwrap_or((line, ")"));
            let ingr = ingr.split_whitespace().collect();
            let allg = allg.strip_suffix(')').expect("Invalid input");
            let allg = allg.split(", ").collect();
            (ingr, allg)
        })
        .collect()
}

fn find_all_ingrs_and_allgs<'a>(input: &Input<'a>) -> (HashSet<&'a str>, HashSet<&'a str>) {
    let mut all_ingrs = HashSet::new();
    let mut all_allgs = HashSet::new();
    for (ingrs, allgs) in input {
        all_ingrs.extend(ingrs.iter().copied());
        all_allgs.extend(allgs.iter().copied());
    }
    (all_ingrs, all_allgs)
}

fn find_ingr_allgs_candidates<'a>(
    input: &Input<'a>,
    all_ingrs: &HashSet<&'a str>,
    all_allgs: &HashSet<&'a str>,
) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut allg_ingrs = all_allgs
        .iter()
        .copied()
        .map(|ingrs| (ingrs, all_ingrs.clone()))
        .collect::<HashMap<_, _>>();
    for (ingrs, allgs) in input {
        for &allg in allgs {
            let candidate_ingrs = &allg_ingrs[&allg];
            let new_candidate = ingrs
                .iter()
                .copied()
                .filter(|ingr| candidate_ingrs.contains(ingr))
                .collect();
            allg_ingrs.insert(allg, new_candidate);
        }
    }

    allg_ingrs
}

pub fn part1(input: &Input) -> usize {
    let (all_ingrs, all_allgs) = find_all_ingrs_and_allgs(input);
    let allg_ingrs = find_ingr_allgs_candidates(input, &all_ingrs, &all_allgs);

    let mut ingr_allgs = all_ingrs
        .iter()
        .map(|&ingrs| (ingrs, all_allgs.clone()))
        .collect::<HashMap<_, _>>();
    for (ingr, candidate_allgs) in ingr_allgs.iter_mut() {
        candidate_allgs.retain(|allg| allg_ingrs[allg].contains(ingr))
    }

    input
        .iter()
        .flat_map(|(ingrs, _)| ingrs)
        .filter(|&ingr| ingr_allgs[ingr].is_empty())
        .count()
}

pub fn part2(input: &Input) -> String {
    let (all_ingrs, all_allgs) = find_all_ingrs_and_allgs(input);
    let mut allg_ingrs = find_ingr_allgs_candidates(input, &all_ingrs, &all_allgs);

    let mut setted_ingrs = HashSet::new();
    let mut allg_ingr_sorted = BTreeMap::new();
    let mut changed = true;
    while changed {
        changed = false;
        for (&allg, candidate_ingrs) in allg_ingrs.iter_mut() {
            candidate_ingrs.retain(|ingr| !setted_ingrs.contains(ingr));
            if candidate_ingrs.len() == 1 {
                changed = true;
                let ingr = candidate_ingrs.iter().copied().next().unwrap();
                setted_ingrs.insert(ingr);
                allg_ingr_sorted.insert(allg, ingr);
                candidate_ingrs.clear()
            }
        }
    }

    allg_ingr_sorted
        .values()
        .copied()
        .intersperse(",")
        .collect()
}
