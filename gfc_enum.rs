use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Poa<T> {
    Yes(T),
    No(T),
    Unknown(T),
    W(T),
    One(T),
    Exempt,
    Blank(T),
    Invalid(T),

}

#[derive(Debug, Clone, Serialize)]
pub enum GfcSex<T> {
    Unknown(T),
    Male(T),
    Female(T),
}

impl<T> Default for Poa<T> {
    fn default() -> Self {
        Poa::Exempt
}
}