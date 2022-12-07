use std::collections::HashSet;

fn ch_to_priority(ch: char) -> u8 {
	1 + [
		'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
		's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
		'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
	]
	.into_iter()
	.enumerate()
	.find_map(|p| (p.1 == ch).then_some(p.0))
	.unwrap() as u8
}

#[aoc(day3, part1)]
fn rucksack_priorities(input: &str) -> u64 {
	let rucksacks = input
		.lines()
		.map(|line| line.split_at(line.len() / 2))
		.map(|(c1, c2)| {
			(
				c1.chars().collect::<HashSet<_>>(),
				c2.chars().collect::<HashSet<_>>(),
			)
		});

	let mut sum = 0;
	for (compartment1, compartment2) in rucksacks {
		let mut items = compartment1.intersection(&compartment2);
		let Some(item) = items.next() else {
			panic!("nothing in common between rucksacks `{compartment1:?}` and `{compartment2:?}`");
		};
		if items.next().is_some() {
			panic!(
				"too many items in common between rucksacks `{compartment1:?}` and \
				 `{compartment2:?}`"
			);
		}
		sum += ch_to_priority(*item) as u64;
	}
	sum
}

#[aoc(day3, part2)]
fn group_priorities(input: &str) -> u64 {
	let groups = input.lines().array_chunks::<3>().map(|[a, b, c]| {
		(
			a.chars().collect::<HashSet<_>>(),
			b.chars().collect::<HashSet<_>>(),
			c.chars().collect::<HashSet<_>>(),
		)
	});
	let mut sum = 0;

	for (member1, member2, member3) in groups {
		let badges = member1
			.intersection(&member2)
			.cloned()
			.collect::<HashSet<_>>()
			.intersection(&member3)
			.cloned()
			.collect::<Vec<_>>();

		if badges.is_empty() {
			panic!(
				"group does not have a badge!
					- {member1:?}
					- {member2:?}
					- {member3:?}"
			);
		} else if badges.len() > 1 {
			panic!(
				"group has too many badges!
					- {member1:?}
					- {member2:?}
					- {member3:?}"
			)
		} else {
			sum += ch_to_priority(badges[0]) as u64;
		}
	}
	sum
}
