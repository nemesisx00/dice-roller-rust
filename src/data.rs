#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use std::cell::RefCell;
use std::collections::BTreeMap;
use crate::dice::{Die, Roll};

const AdditionSpacer: &str = " + ";

#[derive(Debug, Default)]
pub struct Equation
{
	data: RefCell<BTreeMap<Die, usize>>,
}

impl Clone for Equation
{
	fn clone(&self) -> Self
	{
		let instance = Equation::default();
		for (die, quantity) in self.data.borrow().iter()
		{
			instance.set(die.to_owned(), quantity.to_owned());
		}
		return instance;
	}
	
	fn clone_from(&mut self, source: &Self)
	{
		let mut data = self.data.borrow_mut();
		data.clear();
		
		for (die, quantity) in source.read().iter()
		{
			data.insert(die.to_owned(), quantity.to_owned());
		}
	}
}

impl From<&Equation> for Equation
{
	fn from(value: &Self) -> Self
	{
		let instance = Self::default();
		for (die, quantity) in value.read().iter()
		{
			instance.set(die.to_owned(), quantity.to_owned());
		}
		return instance;
	}
}

impl Equation
{
	pub fn add(&self, die: Die)
	{
		let mut data = self.data.borrow_mut();
		if let Some(value) = data.get_mut(&die)
		{
			*value += 1;
		}
		else
		{
			data.insert(die, 1);
		}
	}
	
	pub fn clear(&self)
	{
		self.data.borrow_mut().clear();
	}
	
	pub fn count(&self, die: Die) -> usize
	{
		return match self.data.borrow().get(&die)
		{
			Some(quantity) => *quantity,
			None => 0,
		};
	}
	
	pub fn read(&self) -> BTreeMap<Die, usize>
	{
		return self.data.borrow().clone();
	}
	
	pub fn set(&self, die: Die, quantity: usize)
	{
		self.data.borrow_mut().insert(die, quantity);
	}
	
	pub fn subtract(&self, die: Die)
	{
		let mut data = self.data.borrow_mut();
		if let Some(value) = data.get_mut(&die)
		{
			*value -= 1;
			
			if *value < 1
			{
				data.remove(&die);
			}
		}
	}
	
	pub fn to_string(self) -> String
	{
		let mut eq = String::default();
		for (d, n) in self.data.borrow().iter()
		{
			if eq.len() > 0
			{
				eq.push_str(AdditionSpacer);
			}
			
			eq.push_str(format!("{0}{1}", n, d.to_string()).as_str());
		}
		return eq;
	}
}

// --------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct RollResult
{
	rolls: RefCell<Vec<Roll>>,
}

impl From<Equation> for RollResult
{
	fn from(value: Equation) -> Self
	{
		let result = Self::default();
		for (die, quantity) in value.read()
		{
			result.rolls.borrow_mut().push(die.roll(quantity));
		}
		return result;
	}
}

impl From<&Equation> for RollResult
{
	fn from(value: &Equation) -> Self
	{
		let result = Self::default();
		for (die, quantity) in value.read()
		{
			result.rolls.borrow_mut().push(die.roll(quantity));
		}
		return result;
	}
}

impl RollResult
{
	pub fn add(&self, roll: Roll)
	{
		self.rolls.borrow_mut().push(roll);
	}
	
	pub fn get(&self, die: Die) -> Option<Roll>
	{
		let rolls = self.rolls.borrow();
		
		return match rolls.iter().position(|r| r.die == die)
		{
			Some(index) => Some(rolls.get(index).unwrap().to_owned()),
			None => None,
		};
	}
	
	pub fn total(&self) -> usize
	{
		return self.rolls.borrow().iter()
			.fold(0, |acc, val| acc + val.total);
	}
	
	pub fn to_string(&self) -> String
	{
		return format!(
			"{0} -> {1} = {2}",
			self.toValueString(),
			self.toIntermediateString(),
			self.total()
		).to_string();
	}
	
	pub fn toIntermediateString(&self) -> String
	{
		return self.rolls.borrow().iter()
			.fold(String::default(), |acc, roll| {
				let mut s = acc.to_string();
				if s.len() > 0 { s.push_str(AdditionSpacer); }
				s.push_str(roll.total.to_string().as_str());
				return s;
			});
	}
	
	pub fn toValueString(&self) -> String
	{
		return self.rolls.borrow().iter()
			.fold(String::default(), |acc, roll| {
				let mut s = acc.to_string();
				if s.len() > 0 { s.push_str(AdditionSpacer); }
				
				let values = roll.values.iter()
					.fold("".to_string(), |acc, value| {
						let mut s = acc.to_string();
						if s.len() > 0 { s.push_str(", "); }
						s.push_str(value.to_string().as_str());
						return s;
					});
				
				s.push_str(format!("[{values}]").as_str());
				return s;
			});
	}
}

// --------------------------------------------------

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildEquation() -> Equation
	{
		let d4 = Die::new(4);
		let d6 = Die::new(6);
		
		let data = Equation::default();
		data.add(d4);
		data.add(d6);
		data.add(d6);
		data.add(d6);
		data.add(d6);
		data.add(d6);
		
		return data;
	}
	
	fn buildRollResult() -> RollResult
	{
		let d4 = Die::new(4);
		let d6 = Die::new(6);
		let data = RollResult::default();
		data.add(Roll::new(d4, 4, 2, 6, vec![4, 2]));
		data.add(Roll::new(d6, 6, 3, 9, vec![3, 6]));
		
		return data;
	}
	
	#[test]
	fn test_Equation_add()
	{
		let d4 = Die::new(4);
		let data = Equation::default();
		data.add(d4);
		
		let expected = 1;
		let result = data.count(d4);
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test_Equation_clear()
	{
		let data = buildEquation();
		data.clear();
		
		assert_eq!(data.read().len(), 0);
	}
	
	#[test]
	fn test_Equation_count()
	{
		let d4 = Die::new(4);
		let d6 = Die::new(6);
		let data = buildEquation();
		
		assert_eq!(data.count(d4), 1);
		assert_eq!(data.count(d6), 5);
	}
	
	#[test]
	fn test_Equation_read()
	{
		let d4 = Die::new(4);
		let d6 = Die::new(6);
		let data = buildEquation();
		
		let mut result = data.read();
		*result.get_mut(&d6).unwrap() = 9;
		data.add(d4);
		
		assert_ne!(result.get(&d4), data.read().get(&d4));
		assert_ne!(result.get(&d6), data.read().get(&d6));
	}
	
	#[test]
	fn test_Equation_set()
	{
		let d4 = Die::new(4);
		let data = buildEquation();
		let before = data.count(d4);
		
		data.set(d4, 4);
		let after = data.count(d4);
		
		assert_ne!(before, after);
	}
	
	#[test]
	fn test_Equation_subtract()
	{
		let d4 = Die::new(4);
		let d6 = Die::new(6);
		let data = buildEquation();
		
		let mut expected = data.count(d4) - 1;
		data.subtract(d4);
		
		let mut result = data.count(d4);
		assert_eq!(result, expected);
		
		expected = data.count(d6) - 1;
		data.subtract(d6);
		result = data.count(d6);
		
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test_Equation_toString()
	{
		let data = buildEquation();
		let expected = "1d4 + 5d6".to_string();
		let result = data.to_string();
		
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test_RollResult_from_Equation()
	{
		let d4 = Die::new(4);
		let data = buildEquation();
		let result = RollResult::from(data);
		
		assert_ne!(result.get(d4), None);
	}
	
	#[test]
	fn test_RollResult_get()
	{
		let d4 = Die::new(4);
		let instance = buildRollResult();
		let result = instance.get(d4).unwrap();
		
		assert_eq!(result.die, d4);
		assert_eq!(result.highest, 4);
		assert_eq!(result.lowest, 2);
		assert_eq!(result.total, 6);
		assert_eq!(result.values, vec![4, 2]);
	}
	
	#[test]
	fn test_RollResult_total()
	{
		let instance = buildRollResult();
		let result = instance.total();
		
		assert_eq!(result, 15);
	}
	
	#[test]
	fn test_RollResult_to_string()
	{
		let instance = buildRollResult();
		let result = instance.to_string();
		let expected = "[4, 2] + [3, 6] -> 6 + 9 = 15".to_string();
		
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test_RollResult_toIntermediateString()
	{
		let instance = buildRollResult();
		let result = instance.toIntermediateString();
		let expected = "6 + 9".to_string();
		
		assert_eq!(result, expected);
	}
	
	#[test]
	fn test_RollResult_toValueString()
	{
		let instance = buildRollResult();
		let result = instance.toValueString();
		let expected = "[4, 2] + [3, 6]".to_string();
		
		assert_eq!(result, expected);
	}
}
