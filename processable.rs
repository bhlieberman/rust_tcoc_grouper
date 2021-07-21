use crate::claim::Claim;
use std::{error::Error};

pub trait Processable<U, V> {
    fn process() -> Result<u32, Box<dyn Error>>;
    fn reconfigure() -> Result<u32, Box<dyn Error>>;
}

impl<T, U, V> Processable<U, V> for Claim<T> {
    fn process() -> Result<u32, Box<dyn Error>> {
        todo!()
    }

    fn reconfigure() -> Result<u32, Box<dyn Error>> {
        todo!()
    }
}