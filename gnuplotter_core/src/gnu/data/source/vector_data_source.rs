use std::collections::VecDeque;
use std::fs::File;
use std::io::{Error, Write};
use crate::gnu::data::source::DataSource;
use crate::prelude::*;
use crate::result::*;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct VectorDataSource {
    data: Vec<f64>
}

impl VectorDataSource {
    pub fn add(&mut self, value: f64) {
        self.data.push(value);
    }

    fn get(&self, index: usize) -> Option<f64> {
        self.data.get(index).cloned()
    }
}

struct VectorDataSourceIterator {
    data: VectorDataSource,
    index: usize
}

impl VectorDataSourceIterator {
    fn new(data: VectorDataSource) -> Self {
        VectorDataSourceIterator {
            data,
            index: 0
        }
    }
}

impl Iterator for VectorDataSourceIterator {
    type Item = (i32, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.index;
        self.index += 1;

        self.data
            .get(current)
            .map(
                |x| (current as i32, x)
            )
    }
}

impl GnuCommandFactory for VectorDataSource {
    fn as_commands(&self) -> GnuCommandFactoryResult {
        match File::create("data.txt") {
            Ok(mut file) => {

                for i in 0..self.data.len() {
                    if let Err(_) = write!(&mut file, "{}\t{}\n", i, self.data[i]) {
                        return Err(
                            GnuCommandFactoryError::io_error("Unable to write data to file.")
                        );
                    }
                }

                Ok(
                    vec![
                        GnuCommand::new("some command")
                    ].into()
                )
            },
            Err(_) => {
                Err(
                    GnuCommandFactoryError::io_error("Unable to write data to file.")
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterating_over_data(){
        let mut source = VectorDataSource::default();
        for i in 0..10 {
            source.add((i*i) as f64);
        }

        let it = VectorDataSourceIterator::new(source);
        let data: Vec<(i32, f64)> = it.take(4).collect();

        assert_eq!(data, vec![(0,0f64), (1,1f64), (2,4f64), (3,9f64)]);
    }
}