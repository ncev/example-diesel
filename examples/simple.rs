use diesel::{PgConnection, ConnectionResult, Connection};
use example_diesel::actions::query_jz_adress::find_all;

/// print all adress
/// (without constraints)
fn print_all_adress(connection: &PgConnection) {


    // find all address
    let jz_address = find_all(&connection);


    // explicit pattern match to handle error
    match jz_address {

        // Ok => success
        Ok(jz_address) =>
            jz_address
                .into_iter()
                .for_each(|ad| println!("{}", ad.ad_name)),

        // Err => fail
        Err(err) =>
            eprintln!("error while query all address: {}", err)
    }
}



fn main() -> ConnectionResult<()> {

    // PG connection string
    let pg_connection_string =
        "postgres://";

    // try connection
    let connection =
        PgConnection::establish(pg_connection_string)?;

    print_all_adress(&connection);

    // Ok wrapping unit
    // simply mean everything's append well
    Ok(())
}