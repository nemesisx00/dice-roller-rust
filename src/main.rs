#![allow(non_snake_case, non_upper_case_globals)]

mod data;
mod dice;
mod num;

slint::include_modules!();

use std::borrow::Borrow;
use std::rc::Rc;
use crate::data::{Equation, RollResult};
use crate::dice::Die;

fn main()
{
	let equation = Rc::new(Equation::default());
	let mainWindow = MainWindow::new();
	
	mainWindow.on_incrementDie({
		let winWeak = mainWindow.as_weak();
		let eqWeak = Rc::downgrade(&equation);
		move |value| {
			let sides = value as usize;
			let die = Die::new(sides);
			if let Some(eq) = eqWeak.upgrade()
			{
				eq.add(die);
				winWeak.unwrap()
					.set_equation(Equation::from(eq.borrow()).to_string().into());
			}
		}
	});
	
	mainWindow.on_doRoll({
		let winWeak = mainWindow.as_weak();
		let eqWeak = Rc::downgrade(&equation);
		move || {
			if let Some(eq) = eqWeak.upgrade()
			{
				let win = winWeak.unwrap();
				
				let result = RollResult::from(eq.borrow());
				win.set_result(result.to_string().into());
				
				//TODO: Figure out an intuitive means of identifying when the user wants to decrement a die count rather than increment.
				//For now, just clear the equation after rolling
				eq.clear();
				win.set_equation(Equation::from(eq.borrow()).to_string().into());
			}
		}
	});
	
	mainWindow.run();
}
