use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PayPeriod {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "employeeId")]
    pub employee_id: String,
    #[serde(rename = "payPeriod")]
    pub pay_period: PayPeriod,
    #[serde(rename = "amountPaid")]
    pub amount_paid: String,
}
