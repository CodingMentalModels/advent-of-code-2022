use std::{fs::File, io::Read, path::Path};

const INPUT_PATH_HEAD: &str = "C:/Users/cmsdu/repos/advent-of-code-2022/input/input.txt";

pub struct InputParser;

impl InputParser {

    pub fn new() -> InputParser {
        InputParser {}
    }

    pub fn parse_as_i32(&self, filepath: &str) -> Result<Vec<i32>, String> {
        self.parse_as_string(filepath)?.iter().enumerate().map(
            |(i, line)| line.parse::<i32>()
                .map_err(|_| format!("Unable to parse line {} to i32.", i)
            )
        ).collect()
    }

    pub fn parse_as_binary(&self, filepath: &str) -> Result<Vec<u32>, String> {
        self.parse_as_string(filepath)?.into_iter().map(
            |line| line.chars().map(
                |c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => 2,
                }
            ).collect::<Vec<_>>()
        ).map(|bytes| if bytes.iter().all(|b| *b == 0 || *b == 1) { Ok(bytes) } else { Err(format!("Invalid byte in {:?}", bytes))}).collect::<Vec<_>>()
            .into_iter().map(|maybe_bytes| {
                maybe_bytes.map(
                    |bytes| {
                        let mut result: u32 = 0;
                        for byte in bytes {
                            result = result << 1;
                            result += byte as u32;
                        }
                        Ok(result)
                    }
                )?
            }).collect()
    }

    pub fn parse_as_string(&self, filepath: &str) -> Result<Vec<String>, String> {
        let full_path_string = format!("{}{}", INPUT_PATH_HEAD, filepath);
        let full_path = Path::new(&full_path_string);
        let mut file = File::open(full_path).map_err(|_| "Unable to open file.".to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|_| "Unable to read file.".to_string())?;
        Ok(contents.lines().map(|line| line.to_string()).collect())
    }
}


#[cfg(test)]
mod test_input {
    use super::*;

    #[test]
    fn test_input_parses_input() {
        
        let parser = InputParser::new();
        let parsed = parser.parse_as_i32("input_001.txt").unwrap();

        assert_eq!(parsed.len(), 2000);
    }

    #[test]
    fn test_input_parses_binary() {
                
        let input = InputParser::new().parse_as_binary("input_003.txt").unwrap();
        let shortened_input = input.iter().take(3).map(|s| *s).collect::<Vec<u32>>();

        // 111100101100
        // 101100110001
        // 100110100101


    }
}