use std::fmt::Debug;

#[derive(Debug)]
struct History<const CACHE: usize, T> {
	next_ii: usize,
	cache:   [T; CACHE],
}

impl<const CACHE: usize, T: Debug> History<CACHE, T> {
	fn new(cache: [T; CACHE]) -> Self {
		Self { next_ii: 0, cache }
	}

	fn contains(&self, value: T) -> bool
	where
		T: Eq + Copy,
	{
		for i in 0..CACHE {
			if self.cache[i] == value {
				return true;
			}
		}
		false
	}

	fn shift(&mut self, value: T) {
		debug_assert!(self.next_ii < CACHE);

		let i = self.next_ii;
		self.cache[i] = value;

		if self.next_ii == CACHE - 1 {
			self.next_ii = 0;
		} else {
			self.next_ii += 1;
		}
	}

	fn is_set(&self) -> bool
	where
		T: Eq,
	{
		let mut vals = self.cache.iter();
		while let Some(val) = vals.next() {
			if vals.clone().any(|v| v == val) {
				return false;
			}
		}
		true
	}
}

#[inline(always)]
fn find_marker<const MARKER_SIZE_MINUS_ONE: usize>(input: &str) -> Option<usize> {
	let mut input = input.chars().enumerate();
	let mut history = History::new(
		(&mut input)
			.take(MARKER_SIZE_MINUS_ONE)
			.map(|(_, ch)| ch)
			.array_chunks::<MARKER_SIZE_MINUS_ONE>()
			.next()
			.unwrap(),
	);
	while let Some((ii, ch)) = input.next() {
		if !history.contains(ch) && history.is_set() {
			return Some(ii + 1);
		}
		history.shift(ch);
	}
	None
}

#[aoc(day6, part1)]
fn find_start_packet_marker(input: &str) -> usize {
	find_marker::<3>(input).expect("no start-of-packet marker")
}

#[aoc(day6, part2)]
fn find_start_message_marker(input: &str) -> usize {
	find_marker::<13>(input).expect("no start-of-message marker")
}
