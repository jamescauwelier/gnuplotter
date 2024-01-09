# GNUPlotter Core

The core functionality resides here. It is isolated because it needs to be pulled into the `gnuplotter_macros` 
crates without creating a circular dependency, and still allowing core functionality to be exported.