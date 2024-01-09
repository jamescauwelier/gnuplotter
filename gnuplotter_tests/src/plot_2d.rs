use gnuplotter::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, Plot)]
pub struct Plot2D {
    title: Option<String>,
    x_label: Option<String>,
    y_label: Option<String>,
}

impl Plot2D {
    pub fn new() -> Self {
        Plot2D::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_creation() {
        use super::*;

        let mut plot = Plot2D::default();
        plot.title(Some(String::from("an experiment")));
        plot.x_label(Some(String::from("label x")));
        plot.y_label(Some(String::from("label y")));
        plot.plot();

        assert_eq!(plot.title, Some(String::from("an experiment")));
        assert_eq!(plot.x_label, Some(String::from("label x")));
        assert_eq!(plot.y_label, Some(String::from("label y")));
    }
}