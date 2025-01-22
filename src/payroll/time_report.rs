use csv::ReaderBuilder;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::State;
use rocket::{http::Status, serde::json::Json};
use std::collections::HashMap;

use crate::core::db::DataBaseClient;
use crate::models::report::{PayPeriod, Report};
use crate::utils;

use super::types::{EmployeeRecord, EmployeeReports, PayRollResult};

#[derive(FromForm)]
pub struct Upload<'r> {
    file: TempFile<'r>,
}

#[post("/payroll/time-report/<report_id>", data = "<upload>")]
pub async fn time_report(
    report_id: &str,
    upload: Form<Upload<'_>>,
    db: &State<DataBaseClient>,
) -> Result<Json<String>, Status> {
    let path = upload.file.path();
    let reader = ReaderBuilder::new().from_path(path.unwrap());
    let mut employee_records: Vec<EmployeeRecord> = vec![];

    let file_exists = db
        .find_filename(report_id, upload.file.name().unwrap())
        .await
        .unwrap();

    match file_exists {
        Some(_) => Err(Status::BadRequest),
        None => {
            // create file
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

            employee_records
                .iter()
                .for_each(|e| match list.get_mut(&e.employee_id) {
                    Some(items) => {
                        items.push(e.clone());

                        utils::sort_to_date_id(items);
                    }
                    None => {
                        list.insert(e.employee_id.to_string(), vec![e.clone()]);
                    }
                });

            let items = utils::organize_records_biweekly(list.clone());
            let mut reports: Vec<Report> = vec![];

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

                    let hours_worked: f32 = records
                        .iter()
                        .map(|e| e.hours_worked.parse::<f32>().unwrap())
                        .sum();
                    let job_group = records.first().unwrap().job_group.clone();

                    let amount_paid = if job_group.eq(&"B") {
                        hours_worked * 30.0
                    } else {
                        hours_worked * 20.0
                    };

                    let report = Report {
                        amount_paid: amount_paid.to_string(),
                        employee_id: id.to_string(),
                        id: None,
                        pay_period: PayPeriod {
                            start_date,
                            end_date,
                        },
                    };

                    reports.push(report);
                });
            });

            let res = db.insert_reports(reports).await;
            let _ = db
                .insert_filename(report_id, upload.file.name().unwrap())
                .await;

            match res {
                Ok(_) => Ok(Json("OK".to_string())),
                Err(_) => Err(Status::BadRequest),
            }
        }
    }
}

#[get("/payroll/time-reports")]
pub async fn time_reports(db: &State<DataBaseClient>) -> Result<Json<PayRollResult>, Status> {
    let reports = db.list_reports().await;

    match reports {
        Ok(list) => Ok(Json(PayRollResult {
            payroll_report: EmployeeReports {
                employee_reports: list,
            },
        })),
        Err(_) => Err(Status::NotFound),
    }
}
