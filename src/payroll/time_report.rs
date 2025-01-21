use rocket::State;
use rocket::{http::Status, serde::json::Json};

use crate::core::db::DataBaseClient;

use super::types::{EmployeeReports, PayRollResult};

#[put("/payroll/time-report/<report_id>")]
pub fn time_report(report_id: String) -> &'static str {
    println!("Report ID: {:?}", report_id);

    "CSV Upload"
}

#[get("/payroll/time-reports")]
pub async fn time_reports(db: &State<DataBaseClient>) -> Result<Json<PayRollResult>, Status> {
    let reports = db.list_reports().await;

    match reports {
        Ok(list) => Ok(Json(PayRollResult {
            payroll_report: EmployeeReports {
                employee_reports: list
            }
        })),
        Err(_) => Err(Status::NotFound),
    }
}
