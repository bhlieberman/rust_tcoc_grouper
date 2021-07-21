use chrono::NaiveDate;
use serde::Serialize;
#[allow(dead_code)]
pub struct AgeCriteria<T> {
    age_years: DateVariants<T>,
    age_days_admit: DateVariants<T>,
    age_days_discharge: DateVariants<T>,
}


#[allow(dead_code)]
pub enum DateVariants<T> {
    Some(T),
    String(String),
    Number(u32),
    Date(NaiveDate),
    None,
}

#[allow(dead_code)]
pub struct AgeAtDiagnosis<T> {
    service_end_date: DateVariants<T>,
    date_of_birth: DateVariants<T>,
    age_at_diagnosis: Option<T>,
}

#[allow(dead_code)]
impl<T> AgeAtDiagnosis<T> {
    pub fn new(service_end_date: DateVariants<T>, date_of_birth: DateVariants<T>) -> Self {
        todo!()
    }
}
