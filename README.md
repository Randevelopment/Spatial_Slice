# Spatial_Slice

Spacial Slice is a simple Rust crate for when you want to store two dimensional `Sized` data in linear memory.

Spacial Slice has a Space type that represents this data, and supports (X, Y) coordinate based access.
Additionally it has a SpaceSliceMut type that represents a mutable partition of the 2d space.
