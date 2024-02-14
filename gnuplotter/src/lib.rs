//! # GnuPlotter
//!
//! GnuPlotter aims to enable the usage of the [gnuplot](http://www.gnuplot.info/) graphing utility
//! from within rust code. This library is in a very early stage and I am still actively considering
//! its use. It might not be a good productive use of my time to work on such a library as opposed
//! to manually writing gnuplot scripts. The main selling point is that this library allows the
//! definition of graphs within the rest of a codebase. This generally makes reasoning about the
//! whole easier. It also allows the generated plots to be described in how they semantically fit
//! into the larger whole of the codebase.
//!
//! ## Design goals
//!
//! There are only two major design goals at this point:
//! - easy access to gnuplot functionality
//! - easy customization to the needs of the engineering user
//!
//! ### Easy access to gnuplot functionality
//!
//! For engineers unfamiliar with gnuplot, it can be overwhelming to use the tool. It offers many
//! commands and options, most of which most users won't need. An interface in Rust that documents
//! the most commonly used functionality would make it much easier to use gnuplot at the expense of
//! some functionality.
//!
//! Additionally, an engineer may choose to extend this library to expand on the gnuplot functionality
//! made available through gnuplotter. This extension can then easily be used by other engineers in
//! their organization.
//!
//! To stress the last point, gnuplotter mainly provides an approach to offer easy access to gnuplot
//! from Rust code. It doesn't attempt to be complete, but offer a way to expand it so that most
//! engineers in an organization don't require gnuplot knowledge.
//!
//! ### Easy customization over prescribing a flexible API
//!
//!

pub use gnuplotter_core::required;
pub use gnuplotter_core::maybe;

pub mod prelude {
    pub use gnuplotter_macros::*;
    pub use gnuplotter_core::prelude::*;
    pub use gnuplotter_core::required;
}