use std::collections::HashMap;

use rocket::fs::TempFile;
use rocket::State;
use rocket::{http::Status, serde::json::Json};
use csv::ReaderBuilder;

use crate::core::db::DataBaseClient;
use crate::utils;

use super::types::{EmployeeReports, PayRollResult, EmployeeRecord};

#[post("/payroll/time-report/<report_id>", format ="text/csv", data = "<file>")]
pub async fn time_report(report_id: &str, file: TempFile<'_>) -> Result<Json<String>, Status> {
    let path = file.path();
    let reader = ReaderBuilder::new().from_path(path.unwrap());
    let mut employee_records: Vec<EmployeeRecord> = vec![];


    for results in reader.unwrap().records() {
        let record = results.unwrap();
        let data = EmployeeRecord {
            date: utils::format_date(&String::from(record.get(0).unwrap())),
            hours_worked: String::from(record.get(1).unwrap()),
            employee_id: String::from(record.get(2).unwrap()),
            job_group: String::from(record.get(3).unwrap()),
        };

        employee_records.push(data);
    }

    utils::sort_to_date_id(&mut employee_records);

    let mut list: HashMap<String, Vec<EmployeeRecord>> = HashMap::new();

    employee_records.iter().for_each(|e| {
        match list.get_mut(&e.employee_id) {
            Some(items) => {
                items.push(e.clone());

                utils::sort_to_date_id(items);
            }
            None => {
                list.insert(e.employee_id.to_string(), vec![e.clone()]);
            }
        }
    });

    let res = utils::organize_records_biweekly(list.clone());


    println!("here {:?}", res);


    Ok(Json("CSV Upload".to_string()))
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
