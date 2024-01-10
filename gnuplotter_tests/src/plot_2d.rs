use gnuplotter::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Default, Plot)]
pub struct Plot2D {
    title: Maybe<Title>,
    x_label: Required<XLabel>,
    y_label: Required<YLabel>,
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

        assert_eq!(plot.title, Maybe::Nothing);
        assert_eq!(plot.x_label, Missing);
        assert_eq!(plot.y_label, Missing);

        plot.title("an experiment");
        plot.x_label("label x");
        plot.y_label("label y");
        plot.plot();

        assert_eq!(plot.title, Maybe::value(Title::new("an experiment")));
        assert_eq!(plot.x_label, Required::value(XLabel::new("label x")));
        assert_eq!(plot.y_label, Required::value(YLabel::new("label y")));
    }
}