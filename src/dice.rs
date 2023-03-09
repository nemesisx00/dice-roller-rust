#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use crate::num::randomMulti;

const MinimumDieSides: usize = 2;
const MinimumDieValue: usize = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Die
{
	sides: usize,
}

impl Default for Die
{
	fn default() -> Self
	{
		return Self { sides: MinimumDieSides };
	}
}

impl Die
{
	pub fn new(sides: usize) -> Self
	{
		return Die { sides };
	}
	
	/// Roll this die an arbitrary number of times and return a <code>Roll</code>.
	/// 
	/// Parameters:
	/// * <code>quantity</code> The number of times to roll this <code>Die</code>.
	pub fn roll(self, quantity: usize) -> Roll
	{
		let mut roll = Roll::default();
		roll.die = self;
		roll.values = randomMulti(quantity, MinimumDieValue, self.sides);
		roll.calculate();
		return roll;
	}
	
	/// Generate a string of the format <code>d{sides}</code>.
	pub fn to_string(self) -> String
	{
		return format!("d{0}", self.sides).to_string();
	}
}

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
#[readonly::make]
pub struct Roll
{
	pub die: Die,
	pub highest: usize,
	pub lowest: usize,
	pub total: usize,
	pub values: Vec<usize>,
}

impl Roll
{
	pub fn new(die: Die, highest: usize, lowest: usize, total: usize, values: Vec<usize>) -> Self
	{
		return Self { die, highest, lowest, total, values };
	}
	
	pub fn calculate(&mut self)
	{
		self.highest = *self.values.iter().max().unwrap();
		self.lowest = *self.values.iter().min().unwrap();
		self.total = self.values.iter().sum();
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn rollDiceTest(quantity: usize)
	{
		let max = 20;
		
		let die = Die { sides: max };
		let result = die.roll(quantity);
		
		assert_eq!(result.values.len(), quantity);
		assert!(result.total >= MinimumDieValue * quantity);
		assert!(result.total <= max * quantity);
    }
	
	#[test]
	fn test_Die_toString()
	{
		let die = Die { sides: 20 };
		let result = die.to_string();
		let expected = "d20".to_owned();
		
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test_Die_roll_1() { rollDiceTest(1); }
	
	#[test]
	fn test_Die_roll_10() { rollDiceTest(10); }
	
	#[test]
	fn test_Roll()
	{
		let mut result = Roll::default();
		result.values = vec![1,2,3,4];
		result.calculate();
		
		let expected = 10;
		
		assert_eq!(result.total, expected);
	}
}
