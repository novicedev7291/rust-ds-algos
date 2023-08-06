use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ArrParseError {
    msg: String,
}

impl Display for ArrParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub fn parse_array(arr_str: &str) -> Result<Vec<usize>, ArrParseError> {
    let mut result = Vec::new();
    let mut num = String::from("");
    let mut arr_start = false;
    let mut arr_end = false;
    for char in arr_str.chars() {
        match char {
            '[' | ' ' => {
                arr_start = true;
                continue;
            }
            ',' => {
                result.push(num.to_owned().parse::<usize>().unwrap());
                num.clear();
            }
            ']' => {
                result.push(num.to_owned().parse::<usize>().unwrap());
                arr_end = true;
                break;
            }
            _ => {
                if !arr_start {
                    return Err(ArrParseError {
                        msg: "Expecting [ at the start of array".to_string(),
                    });
                }
                num.push(char);
            }
        }
    }

    if !arr_end {
        return Err(ArrParseError {
            msg: "Expecting ] at the end of array".to_string(),
        });
    }

    Ok(result)
}
