use std::marker::PhantomData;

/// A Space represents a rectangular 2 dimensional array of contiguous
/// dynamically allocated memory
pub struct Space<T> {
    data: Box<[T]>,
    width: usize,
    height: usize
}

impl<T> Space<T> {
    /// Creates a space full of the provided value,
    /// with the provided dimensions
    #[inline]
    pub fn new_flat(value: T, width: usize, height: usize) -> Self
        where T: Clone {

        Space {
            data: vec![ value; width * height ].into_boxed_slice(),
            width,
            height
        }
    }

    /// Creates a space full of the provided value,
    /// with the provided dimensions
    #[inline]
    pub fn new_mapped(func: fn(usize, usize) -> T, width: usize, height: usize) -> Self
        where T: Clone {

        let mut vec = Vec::with_capacity(width * height);

        for y in 0 .. height {
            for x in 0 .. width {
                vec.push(func(x,y));
            }
        }

        Space {
            data: vec.into_boxed_slice(),
            width,
            height
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Creates an immutable reference to an element at an absolute position
    /// in the space
    /// If the position specified is outside the space None is returned
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let index = y * self.width + x;

        self.data.get(index)
    }

    /// Creates a mutable reference to an element at an absolute position
    /// in the space
    /// If the position specified is outside the space None is returned
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = y * self.width + x;

        self.data.get_mut(index)
    }

    /// Sets the value for the specified absolute position in the space
    /// If the position specified is outside the space false is returned
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: T) -> bool {
        let index = y * self.width + x;

        if index < self.data.len() {
            self.data[index] = value;
            true
        } else {
            false
        }
    }

    /// Create a mutable slice representing the entire space
    #[inline]
    pub fn as_slice_mut(&mut self) -> SpaceSliceMut<'_, T> {
        SpaceSliceMut {
            parent: self,
            phantom: PhantomData,

            x: 0,
            y: 0,

            width: self.width,
            height: self.height
        }
    }
}

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

/// The data structure that represents a mutable view of a subspace
/// of some parent space
pub struct SpaceSliceMut<'a, T> {
    
    parent: *mut Space<T>,
    phantom: PhantomData<&'a mut Space<T>>,

    x: usize,
    y: usize,

    width: usize,
    height: usize
}

impl<'a, T> SpaceSliceMut<'a, T> {

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn convert_coord(&self, pos_type: PostioningType, x: usize, y: usize) -> Option<(usize, usize)> {
        match pos_type {
            PostioningType::Absolute =>  {
                if x < self.x 
                    || x >= self.x + self.width
                    || y < self.y
                    || y >= self.y + self.height {
                    return None;
                }

                Some((x, y))
            }
            PostioningType::Relative => Some((self.x + x, self.y + y))
        }
    }

    /// Creates an immutable reference to a value in this slice using 
    /// the specified addressing mode
    /// If the value queried is outside the slice None will be returned
    #[inline]
    pub fn get(&self, pos_type: PostioningType, x: usize, y: usize) -> Option<&T> {
        let (abs_x, abs_y) = self.convert_coord(pos_type, x, y)?;
        
        unsafe {
            (*self.parent).get(abs_x, abs_y)
        }
    }

    /// Sets the value for the specified absolute position in the space
    /// If the position specified is outside the space false is returned
    #[inline]
    pub fn set(&mut self, pos_type: PostioningType, x: usize, y: usize, value: T) -> bool {
        if let Some((abs_x, abs_y)) = self.convert_coord(pos_type, x, y) {
            unsafe {
                (*self.parent).set(abs_x, abs_y, value)
            }
        } else {
            false
        }
    }

    #[inline]
    pub fn split_horizontal(self, pos_type: PostioningType, x_value: usize) -> HorizontalSplit<SpaceSliceMut<'a, T>> {
        let left_x = self.x;

        let right_x = match pos_type {
            PostioningType::Absolute => x_value,
            PostioningType::Relative => self.x + x_value
        };

        if right_x > self.width {
            panic!("Invalid x value ({}) provided for slice with width {}", right_x, self.width);
        }
        
        let left_width = right_x - left_x;
        let right_width = self.width - left_width;

        HorizontalSplit {
            left: SpaceSliceMut {
                parent: self.parent,
                phantom: PhantomData,
                
                x: left_x,
                width: left_width,
                
                y: self.y,
                height: self.height
            },
            right: SpaceSliceMut {
                parent: self.parent,
                phantom: PhantomData,
                
                x: right_x,
                width: right_width,

                y: self.y,
                height: self.height
            }
        }
    }
    
    #[inline]
    pub fn split_vertical(self, pos_type: PostioningType, y_value: usize) -> VerticalSplit<SpaceSliceMut<'a, T>> {
        let above_y = self.y;

        let below_y = match pos_type {
            PostioningType::Absolute => y_value,
            PostioningType::Relative => self.y + y_value
        };

        if below_y > self.height {
            panic!("Invalid y value ({}) provided for slice with height {}", below_y, self.height);
        }
        
        let above_height = below_y - above_y;
        let below_height = self.height - above_height;

        VerticalSplit {
            above: SpaceSliceMut {
                parent: self.parent,
                phantom: PhantomData,
                
                y: above_y,
                height: above_height,
                
                x: self.x,
                width: self.width
            },

            below: SpaceSliceMut {
                parent: self.parent,
                phantom: PhantomData,
                
                y: below_y,
                height: below_height,
                
                x: self.x,
                width: self.width
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_split_width_check() {
        let mut space = Space::new_flat(1u32, 4, 4);
        let space_slice = space.as_slice_mut();

        let HorizontalSplit { left, right } = space_slice.split_horizontal(PostioningType::Absolute, 2);

        assert_eq!(left.width(), 2);
        assert_eq!(right.width(), 2);
    }
    
    #[test]
    fn vertical_split_height_check() {
        let mut space = Space::new_flat(1u32, 4, 4);
        let space_slice = space.as_slice_mut();

        let VerticalSplit { above, below } = space_slice.split_vertical(PostioningType::Absolute, 2);

        assert_eq!(above.height(), 2);
        assert_eq!(below.height(), 2);
    }

    #[test]
    fn new_flat_test() {
        let side_length = 100;

        let space = Space::new_flat(true, side_length, side_length);

        for y in 0 .. side_length {
            for x in 0 .. side_length {
                assert!(space.get(x, y).unwrap());
            }
        }
    }

    #[test]
    fn new_mapped_test() {
        let side_length = 100;

        let space = Space::new_mapped(|x, y| (x,y), side_length, side_length);

        for y in 0 .. side_length {
            for x in 0 .. side_length {
                assert_eq!(*space.get(x, y).unwrap(), (x,y));
            }
        }
    }
}
