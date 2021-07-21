use core::hash::Hash;
use std::borrow::BorrowMut;
use std::collections::{HashMap, hash_map::Entry};
use std::cell::{RefCell};

use chrono::NaiveDate;

use crate::gfc_enum::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Claim<T> {
	claim_id: RefCell<String>,
	codes: Vec<T>,
}

#[derive(Clone, Debug, Default)]
pub struct Code<U> {
	serial_id: String,
	pub value: String,
	flags: Vec<Poa<U>>,
}

pub struct DiagnosisCode<U> {
    poa: Poa<U>,
    value: String
}

#[derive(Default)]
pub struct HcpcsCode {
    units: u32,
    revenue_code: u32,
    mods: String,
    charge: f32,
    short_desc: String,
    long_desc: String,
}

pub struct ProcedureCode<T> {
    date: T,
    value: String
}


impl<U> DiagnosisCode<U> {
    pub fn new() -> Self {
        DiagnosisCode { poa: Default::default(), value: Default::default() }
    }
}

impl HcpcsCode {
    pub fn new() -> Self {
        Default::default()
    }
}


impl<T> ProcedureCode<T>
    where 
        T: Default 
    {
        pub fn new() -> Self {
            ProcedureCode { date: Default::default() , value: Default::default() }
        }
    }


#[derive(Debug, Default)]
pub struct Flags<T, U> {
	serial_id: String,
	flag_map: HashMap<T, U>,
}

impl<T> Claim<T> where T: Default + Clone {
	pub fn new() -> Self {
		Default::default()
	}
    pub fn set_claim_id(&self, value: String) {
        self.claim_id.replace(value);
    }
	pub fn add_codes(&mut self, value: T) {
		self.codes.insert(0, value);
	}
    pub fn get_codes(&self) -> Option<&[T]> {
        match self.codes.is_empty() {
            true => None,
            _ => Some(&self.codes),
        }
    }
    pub fn get_codes_by_type(&self, var: Poa<T>) {
        todo!() // need an iterator over the CodeList that returns a
        // a filtered sublist by type of enum variant
    }
}

impl<U> Code<U> where U: Default {
	pub fn new(name: String) -> Self {
		Code { value: name, ..Default::default() }
	}
	pub fn set_flags(&mut self, flags: Poa<U>) {
		let mut flag_container = vec![];
        flag_container.insert(0, flags);
        self.flags = flag_container;
	}		
}

impl<T, U> Flags<T, U> where T: Default + Hash + PartialEq + Eq {
    pub fn new() -> Self {
        Flags { flag_map: HashMap::new() , serial_id: Default::default() }
    }
    pub fn set_flag(&mut self, key: T, value: U) -> &mut U {
        self.flag_map.entry(key).or_insert(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::claim::{Claim, Code, Flags};

    #[test]
    fn flags() {
        //let mut code: Code<String> = Code::new(RefCell::new("test_code".to_owned()));
        let mut flags: Flags<String, Vec<String>> = Flags::new();
        flags.set_flag(String::from("test_flag"), vec![String::from("test_flag_value")]);
        //code.set_flags(flags.flag_map["test_flag"][0].to_string());
        //assert_eq!(code.flags[0], String::from("test_flag_value"));
    }
    #[test]
    fn codes() {
        
    }
}