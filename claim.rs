use core::hash::Hash;
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq)]
pub struct Claim<T> {
	claim_id: String,
	codes: Vec<T>,
}

#[derive(Clone, Debug, Default)]
pub struct Code<U> {
	serial_id: String,
	blank: String,
	pub value: String,
	flags: Vec<U>,
}

#[derive(Debug, Default)]
pub struct Flags<T, U> {
	serial_id: String,
	pub flag_map: HashMap<T, U>,
}

impl<T> Claim<T> where T: Default {
	pub fn new() -> Claim<T> {
		Claim::default()
	}
	pub fn add_codes(&mut self, value: T) {
		self.codes.insert(0, value);
	}
}

impl<U> Code<U> where U: Default {
	pub fn new() -> Code<U> {
		Code::default()
	}
	pub fn set_flags(&mut self, flags: U) {
		let mut flag_container = vec![];
        flag_container.insert(0, flags);
        self.flags = flag_container;
	}		
}

impl<T, U> Flags<T, U> where T: Default + Hash + PartialEq + Eq, U: Default {
    pub fn new() -> Flags<T, U> {
        Flags::default()
    }
    pub fn set_flag(&mut self, key: T, value: U) -> &U {
        self.flag_map.entry(key).or_insert(value)
    }
}