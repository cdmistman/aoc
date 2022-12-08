use nom::bytes::complete::tag;
use nom::IResult;

type Stacks = Vec<Vec<char>>;

#[derive(Clone, Copy)]
struct Move {
	count: usize,
	from:  usize,
	to:    usize,
}

fn parse_move_directive(i: &str) -> IResult<&str, Move> {
	fn pusize(i: &str) -> IResult<&str, usize> {
		nom::character::complete::u64(i).map(|(i, n)| (i, n as _))
	}

	let (i, _) = tag("move ")(i)?;
	let (i, count) = pusize(i)?;
	let (i, _) = tag(" from ")(i)?;
	let (i, from) = pusize(i)?;
	let (i, _) = tag(" to ")(i)?;
	let (i, to) = pusize(i)?;
	Ok((i, Move {
		count,
		from: from - 1,
		to: to - 1,
	}))
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Stacks, Vec<Move>) {
	let (stack_section, moves_section) = input.split_once("\n\n").unwrap();

	let mut stack_lines = stack_section.lines()
		// we're iterating each line on a char-by-char basis
		// skip "left columns" ('[' | ' '), "right columns" (inverse),
		// and space columns between stacks.
		.map(|l| l.chars().enumerate().filter_map(|(ii, l)| ((ii - 1) % 4 == 0).then_some(l)))
		.collect::<Vec<_>>();

	let mut stacks = Vec::new();
	loop {
		let stack = stack_lines.iter_mut()
			// right now, the bottom of the stack is at the end of the iterator,
			// but we want it at the start of the vec
			.rev()
			// get the first col of each row
			// if there are no more stacks, this will always result in `None`
			.filter_map(|l| l.next())
			// skip the stack index below the stack
			.skip(1)
			// skip the whitespace above the stack
			.filter(|c| !c.is_whitespace())
			// collect into the stack
			.collect::<Vec<_>>();
		if stack.is_empty() {
			break;
		} else {
			stacks.push(stack);
		}
	}

	let moves = moves_section
		.lines()
		.map(parse_move_directive)
		.map(|r| r.map(|(_, m)| m))
		.collect::<Result<_, _>>()
		.unwrap();

	(stacks, moves)
}

#[aoc(day5, part1)]
fn tops_1x1((stacks, moves): &(Stacks, Vec<Move>)) -> String {
	let mut stacks = stacks.clone();
	for Move { count, from, to } in moves.into_iter().cloned() {
		for _ in 0..count {
			let krate = stacks[from].pop().unwrap();
			stacks[to].push(krate);
		}
	}

	stacks.into_iter().filter_map(|mut s| s.pop()).collect()
}

#[aoc(day5, part2)]
fn tops_stacked((stacks, moves): &(Stacks, Vec<Move>)) -> String {
	let mut stacks = stacks.clone();
	for Move { count, from, to } in moves.into_iter().cloned() {
		let src = &mut stacks[from];
		let src_start = src.len() - count;
		// must unfortunately collect into vec to avoid 2nd mutable borrow/lifetime
		// issues
		let substack = src.drain(src_start..).collect::<Vec<_>>();
		stacks[to].extend(substack);
	}

	stacks.into_iter().filter_map(|mut s| s.pop()).collect()
}
