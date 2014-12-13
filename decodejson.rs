extern crate serialize;
use serialize::json;

fn main() {
    let file_json = r#"
    {
        "FirstName": "John",
        "LastName": "Doe",
        "Age": 43,
        "Address": {
            "Street": "Downing Street 10",
            "City": "London",
            "Country": "Great Britain"
        },
        "PhoneNumbers": [
            "+44 1234567",
            "+44 2345678"
        ]
    }
    "#;
    match json::from_str(file_json) {
        Ok(json) => println!("{}", json.find("Age").unwrap()),
        Err(err) => println!("{}", err),
    }
}