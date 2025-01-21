use mongodb::{
    bson::doc,
    sync::{ Client, Collection }
};
use std::error::Error;
use crate::models::report::Report;

#[derive(Debug)]
pub struct DataBaseClient {
    report_collection: Collection<Report>,
}

impl DataBaseClient {
    pub fn init() -> Self {
        let uri = "mongodb://localhost:27017/";
        let client = Client::with_uri_str(uri).unwrap();

        let database = client.database("payroll");
        let collection: Collection<Report> = database.collection("employeeReports");

        DataBaseClient {
            report_collection: collection,
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
}
