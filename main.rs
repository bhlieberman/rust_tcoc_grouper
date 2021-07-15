use crate::claim::{Claim, Code, Flags};

mod claim;

fn main() {
    let mut code: Code<String> = Code::new();
    let mut flags: Flags<String, Vec<String>> = Flags::new();
    flags.set_flag(String::from("test_flag"), vec![String::from("test_flag_value")]);
    code.value = String::from("test_code");
    code.set_flags(flags.flag_map["test_flag"][0].to_string());
    let mut claim = Claim::new();
    claim.add_codes(code.value.as_str());
    println!("{:?}", claim);
    println!("{:?}", code);
}
