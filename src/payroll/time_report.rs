use rocket::fs::TempFile;
use rocket::State;
use rocket::{http::Status, serde::json::Json};
use csv::ReaderBuilder;

use crate::core::db::DataBaseClient;

use super::types::{EmployeeReports, PayRollResult};


#[post("/payroll/time-report/<report_id>", format ="text/csv", data = "<file>")]
pub async fn time_report(report_id: &str, file: TempFile<'_>) -> &'static str {
    let path = file.path();
    let reader = ReaderBuilder::new().from_path(path.unwrap());


    for results in reader.unwrap().records() {
        let record = results.unwrap();

        println!("here {:?}", record);
    }

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
