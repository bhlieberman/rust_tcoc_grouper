use crate::claim::{Claim, Code, Flags};
use crate::gfc_enum::*;
use crate::processable::*;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::Debug;

mod claim;
mod gfc_enum;
mod processable;
mod age_criteria;
mod inputs;
mod date_period;
mod errors;


fn main() {
      let test_claim = new_claim::<Poa<String>>();
      match test_claim {
          Ok(test_claim) => retrieve_claim_data(test_claim),
          Err(e) => eprintln!("Error")
      };
}

fn new_claim<T>() -> Result<Claim<String>, Box<dyn Error>> {
    let mut new_claim: Claim<String> = Claim::new();
    let mut first_code: Code<String> = Code::new("Code1".to_owned());
    let mut new_flag: Flags<String, Poa<String>> = Flags::new();
    first_code.set_flags(
        new_flag.set_flag(
            "test_flag".to_owned(), 
            Poa::Blank("".to_owned()))
            .clone()
        );
    new_claim.add_codes(first_code.value);
    println!("{:?}", new_claim);
    new_claim.set_claim_id("value".to_owned());
    
    Ok(new_claim)

    
}

fn retrieve_claim_data<T: Default + Debug + Clone>(claim: Claim<T>) {
    println!("{:?}", claim.get_codes());
}
