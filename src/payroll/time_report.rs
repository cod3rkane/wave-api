use rocket::State;
use rocket::{http::Status, serde::json::Json};

use crate::core::db::DataBaseClient;

#[put("/payroll/time-report/<report_id>")]
pub fn time_report(report_id: String) -> &'static str {
    println!("Report ID: {:?}", report_id);

    "CSV Upload"
}

#[get("/payroll/time-reports")]
pub async fn time_reports(db: &State<DataBaseClient>) -> Result<Json<String>, Status> {
    let reports = db.list_reports().await;

    Ok(Json(String::from("Hello Json Response!")))
}
