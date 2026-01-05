use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<String> {
    let mut results = vec![];

    for line in text.lines() {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}
fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logsi.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("error-logs.txt", error_logs.join("\n"))?;
    Ok(())
    // let text = fs::read_to_string("logs.txt").expect("failed to read logs.txt");

    // let error_logs = extract_errors(text.as_str());
    // fs::write("errors.txt", error_logs.join("\n")).expect("failed to write file")

    // match fs::read_to_string("logs.txt") {
    //     Ok(text_read) => {
    //         let error_logs = extract_errors(text_read.as_str());

    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Success"),
    //             Err(failure_reason) => println!("{}", failure_reason),
    //         }
    //     }
    //     Err(failure_reason) => println!("{}", failure_reason),
    // }
}

// Tuple examples
//
// fn divide(num1: f64, num2: f64) -> Result<f64, Error> {
//     if num2 == 0.0 || num1 == 0.0 {
//         Err(Error::other("can't divide by 0"))
//     } else {
//         Ok(num1 / num2)
//     }
// }

// fn validate_email(email: String) -> Result<(), Error> {
//     if email.contains("@") {
//         Ok(())
//     } else {
//         Err(Error::other("emails must have an @"))
//     }
// }
// fn main() {
//     match divide(0.0, 3.0) {
//         Ok(result_of_div) => println!("{}", result_of_div),
//         Err(what_went_wrong) => println!("{}", what_went_wrong),
//     }

//     match validate_email(String::from("vcupri@codevider.com")) {
//         Ok(..) => println!("email is valid"),
//         Err(reason_of_error) => println!("{}", reason_of_error),
//     }
// }
