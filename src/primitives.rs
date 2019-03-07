
/// A positioning type indicates how to interpret an X/Y coordinate in a slice
pub enum PostioningType {
    /// Absolute positioning indexes directly into the space that this slice references
    Absolute, 

    /// Relative positioning indexes into the slice,
    /// it treats the slices (x,y) values as the origin (0,0)
    /// therefore values must be offset by (x,y) to be interpretted absolutely
    Relative
}

/// Represents a partition with left and right values
pub struct HorizontalSplit<T> {
    pub left: T,
    pub right: T
}

/// Represents a partition with above and below values
pub struct VerticalSplit<T> {
    pub above: T,
    pub below: T
}