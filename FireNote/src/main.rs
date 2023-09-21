
use firestore_db_and_auth::errors::FirebaseError::APIError;
use firestore_db_and_auth::{documents, errors, Credentials, FirebaseAuthBearer};
use serde_json;

const CREDENTIAL_PATH: &str = "/mnt/84C0BF1FC0BF1700/Shared/rs_t/firbase.json";

fn main () { 
    println!("Hello there");
        

    let credentials = Credentials::from_file(CREDENTIAL_PATH).unwrap();
    
    credentials.verify().unwrap();

    let jsoned = serde_json::to_string(&credentials).unwrap();

}
