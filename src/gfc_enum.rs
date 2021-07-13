pub enum GFC {
    Int(u32),
    Character(char),
    String(String),
}

pub enum POA {
    Yes(GFC),
    No(GFC),
    Unknown(GFC),
    W(GFC),
    One(GFC),
    Exempt(GFC),
    Blank(GFC),
    Invalid(GFC),

}