use std::fmt::{Display, Formatter};

pub struct Filename(String);

impl Filename {
    pub fn new(filename: &str) -> Self {
        Filename(filename.into())
    }
}

impl Display for Filename {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_of_filename_wrapper(){
        let filename = Filename::new("./some-file.png");

        assert_eq!(filename.to_string(), "./some-file.png");
    }
}