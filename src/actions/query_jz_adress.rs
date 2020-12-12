use crate::models::models::*;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use crate::models::schema::jz_adress::dsl::*;

/// find all JzAdress
pub fn find_all(
    conn: &PgConnection
)
    // QueryResult
    // https://docs.diesel.rs/diesel/result/type.QueryResult.html
    //
    // is type alias on a Result<T, Error>
    // where 'Error' is meant to be a diesel Error: https://docs.diesel.rs/diesel/result/enum.Error.html
    -> QueryResult<Vec<JzAdress>>
{
    jz_adress.load::<JzAdress>(conn)
}