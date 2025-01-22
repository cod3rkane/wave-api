use std::collections::HashMap;

use chrono::NaiveDate;
use rocket::fs::TempFile;
use rocket::State;
use rocket::{http::Status, serde::json::Json};
use csv::ReaderBuilder;

use crate::core::db::DataBaseClient;
use crate::models::report::{PayPeriod, Report};
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

    let items = utils::organize_records_biweekly(list.clone());

    // A = 20
    // B = 30

    items.iter().for_each(|(id, dates)| {
        dates.iter().for_each(|(date, records)| {
            let parts: Vec<&str> = date.split('-').collect();
            let year = parts[0].parse::<u32>().unwrap();
            let month = parts[1].parse::<u32>().unwrap();
            let end = parts[3].parse::<u32>().unwrap();
            let start_date = if end <= 15 {
                format!("{:4}-{:02}-{:02}", year, month, 01)
            } else {
                format!("{:4}-{:02}-{:02}", year, month, 16)
            };
            let end_date = if end <= 15 {
                format!("{:4}-{:02}-{:02}", year, month, 15)
            } else {
                format!("{:4}-{:02}-{:02}", year, month, 31)
            };

            let hours_worked: f32 = records.iter().map(|e| e.hours_worked.parse::<f32>().unwrap()).sum();
            let job_group = records.first().unwrap().job_group.clone();

            let amount_paid = if job_group.eq(&"B") { hours_worked * 30.0 } else { hours_worked * 20.0 };

            let report = Report {
                amount_paid: amount_paid.to_string(),
                employee_id: id.to_string(),
                id: None,
                pay_period: PayPeriod {
                    start_date,
                    end_date,
                }
            };

            println!("here Report {:?}", report);
        });
    });

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
