#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use rand::prelude::*;

pub fn random(minimum: usize, maximum: usize) -> usize
{
	let mut rng = thread_rng();
	return rng.gen_range(minimum..=maximum);
}

pub fn randomMulti(quantity: usize, minimum: usize, maximum: usize) -> Vec<usize>
{
	let mut rng = thread_rng();
	let mut output = vec![];
	
	for _ in 0..quantity
	{
		output.push(rng.gen_range(minimum..=maximum));
	}
	
	return output;
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	#[test]
	fn test_random()
	{
		let min = 1;
		let max = 20;
		let result = random(min, max);
		
		assert!(result >= min);
		assert!(result <= max);
    }
	
	#[test]
	fn test_randomMulti()
	{
		let min = 1;
		let max = 20;
		let quantity = 10;
		let result = randomMulti(quantity, min, max);
		
		assert_eq!(result.len(), quantity);
		assert!(result.iter().sum::<usize>() >= min * quantity);
		assert!(result.iter().sum::<usize>() <= max * quantity);
	}
}
