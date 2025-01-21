use crate::payroll::types::EmployeeRecord;

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
        format!("{}-{}-{}", parts[2], parts[1], parts[0])
    } else {
        date.to_string()
    }
}
