use std::io;
use std::collections::HashMap;

fn recover(input: &String) -> i32 {
		let digits: Vec<_> = input.chars().filter(|char| char.is_digit(10)).collect();
		let result = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
		result.parse::<i32>().unwrap()
}

fn main() {
	let map: HashMap<&str, u32> = HashMap::from([
		("1", 1),
		("2", 2),
		("3", 3),
		("4", 4),
		("5", 5),
		("6", 6),
		("7", 7),
		("8", 8),
		("9", 9),
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9),
	]);
	
	let lines: Vec<_> = io::stdin().lines().map(|line| line.unwrap()).collect();
	let part1: i32 = lines.iter().map(recover).sum();
	println!("Part 1: {}", part1);

	let part2: u32 = lines.iter().map(|line| {
		let length = line.len();
		let mut first: u32 = 0;
		let mut last: u32 = 0;

		// Se deja de ejercicio al lector
		map.iter().map(|(key,value)|
			line.find(key).map(|idx| (value, idx))
		); // Iterator<Option<(Value, Idx>)>> <- min idx

		/*
		let o: Vec<(u32, usize)> = map
                .iter()
                .flat_map(|(k, v)| line.find(k).map(|s| (v.to_owned(), s)))
                .collect();
		
		 */
		// Se deja de ejercicio al lector
		(for (key, value) in map.iter() {
			line.rev().find(key.rev()).map(|idx| (value, idx))
		}); // Iterator<Option<(Value, Idx)>> <- min idx

		first*10 + last
	}).collect::<Vec<_>>().iter().sum();
	println!("Part 2: {}", part2);
}
