use std::fs::File;
use std::io::{self, Read};
use crate::signature::dal::connect_database;

mod dal;

pub async fn check_signature(signature: &String) -> bool {
  // this is not a malicious hash, just for test.
  match connect_database(signature).await {
    Ok(check) => return true,  // Se Ok, retorna true
    Err(ref e) => return false, // Se Err, retorna false
  }


}
