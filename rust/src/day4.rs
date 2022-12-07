use std::ops::RangeInclusive;

#[derive(Debug)]
struct AssignmentPair(RangeInclusive<u64>, RangeInclusive<u64>);

#[aoc_generator(day4)]
fn pairs(input: &str) -> Vec<AssignmentPair> {
	input
		.lines()
		.map(|line| line.split_once(',').unwrap())
		.map(|(l, r)| (l.split_once('-').unwrap(), r.split_once('-').unwrap()))
		.map(|((l1, l2), (r1, r2))| {
			AssignmentPair(
				l1.parse().unwrap()..=l2.parse().unwrap(),
				r1.parse().unwrap()..=r2.parse().unwrap(),
			)
		})
		.collect()
}

#[aoc(day4, part1)]
fn fully_containing_pairs(pairs: &[AssignmentPair]) -> usize {
	pairs
		.into_iter()
		.filter(|AssignmentPair(l, r)| {
			(l.contains(r.start()) && l.contains(r.end()))
				|| (r.contains(l.start()) && r.contains(l.end()))
		})
		.count()
}

#[aoc(day4, part2)]
fn overlapping_pairs(pairs: &[AssignmentPair]) -> usize {
	pairs
		.into_iter()
		.filter(|AssignmentPair(l, r)| {
			l.contains(r.start())
				|| l.contains(r.end())
				|| r.contains(l.start())
				|| r.contains(l.end())
		})
		.count()
}
