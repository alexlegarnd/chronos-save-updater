use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveVersion1 {
    pub template: WeekVersion1,
    pub version: u32,
    pub weeks: Vec<WeekVersion1>,
}

#[derive(Serialize, Deserialize)]
pub struct WeekVersion1 {
    pub monday: DayVersion1,
    pub tuesday: DayVersion1,
    pub wednesday: DayVersion1,
    pub thurday: DayVersion1,
    pub friday: DayVersion1,
    #[serde(alias = "weekNumber", rename(serialize = "weekNumber"))]
    pub week_number: u32,
}

#[derive(Serialize, Deserialize)]
pub struct DayVersion1 {
    #[serde(alias = "break", rename(serialize = "break"))]
    pub break_time: u32,
    pub end: String,
    pub start: String,
}
