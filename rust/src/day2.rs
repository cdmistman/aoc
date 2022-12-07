use Col1::*;
use Col2::*;
use Move::*;
use RoundResult::*;

enum Col1 {
	A,
	B,
	C,
}

enum Col2 {
	X,
	Y,
	Z,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Move {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

impl Move {
	fn score(self, result: RoundResult) -> u8 {
		self as u8 + result as u8
	}

	fn vs(self, other: Move) -> RoundResult {
		match (self, other) {
			(Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
			(Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Loss,
			(Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
		}
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum RoundResult {
	Loss = 0,
	Draw = 3,
	Win = 6,
}

#[aoc_generator(day2)]
fn decrypt_guide(guide: &str) -> Vec<(Col1, Col2)> {
	guide
		.lines()
		.map(|round_line| {
			let mut chars = round_line.chars();
			let col1 = match chars.next().unwrap() {
				'A' => A,
				'B' => B,
				'C' => C,
				invalid => unreachable!("can't decrypt my move `{invalid}`"),
			};
			chars.next().unwrap();
			let col2 = match chars.next().unwrap() {
				'X' => X,
				'Y' => Y,
				'Z' => Z,
				invalid => unreachable!("can't decrypt opponent's move `{invalid}`"),
			};
			(col1, col2)
		})
		.collect()
}

#[aoc(day2, part1)]
fn calculate_score(guide: &[(Col1, Col2)]) -> u64 {
	let mut total_score = 0u64;
	for (col1, col2) in guide {
		let opp_move = match col1 {
			A => Rock,
			B => Paper,
			C => Scissors,
		};
		let my_move = match col2 {
			X => Rock,
			Y => Paper,
			Z => Scissors,
		};
		let result = my_move.vs(opp_move);
		total_score += my_move.score(result) as u64;
	}
	total_score
}

#[aoc(day2, part2)]
fn calculate_actual_score(guide: &[(Col1, Col2)]) -> u64 {
	let mut total_score = 0u64;
	for (col1, col2) in guide {
		let opp_move = match col1 {
			A => Rock,
			B => Paper,
			C => Scissors,
		};
		let desired_result = match col2 {
			X => Loss,
			Y => Draw,
			Z => Win,
		};
		let my_move = match (desired_result, opp_move) {
			(Draw, Rock) | (Loss, Paper) | (Win, Scissors) => Rock,
			(Win, Rock) | (Draw, Paper) | (Loss, Scissors) => Paper,
			(Loss, Rock) | (Win, Paper) | (Draw, Scissors) => Scissors,
		};
		total_score += my_move.score(desired_result) as u64;
	}
	total_score
}
