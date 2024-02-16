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
//! One way to approach a plotting library would be to decide on a certain type of plot to support
//! and then to go implement a very flexible version of it. You would decide which features to
//! activate based on what methods are being called. For the engineers using such a library, there
//! is a lot of API details to familiarize themselves with. I asked myself the question how this
//! might be simplified. If I just want an x label, a y label, and a title, why do I care about the
//! rest of a fairly complex API?
//!
//! The goal of gnuplotter is therefore to hide as much of the implementation details of a plot type
//! as possible. One engineer might define a new plot type declaratively, while other engineers from
//! that point only need to concern themselves with this very minimal API.
//!
//! In other words, gnuplotter enables the composition of a new plot type based on a user's specific
//! needs, which we know very little about. The aim is to offer flexibility in the amount of
//! plotting features made available, balanced by an ability to simplify plotting API's based on
//! a very specific user's context.
//!
//! ## Solution
//!
//! ### Gnu commands
//!
//! Adding features to the library is easy. Just encode them in a struct that implements the
//! `GnuCommandFactory` trait. A `GnuCommand` is nothing more that a way to generate a string
//! representation of a command you would send to gnuplot yourself when using the CLI or a script.
//!
//! ### Composing commands
//!
//! These commands are then composed in a queue. A struct that consists of other elements, such as
//! the `Axis<T>` in the example linked below, or ADT's such as `Required<T>` implement the
//! `GnuCommandFactory` to compose the commands into a single queue.
//!
//! ### Rendering plots using gnuplot backend
//!
//! In the end, all collected commands are written to gnuplot by calling the gnuplot command from
//! within the rust program. This is made available through a simple `CanRender` trait that is
//! easily derived using the `Plot` macro.
//!
//! ## Examples
//!
//! ### [Basic demo](https://github.com/jamescauwelier/gnuplotter/blob/main/gnuplotter/examples/demo_basic.rs)
//!
//! A basic demonstration constructs a simple 2D plot with two labeled axis (X and Y) of which one
//! is optional. It shows both how to define a plot struct and how a user can use it.

pub use gnuplotter_core::required;
pub use gnuplotter_core::maybe;

pub mod prelude {
    pub use gnuplotter_macros::*;
    pub use gnuplotter_core::prelude::*;
    pub use gnuplotter_core::required;
}