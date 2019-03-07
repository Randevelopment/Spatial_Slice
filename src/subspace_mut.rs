use std::marker::PhantomData;

use crate::Space;
use crate::primitives::*;

/// The data structure that represents a mutable view of a subspace
/// of some parent space
pub struct SubSpaceMut<'a, T> {
    /// The space that this SubSpace is from
    parent: *mut Space<T>,

    /// Assists the borrow checker in preventing race conditions
    phantom: PhantomData<&'a mut Space<T>>,

    /// The X position that this SubSpace starts at
    x: usize,

    /// The Y position that this SubSpace starts at
    y: usize,

    /// The width (X direction) of this SubSpace
    width: usize,

    /// The height (Y direction) of this SubSpace
    height: usize
}

impl<T> Space<T> {
    /// Create a mutable slice representing the entire space
    /// 
    /// Two mutable subspaces cannot coexist as that would allow data racaes
    /// 
    /// ```compile_fail
    /// let space = Space::new_flat(true, 10, 10);
    /// let subspace1 = space.as_subspace_mut();
    /// let subspace2 = space.as_subspace_mut();
    /// ```
    #[inline]
    pub fn as_subspace_mut(&mut self) -> SubSpaceMut<'_, T> {
        SubSpaceMut {
            parent: self,
            phantom: PhantomData,

            x: 0,
            y: 0,

            width: self.width,
            height: self.height
        }
    }
}

impl<'a, T> SubSpaceMut<'a, T> {

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
                    
                    None
                } else {
                    Some((x, y))
                }
            }
            PostioningType::Relative => {
                if x > self.width || y > self.height {
                    None
                } else {
                    Some((self.x + x, self.y + y))
                }
            }
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

    /// Splits this SubSpaceMut into two new ones horizontally
    /// The left subspace contains all the points in this one that have x less than the given x_value
    /// The right subspace contains all the points in this one that have x greater than or equal to the given x_value
    #[inline]
    pub fn split_horizontal(self, pos_type: PostioningType, x_value: usize) -> HorizontalSplit<SubSpaceMut<'a, T>> {
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
            left: SubSpaceMut {
                parent: self.parent,
                phantom: PhantomData,
                
                x: left_x,
                width: left_width,
                
                y: self.y,
                height: self.height
            },
            right: SubSpaceMut {
                parent: self.parent,
                phantom: PhantomData,
                
                x: right_x,
                width: right_width,

                y: self.y,
                height: self.height
            }
        }
    }
    
    /// Splits this SubSpaceMut into two new ones vertically
    /// The above subspace contains all the points in this one that have y less than the given y_value
    /// The below subspace contains all the points in this one that have y greater than or equal to the given y_value
    #[inline]
    pub fn split_vertical(self, pos_type: PostioningType, y_value: usize) -> VerticalSplit<SubSpaceMut<'a, T>> {
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
            above: SubSpaceMut {
                parent: self.parent,
                phantom: PhantomData,
                
                y: above_y,
                height: above_height,
                
                x: self.x,
                width: self.width
            },

            below: SubSpaceMut {
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
        let space_slice = space.as_subspace_mut();

        let HorizontalSplit { left, right } = space_slice.split_horizontal(PostioningType::Absolute, 2);

        assert_eq!(left.width(), 2);
        assert_eq!(right.width(), 2);
    }
    
    #[test]
    fn vertical_split_height_check() {
        let mut space = Space::new_flat(1u32, 4, 4);
        let space_slice = space.as_subspace_mut();

        let VerticalSplit { above, below } = space_slice.split_vertical(PostioningType::Absolute, 2);

        assert_eq!(above.height(), 2);
        assert_eq!(below.height(), 2);
    }
}