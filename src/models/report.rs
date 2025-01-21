use mongodb::bson::DateTime;
use mongodb::bson::{oid::ObjectId, Decimal128};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PayPeriod {
    #[serde(rename = "startDate")]
    pub start_date: DateTime,
    #[serde(rename = "endDate")]
    pub end_date: DateTime,
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
    pub amount_paid: Decimal128,
}
