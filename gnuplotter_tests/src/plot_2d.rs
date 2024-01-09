use gnuplotter::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Default, Plot)]
pub struct Plot2D {
    title: Option<String>,
    x_label: Required<String>,
    y_label: Required<String>,
}

impl Plot2D {
    pub fn new() -> Self {
        Plot2D::default()
    }
}

#[cfg(test)]
mod tests {
    use gnuplotter::prelude::Required::Missing;
    use super::*;

    #[test]
    fn test_plot_creation() {
        let mut plot = Plot2D::default();
        assert_eq!(plot.x_label, Missing);

        plot.title(Some(String::from("an experiment")));
        plot.x_label(Required::new(String::from("label x")));
        plot.y_label(Required::new(String::from("label y")));
        plot.plot();

        assert_eq!(plot.title, Some(String::from("an experiment")));
        assert_eq!(plot.x_label, Required::new(String::from("label x")));
        assert_eq!(plot.y_label, Required::new(String::from("label y")));
    }
}