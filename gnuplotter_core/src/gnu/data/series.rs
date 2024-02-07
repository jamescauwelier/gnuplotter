use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::io::Write;
use rand::random;
use crate::prelude::*;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Series<T>
where
    T: Default + Clone + ToString
{
    data: Vec<Serie<T>>
}

impl<T> Series<T>
where
    T: Default + Clone + ToString
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, serie: Serie<T>) {
        self.data.push(serie);
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        for serie in &self.data {
            len = std::cmp::max(len, serie.len());
        }

        len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> SeriesIterator<T> {
        SeriesIterator {
            data: self.clone(),
            index: 0
        }
    }

    pub fn write_to_file(&self, filename: &str) -> Result<String> {
        match File::create(filename) {
            Ok(mut file) => {

                for (index, values) in self.iter() {
                    let data = values.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\t");
                    if let Err(_) = writeln!(file, "{}\t{}", index, data) {
                        return Err("Unable to write series data".into());
                    }
                }

                Ok(filename.into())
            },
            Err(_) => Err("Unable to create file.".into())
        }
    }
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct SeriesIterator<T>
where
    T: Default + Clone + ToString
{
    data: Series<T>,
    index: usize
}

impl<T> Iterator for SeriesIterator<T>
where
    T: Default + Clone + ToString
{
    type Item = (usize, Vec<T>);

    fn next(&mut self) -> Option<Self::Item> {

        if (self.data.len()) <= self.index {
            return None;
        }

        let mut result = vec![];
        for data in self.data.data.iter() {
            result.push(data.get(self.index).unwrap());
        }

        let old_index = self.index;
        self.index += 1;

        Some((old_index, result))
    }
}

impl<T> GnuCommandFactory for Series<T>
where
    T: Default + Clone + ToString
{
    fn as_commands(&self) -> Result<VecDeque<GnuCommand>> {

        use std::fs;
        let _ = fs::create_dir(".tmp");

        let filename = "./.tmp/series_data.txt";
        self.write_to_file(filename)?;

        let mut command = "plot ".to_string();
        for i in 0..self.data.len() {
            if let Some(title) = self.data[i].title() {
                command += &format!("\"{}\" using 1:{} title '{}' with linespoint, ", filename, i + 2, title);
            } else {
                command += &format!("\"{}\" using 1:{} with linespoint, ", filename, i + 2);
            }
        }
        let command = command.strip_suffix(", ").ok_or("Unable to strip suffix")?;

        Ok(
            vec![GnuCommand::new(command)].into()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_series_len(){
        let mut serie_1 = Serie::new();
        serie_1.add(1.0);

        let mut serie_2 = Serie::new();
        serie_2.add(2.0);
        serie_2.add(4.0);
        serie_2.add(6.0);

        let mut series = Series::new();
        series.add(serie_1);
        series.add(serie_2);

        assert_eq!(series.len(), 3);
    }

    #[test]
    fn test_is_empty(){
        let serie_1: Serie<f64> = Serie::new();
        let mut series = Series::new();
        series.add(serie_1);

        assert!(series.is_empty());

        let mut serie_1 = Serie::new();
        serie_1.add(1.0);
        let mut series = Series::new();
        series.add(serie_1);

        assert!(!series.is_empty());
    }

    #[test]
    fn test_creating_series() {
        let mut serie_1 = Serie::new();
        serie_1.add(1.0);
        serie_1.add(2.0);
        serie_1.add(3.0);

        let mut serie_2 = Serie::new();
        serie_2.add(2.0);
        serie_2.add(4.0);
        serie_2.add(6.0);

        let mut series = Series::new();
        series.add(serie_1);
        series.add(serie_2);

        let mut it = series.iter();
        assert_eq!(it.next(), Some((0, vec![1.0, 2.0])));
        assert_eq!(it.next(), Some((1, vec![2.0, 4.0])));
        assert_eq!(it.next(), Some((2, vec![3.0, 6.0])));
    }

    #[test]
    fn test_series_command() {
        let mut serie_1 = Serie::with_title(Some("A".into()));
        serie_1.add(1.0);
        serie_1.add(2.0);
        serie_1.add(3.0);

        let mut serie_2 = Serie::with_title(Some("B".into()));
        serie_2.add(2.0);
        serie_2.add(4.0);
        serie_2.add(6.0);

        let mut series = Series::new();
        series.add(serie_1);
        series.add(serie_2);

        let mut command = series.as_commands().unwrap();

        assert_eq!(command.pop_front().unwrap(), GnuCommand::new("plot \"./.tmp/series_data.txt\" using 1:2 title 'A' with linespoint, \"./.tmp/series_data.txt\" using 1:3 title 'B' with linespoint"));
    }

    #[test]
    fn test_series_command_without_titles() {
        let mut serie_1 = Serie::new();
        serie_1.add(1.0);
        serie_1.add(2.0);
        serie_1.add(3.0);

        let mut serie_2 = Serie::new();
        serie_2.add(2.0);
        serie_2.add(4.0);
        serie_2.add(6.0);

        let mut series = Series::new();
        series.add(serie_1);
        series.add(serie_2);

        let mut command = series.as_commands().unwrap();

        assert_eq!(command.pop_front().unwrap(), GnuCommand::new("plot \"./.tmp/series_data.txt\" using 1:2 with linespoint, \"./.tmp/series_data.txt\" using 1:3 with linespoint"));
    }
}