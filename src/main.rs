use std::fs::File;

fn main() {
    match File::open("NON-EXISTENT") {    
        Ok(_) => {   
            "anything".to_string()
        }
        Err(_) => format!("1234567890+1234567890+1234567890+1234567890+1234567890+1234567890+1234567890+123456789"),
    };
    // If the text inside of the format string is 1 char shorter the problem goes away
}

// error[internal]: left behind trailing whitespace
//  --> /home/one/rustfmt_mre/src/main.rs:5:5:19
//   |
// 5 |         Ok(_) => {
//   |                   ^^^
//   |

// warning: rustfmt has failed to format. See previous 1 errors.
