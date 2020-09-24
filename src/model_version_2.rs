pub use crate::model_version_1::DayVersion1;
pub use crate::model_version_1::SaveVersion1;
pub use crate::model_version_1::WeekVersion1;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveVersion2 {
    pub template: WeekVersion2,
    pub version: u32,
    pub weeks: Vec<WeekVersion2>,
}

#[derive(Serialize, Deserialize)]
pub struct WeekVersion2 {
    pub monday: DayVersion2,
    pub tuesday: DayVersion2,
    pub wednesday: DayVersion2,
    pub thurday: DayVersion2,
    pub friday: DayVersion2,
    #[serde(alias = "weekNumber", rename(serialize = "weekNumber"))]
    pub week_number: u32,
}

#[derive(Serialize, Deserialize)]
pub struct DayVersion2 {
    #[serde(alias = "break", rename(serialize = "break"))]
    pub break_time: u32,
    pub end: String,
    pub start: String,
    pub validate: bool,
}

// Upgrade

pub fn upgrade_to_v2(s: SaveVersion1) -> SaveVersion2 {
    let save = SaveVersion2 {
        version: 2,
        template: upgrade_template(s.template),
        weeks: upgrade_weeks(s.weeks),
    };
    save
}

fn upgrade_template(w: WeekVersion1) -> WeekVersion2 {
    println!("INFO: Upgrading template");
    let week = WeekVersion2 {
        monday: upgrade_day(w.monday, false),
        tuesday: upgrade_day(w.tuesday, false),
        wednesday: upgrade_day(w.wednesday, false),
        thurday: upgrade_day(w.thurday, false),
        friday: upgrade_day(w.friday, false),
        week_number: w.week_number,
    };
    week
}

fn upgrade_weeks(ws: Vec<WeekVersion1>) -> Vec<WeekVersion2> {
    println!("INFO: Upgrading weeks");
    let mut v: Vec<WeekVersion2> = Vec::new();
    let naive_date_time = Utc::now().naive_utc();
    let week_number = naive_date_time.iso_week().week();
    let dow = naive_date_time.weekday();
    for w in ws {
        if w.week_number < week_number {
            let week = WeekVersion2 {
                monday: upgrade_day(w.monday, true),
                tuesday: upgrade_day(w.tuesday, true),
                wednesday: upgrade_day(w.wednesday, true),
                thurday: upgrade_day(w.thurday, true),
                friday: upgrade_day(w.friday, true),
                week_number: w.week_number,
            };
            v.push(week);
        } else if w.week_number == week_number {
            let week = WeekVersion2 {
                monday: upgrade_day(w.monday, dow.num_days_from_monday() > 0),
                tuesday: upgrade_day(w.tuesday, dow.num_days_from_monday() > 1),
                wednesday: upgrade_day(w.wednesday, dow.num_days_from_monday() > 2),
                thurday: upgrade_day(w.thurday, dow.num_days_from_monday() > 3),
                friday: upgrade_day(w.friday, dow.num_days_from_monday() > 4),
                week_number: w.week_number,
            };
            v.push(week);
        } else {
            let week = WeekVersion2 {
                monday: upgrade_day(w.monday, false),
                tuesday: upgrade_day(w.tuesday, false),
                wednesday: upgrade_day(w.wednesday, false),
                thurday: upgrade_day(w.thurday, false),
                friday: upgrade_day(w.friday, false),
                week_number: w.week_number,
            };
            v.push(week);
        }
    }
    v
}

fn upgrade_day(d: DayVersion1, new_validate: bool) -> DayVersion2 {
    println!("INFO: Upgrading day");
    let day = DayVersion2 {
        break_time: d.break_time,
        end: d.end,
        start: d.start,
        validate: new_validate,
    };
    day
}

// Downgrade

pub fn downgrade_to_v1(s: SaveVersion2) -> SaveVersion1 {
    let save = SaveVersion1 {
        version: 1,
        template: downgrade_week(s.template),
        weeks: downgrade_weeks(s.weeks),
    };
    save
}

fn downgrade_week(w: WeekVersion2) -> WeekVersion1 {
    let week = WeekVersion1 {
        monday: downgrade_day(w.monday),
        tuesday: downgrade_day(w.tuesday),
        wednesday: downgrade_day(w.wednesday),
        thurday: downgrade_day(w.thurday),
        friday: downgrade_day(w.friday),
        week_number: w.week_number,
    };
    week
}

fn downgrade_weeks(ws: Vec<WeekVersion2>) -> Vec<WeekVersion1> {
    println!("INFO: Downgrading weeks");
    let mut v: Vec<WeekVersion1> = Vec::new();
    for w in ws {
        v.push(downgrade_week(w));
    }
    v
}

fn downgrade_day(d: DayVersion2) -> DayVersion1 {
    println!("INFO: Downgrading day");
    let day = DayVersion1 {
        break_time: d.break_time,
        end: d.end,
        start: d.start,
    };
    day
}
