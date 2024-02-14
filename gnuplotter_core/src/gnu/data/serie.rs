#[derive(Default, PartialEq, Debug, Clone)]
pub struct Serie<T>
where
    T: Default
{
    title: Option<String>,
    data: Vec<T>
}

impl<T> Serie<T>
where
    T: Default + Clone
{
    pub fn new() -> Self {
        Serie::default()
    }

    pub fn with_title(title: &str) -> Self {
        Serie {
            title: Some(title.to_string()),
            data: vec![]
        }
    }

    pub fn add(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn get(&self, index: usize) -> Option<T> {
        self.data.get(index).cloned()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    pub fn title(&self)-> &Option<String> {
        &self.title
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serie_creation() {
        let serie = Serie::<f64>::new();
        assert_eq!(serie.data.len(), 0);
        assert_eq!(serie.title(), &None);
    }

    #[test]
    fn test_serie_creation_with_title() {
        let serie = Serie::<f64>::with_title("Serie A");
        assert_eq!(serie.data.len(), 0);
        assert_eq!(serie.title(), &Some("Serie A".into()));
    }

    #[test]
    fn test_serie_add() {
        let mut serie = Serie::<f64>::new();
        serie.add(1.0);
        serie.add(2.0);
        serie.add(3.0);

        assert_eq!(serie.data.len(), 3);
    }

    #[test]
    fn test_serie_get(){
        let mut serie = Serie::<f64>::new();
        serie.add(1.0);
        serie.add(2.0);
        serie.add(3.0);

        assert_eq!(serie.get(0), Some(1.0));
        assert_eq!(serie.get(1), Some(2.0));
        assert_eq!(serie.get(2), Some(3.0));
        assert_eq!(serie.get(3), None);
    }

    #[test]
    fn test_len(){
        let mut serie = Serie::<f64>::new();

        assert_eq!(serie.len(), 0);

        serie.add(1.0);
        serie.add(2.0);
        serie.add(3.0);

        assert_eq!(serie.len(), 3);
    }
}