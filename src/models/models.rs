use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::models::schema::*;

/// JzAdress representation
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name="jz_adress"]
pub struct JzAdress {
    pub ad_uuid: Uuid,
    pub ad_street: String,
    pub ad_country: String,
    pub ad_adress_additional: String,
    pub ad_zip_code: String,
    pub ad_city: String,
    pub ad_number: String,
    pub ad_name: String,
}