pub struct AgeCriteria {
    age_years: u32,
    age_days_admit: u32,
    age_days_discharge: u32,
}

impl AgeCriteria {
    pub fn new(years: u32, days_admit: u32, days_discharge: u32) -> AgeCriteria {
        AgeCriteria {
            age_years: years,
            age_days_admit: days_admit,
            age_days_discharge: days_discharge
        }
    }
}