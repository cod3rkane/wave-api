#[put("/payroll/time-report/<report_id>")]
pub fn time_report(report_id: String) -> &'static str {
    println!("Report ID: {:?}", report_id);

    "CSV Upload"
}

#[get("/payroll/time-reports")]
pub fn time_reports() -> &'static str {
    "Via Carai!"
}
