use mongodb::{
    bson::doc, results::{InsertManyResult, InsertOneResult}, sync::{ Client, Collection }
};
use std::error::Error;
use crate::models::report::Report;
use crate::models::report_file::ReportFile;

#[derive(Debug)]
pub struct DataBaseClient {
    report_collection: Collection<Report>,
    report_files: Collection<ReportFile>,
}

impl DataBaseClient {
    pub fn init() -> Self {
        let uri = "mongodb://localhost:27017/";
        let client = Client::with_uri_str(uri).unwrap();

        let database = client.database("payroll");
        let report_collection: Collection<Report> = database.collection("employeeReports");
        let report_file_collection: Collection<ReportFile> = database.collection("reportFiles");

        DataBaseClient {
            report_collection: report_collection,
            report_files: report_file_collection,
        }
    }

    pub async fn list_reports(&self) -> Result<Vec<Report>, Box<dyn Error>> {
        let query = self.report_collection.find(doc! {}).await;
        let mut cursor = query.unwrap();
        let mut reports: Vec<Report> = vec![];

        while cursor.advance().await? {
            let mut report = cursor.deserialize_current().unwrap();

            report.amount_paid = "$".to_string() + &report.amount_paid.to_string();

            reports.push(report);
        }

        Ok(reports)
    }

    pub async fn insert_reports(&self, report: Vec<Report>) -> Result<InsertManyResult, Box<dyn Error>> {
        let res = self.report_collection.insert_many(report).await?;

        Ok(res)
    }
}
