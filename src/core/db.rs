use mongodb::{
    bson::{Document, doc},
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

    pub async fn list_reports(&self) -> Result<String, Box<dyn Error>> {
        let mut reports = self.report_collection.find(doc! {}).await;
        let mut cursor = reports.unwrap();

        while cursor.advance().await? {
            println!("here {:?}", cursor.deserialize_current());
        }

        Ok("Hello".to_string())
    }
}
