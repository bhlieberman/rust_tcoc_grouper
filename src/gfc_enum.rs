pub enum GFC {
    int(u32),
    char(char),
    string(String),
}

pub enum POA {
    yes(String),
    no(String),
    unknown(String),
    w(String),
    one(String),
    exempt(String),
    blank(String),
    invalid(char),

}

pub struct GfcPoa {
    char_val: char,
    description: String,
}