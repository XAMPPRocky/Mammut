extern crate mammut;
extern crate serde_json as json;

use mammut::entities::account::Account;
use std::fs::File;
use std::io::prelude::*;

// Test that the received account JSON can be deserialized correctly and that
// the account IDs are accepted either as string or as integer.
#[test]
fn account() {
    let mut file = File::open("tests/account1.json").unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    let account: Account = json::from_str(&file_content).unwrap();
    assert_eq!(account.id, 123456);

    let mut file = File::open("tests/account2.json").unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    let account: Account = json::from_str(&file_content).unwrap();
    assert_eq!(account.id, 123456);
}
