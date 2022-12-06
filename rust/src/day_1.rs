use eyre::Result;

#[aoc_generator(day1)]
fn count_calories(input: &str) -> Vec<u64> {
	let mut lines = input.lines();
	let mut elves = Vec::new();
	loop {
		let mut elf_calories = Vec::new();
		while let Some(line) = lines.next() {
			if line.is_empty() {
				break;
			}
			elf_calories.push(line.parse::<u64>().unwrap())
		}
		if elf_calories.is_empty() {
			break;
		}
		elves.push(elf_calories.into_iter().sum());
	}
	elves
}

#[aoc(day1, part1)]
pub fn max_calories(elves: &Vec<u64>) -> Option<u64> {
	elves.clone().into_iter().max()
}

#[aoc(day1, part2)]
pub fn max_3_calories(elves: &Vec<u64>) -> Option<u64> {
	let mut elves = elves.clone();
	elves.sort();
	let a = elves.pop()?;
	let b = elves.pop()?;
	let c = elves.pop()?;
	Some(a + b + c)
}
