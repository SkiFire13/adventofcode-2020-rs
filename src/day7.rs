#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = HashMap<&'a str, Vec<(usize, &'a str)>>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let line = line.trim_end_matches('.');
            let (k, rest) = line
                .splitn(2, " bags")
                .collect_tuple()
                .expect("Invalid input");
            let (_, rest) = rest
                .splitn(2, "contain ")
                .collect_tuple()
                .expect("Invalid input");
            let vs = rest
                .split(", ")
                .filter(|&c| c != "no other bags")
                .map(|c| {
                    let c = c.trim_end_matches('s');
                    let c = c.trim_end_matches(" bag");
                    let (n, rest) = c.splitn(2, ' ').collect_tuple().expect("Invalid input");
                    (n.parse().expect("Invalid input"), rest)
                })
                .collect();
            (k, vs)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let bags = input;
    
    fn helper<'a>(
        target: &'a str, 
        bags: &HashMap<&'a str, Vec<(usize, &'a str)>>, 
        cache: &mut HashMap<&'a str, bool>
    ) -> bool {
        if let Some(&cached) = cache.get(&target) {
            return cached;
        }
        let answer = bags[&target]
            .iter()
            .any(|&(_, v)| v == "shiny gold" || helper(v, bags, cache));
        cache.insert(target, answer);
        answer
    }

    let mut cache = HashMap::new();
    bags
        .iter()
        .filter(|&(&bag, _)| helper(bag, bags, &mut cache))
        .count()
}

pub fn part2(input: &Input) -> usize {
    let bags = input;

    fn helper<'a>(
        target: &'a str,
        bags: &HashMap<&'a str, Vec<(usize, &'a str)>>,
        cache: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if let Some(&cached) = cache.get(&target) {
            return cached;
        }
        let answer = bags[target]
            .iter()
            .map(|&(n, v)| n * (1 + helper(v, bags, cache)))
            .sum();
        cache.insert(target, answer);
        answer
    }

    helper("shiny gold", bags, &mut HashMap::new())
}
