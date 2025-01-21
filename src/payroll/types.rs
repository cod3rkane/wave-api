use serde::{Deserialize, Serialize};

use crate::models::report::Report;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeReports {
    #[serde(rename = "employeeReports")]
    pub employee_reports: Vec<Report>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayRollResult {
    #[serde(rename = "payrollReport")]
    pub payroll_report: EmployeeReports,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeRecord {
    pub date: String,
    pub hours_worked: String,
    pub employee_id: String,
    pub job_group: String,
}
