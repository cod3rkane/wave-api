use chrono::prelude::*;
use std::collections::HashMap;

use crate::payroll::types::EmployeeRecord;

/// Sorts EmployeeRecord by ID and Date ASC
pub fn sort_to_date_id(list: &mut Vec<EmployeeRecord>) {
    list.sort_by(|a, b| {
        let id = a.employee_id.cmp(&b.employee_id);

        if id == std::cmp::Ordering::Equal {
            a.date.cmp(&b.date)
        } else {
            id
        }
    });
}

/// Formats date from DD/MM/YYYY to ISO YYYY-MM-DD
pub fn format_date(date: &str) -> String {
    let parts: Vec<&str> = date.split('/').collect();

    if parts.len() == 3 {
        format!(
            "{:4}-{:02}-{:02}",
            parts[2].parse::<u32>().unwrap(),
            parts[1].parse::<u32>().unwrap(),
            parts[0].parse::<u32>().unwrap()
        )
    } else {
        date.to_string()
    }
}

pub fn organize_records_biweekly(
    list: HashMap<String, Vec<EmployeeRecord>>,
) -> HashMap<String, HashMap<String, Vec<EmployeeRecord>>> {
    let mut res: HashMap<String, HashMap<String, Vec<EmployeeRecord>>> = HashMap::new();

    list.iter().for_each(|(id, records)| {
        records.iter().for_each(|r| {
            let d = NaiveDate::parse_from_str(&r.date, "%Y-%m-%d").unwrap();
            let date = d.and_hms_opt(0, 0, 0).unwrap().and_utc();

            if date.day().cmp(&15).is_le() {
                // 1th-15th
                let date_title = format!("{}-{}-1-15", date.year(), date.month());

                match res.get_mut(&id.to_string()) {
                    Some(date_records) => {
                        // User Exits
                        match date_records.get_mut(&date_title) {
                            Some(records) => {
                                records.push(r.clone());
                            }
                            None => {
                                date_records.insert(date_title, vec![r.clone()]);
                            }
                        }
                    }
                    None => {
                        // No User yet
                        res.insert(
                            id.to_string(),
                            HashMap::from([(date_title, vec![r.clone()])]),
                        );
                    }
                }
            } else {
                // 16th-end of month
                let date_title = format!("{}-{}-16-31", date.year(), date.month());

                match res.get_mut(&id.to_string()) {
                    Some(date_records) => {
                        // User Exits
                        match date_records.get_mut(&date_title) {
                            Some(records) => {
                                records.push(r.clone());
                            }
                            None => {
                                date_records.insert(date_title, vec![r.clone()]);
                            }
                        }
                    }
                    None => {
                        // No User yet
                        res.insert(
                            id.to_string(),
                            HashMap::from([(date_title, vec![r.clone()])]),
                        );
                    }
                }
            }
        });
    });

    res
}
