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

		for i in 0..length {
			let slice = line.get(i..).unwrap();
			for key in map.keys() {
				if slice.starts_with(key) {
					first = *map.get(key).unwrap();
					break;
				}
			}
		}

		for i in length..0 {

			let slice = line.get(i..).unwrap();
			for key in map.keys() {
				if slice.starts_with(key) {
					last = *map.get(key).unwrap();
					break;
				}
			}
		}

		first*10 + last
	}).collect::<Vec<_>>().iter().sum();
	println!("Part 2: {}", part2);
}
