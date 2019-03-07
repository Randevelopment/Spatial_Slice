use crate::Space;
use crate::primitives::*;

/// The data structure that represents a read-only view of a subspace
/// of some parent space
#[derive(Debug, Clone)]
pub struct SubSpace<'a, T> {
    /// The space that this SubSpace is from
    parent: &'a Space<T>,

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
    /// Create a read only slice representing the entire space
    #[inline]
    pub fn as_subspace(&self) -> SubSpace<'_, T> {
        SubSpace {
            parent: self,

            x: 0,
            y: 0,

            width: self.width,
            height: self.height
        }
    }
}

impl<'a, T> SubSpace<'a, T> {
    /// The width (X direction) of this SubSpace
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// The height (Y direction) of this SubSpace
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

        self.parent.get(abs_x, abs_y)
    }

    /// Creates an iterator that reads through the SubSpace lexicographically
    pub fn iter(&self) -> SubSpaceIter<'_, T> {
        SubSpaceIter {
            parent: self,
            x: 0,
            y: 0
        }
    }

    pub fn as_space(&'a self) -> Space<T>
        where
            T: Clone + 'static {

        let mut iter = self.iter();

        Space::clone_from_iter(&mut iter, self.width, self.height).unwrap()
    }

    /// Splits this SubSpace into two new ones horizontally
    /// The left subspace contains all the points in this one that have x less than the given x_value
    /// The right subspace contains all the points in this one that have x greater than or equal to the given x_value
    #[inline]
    pub fn split_horizontal(&self, pos_type: PostioningType, x_value: usize) -> HorizontalSplit<SubSpace<'a, T>> {
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
            left: SubSpace {
                parent: self.parent,
                
                x: left_x,
                width: left_width,
                
                y: self.y,
                height: self.height
            },
            right: SubSpace {
                parent: self.parent,
                
                x: right_x,
                width: right_width,

                y: self.y,
                height: self.height
            }
        }
    }
    
    /// Splits this SubSpace into two new ones vertically
    /// The above subspace contains all the points in this one that have y less than the given y_value
    /// The below subspace contains all the points in this one that have y greater than or equal to the given y_value
    #[inline]
    pub fn split_vertical(&self, pos_type: PostioningType, y_value: usize) -> VerticalSplit<SubSpace<'a, T>> {
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
            above: SubSpace {
                parent: self.parent,
                
                y: above_y,
                height: above_height,
                
                x: self.x,
                width: self.width
            },

            below: SubSpace {
                parent: self.parent,
                
                y: below_y,
                height: below_height,
                
                x: self.x,
                width: self.width
            }
        }
    }
}

pub struct SubSpaceIter<'a, T> {
    parent: &'a SubSpace<'a, T>,

    x: usize,
    y: usize
}

impl<'a, T> Iterator for SubSpaceIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.parent.get(PostioningType::Relative, self.x, self.y);

        if self.x == self.parent.width - 1 {
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_test() {
        let space = Space::new_mapped(|x, y| 10 * (y as u32) + (x as u32), 10, 10);
        let subspace = space.as_subspace();
        let subspace_iter = subspace.iter().map(|v| *v);

        let counter = 0 .. 100u32;

        assert!(subspace_iter.eq(counter));
    }

    #[test]
    fn horizontal_split_width_check() {
        let space = Space::new_flat(1u32, 4, 4);
        let space_slice = space.as_subspace();

        let HorizontalSplit { left, right } = space_slice.split_horizontal(PostioningType::Absolute, 2);

        assert_eq!(left.width(), 2);
        assert_eq!(right.width(), 2);
    }
    
    #[test]
    fn vertical_split_height_check() {
        let space = Space::new_flat(1u32, 4, 4);
        let space_slice = space.as_subspace();

        let VerticalSplit { above, below } = space_slice.split_vertical(PostioningType::Absolute, 2);

        assert_eq!(above.height(), 2);
        assert_eq!(below.height(), 2);
    }

    #[test]
    fn partition_test() {
        let space = Space::new_mapped(|x, _| x < 10, 20, 20);
        let subspace = space.as_subspace();

        let HorizontalSplit { left, right } = subspace.split_horizontal(PostioningType::Absolute, 10);

        assert!(left.iter().all(|v| *v));
        assert!(right.iter().all(|v| !*v));
    }

    #[test]
    fn clone_test() {
        let original = Space::new_mapped(|x, y| (x, y), 100, 100);

        let cloned = original.clone();

        let round_trip = original.as_subspace().as_space();

        assert_eq!(original, cloned);
        assert_eq!(original, round_trip);
    }
}
