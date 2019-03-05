
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

    pub fn as_slice(&self) -> SpaceSlice<T> {
        SpaceSlice {
            parent: self,

            x: 0,
            y: 0,

            width: self.width,
            height: self.height
        }
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

pub struct SpaceSlice<T> {

    parent: *const Space<T>,

    x: usize,
    y: usize,

    width: usize,
    height: usize
}

impl<T> SpaceSlice<T> {

}

pub struct SpaceSliceMut<T> {
    
    parent: *mut Space<T>,

    x: usize,
    y: usize,

    width: usize,
    height: usize
}

impl<T> SpaceSliceMut<T>
    where T: Clone {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
