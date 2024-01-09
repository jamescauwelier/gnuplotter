# GNUPlotter Macros

This crate contains the macros used to generate the plots using `gnuplot`. 
The macro's consider plots to be defined using structs that derive the `Plot` macro. This will both 
provided setter functionality for the plot as a builder would, but also provides validation functions
regarding the plot's content. Different from a builder, a plot is not necessarily produced / visualized
only once, and so it needs to build itself, so it can keep building by adding new data series (for example).

