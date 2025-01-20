use rocket::{http::Status, serde::json::Json};

#[put("/payroll/time-report/<report_id>")]
pub fn time_report(report_id: String) -> &'static str {
    println!("Report ID: {:?}", report_id);

    "CSV Upload"
}

#[get("/payroll/time-reports")]
pub fn time_reports() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello Json Response!")))
}
