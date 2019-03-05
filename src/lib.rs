
pub struct Space<T> {
    data: Box<[T]>,
    width: usize,
    height: usize
}

impl<T> Space<T> {

    pub fn new(value: T, width: usize, height: usize) -> Self
        where T: Clone {

        Space {
            data: vec![ value; width * height ].into_boxed_slice(),
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

    pub fn as_slice_mut(&mut self) -> SpaceSliceMut<T> {
        SpaceSliceMut {
            parent: self,

            x: 0,
            y: 0,

            width: self.width,
            height: self.height
        }
    }
}

pub enum PostioningType {
    Absolute, Relative
}

pub struct HorizontalSplit<T> {
    left: T,
    right: T
}

pub struct VerticalSplit<T> {
    above: T,
    below: T
}

pub struct SpaceSliceMut<T> {
    
    parent: *mut Space<T>,

    x: usize,
    y: usize,

    width: usize,
    height: usize
}

impl<T> SpaceSliceMut<T> {
    pub fn split_horizontal(self, pos_type: PostioningType, x_value: usize) -> HorizontalSplit<SpaceSliceMut<T>> {
        let left_x = self.x;

        let right_x = match pos_type {
            PostioningType::Absolute => x_value,
            PostioningType::Relative => self.x + x_value
        };
        
        let left_width = right_x - left_x;
        let right_width = self.width - left_width;

        HorizontalSplit {
            left: SpaceSliceMut {
                parent: self.parent,
                
                x: left_x,
                width: left_width,
                
                y: self.y,
                height: self.height
            },
            right: SpaceSliceMut {
                parent: self.parent,
                
                x: right_x,
                width: right_width,

                y: self.y,
                height: self.height
            }
        }
    }
    
    pub fn split_vertical(self, pos_type: PostioningType, y_value: usize) -> VerticalSplit<SpaceSliceMut<T>> {
        match pos_type {
            PostioningType::Absolute => {

            },

            PostioningType::Relative => {

            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
