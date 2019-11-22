use std::error::Error;
use std::fmt;

type BoxResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct NoStringGiven;

impl fmt::Display for NoStringGiven {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "you need to give a string to convert!")
    }
}

impl Error for NoStringGiven {
    fn description(&self) -> &str {
        "you need to give a string to convert!"
    }

    fn cause(&self) -> Option<&dyn Error> {
        // No need to implement cause
        None
    }
}

pub fn string_to_i32(optional_string: Option<&str>) -> std::result::Result<i32, Box<dyn Error>> {
    optional_string
        .ok_or_else(|| NoStringGiven.into()) // Converts to Box
        .and_then(|s| {
            // map_err maps parse error type into the expected Box error type
            s.parse::<i32>().map_err(|e| e.into()) // Converts to Box
        })
        .and_then(result)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn string_conversion() {
        // No good way to compare Box values (that I could find) other than formatting them to string
        let res = string_to_i32(None);
        assert_eq!(format!("{}", res.unwrap_err()), NoStringGiven.to_string());

        // error within string function should return that error
        let res = string_to_i32(Some("not an integer lol"));
        assert_eq!(
            format!("{}", res.unwrap_err()),
            "invalid digit found in string".to_owned()
        );

        println!("First call: ");
        let _res = string_to_i32(None).map(test_map);

        println!("Second call: ");
        // Same as previous function call
        let _res = string_to_i32(Some("1")).map(|v| test_map(v));
        
        // assert_eq!(res.unwrap(), 1);
    }

    fn test_map(res: i32) -> i32 {
        println!("CALLED test_map, value is: {}", res);
        res
    }
}

//

//

fn result(i: i32) -> BoxResult<i32> {
    Ok(i * 2)
}
