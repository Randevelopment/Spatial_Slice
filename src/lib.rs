mod primitives;

mod subspace;
mod subspace_mut;

pub use primitives::*;
pub use subspace::*;
pub use subspace_mut::*;

/// A Space represents a rectangular 2 dimensional array of contiguous
/// dynamically allocated memory
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Space<T> {
    /// The linear memory that the data is stored in
    data: Box<[T]>,

    /// The width (X direction) of the Space
    width: usize,

    /// The height (Y direction) of the Space
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

    /// Creates a space using the provided function,
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

    /// Creates a space by iterating through the given iterator
    /// This operation fails if the provided iterator does not contain enough data
    #[inline] 
    pub fn from_iter<I>(iter: &mut I, width: usize, height: usize) -> Option<Self> 
        where
            I: Iterator<Item=T> {

        let size = width * height;
        let mut vec = Vec::with_capacity(size);

        for _ in 0 .. size {
            if let Some(element) = iter.next() {
                vec.push(element);
            } else {
                return None;
            }
        }

        Some(Space {
            data: vec.into_boxed_slice(),
            width,
            height
        })
    }

    /// Creates a space by iterating through the given iterator
    /// This operation fails if the provided iterator does not contain enough data
    #[inline] 
    pub fn clone_from_iter<'a, I>(iter: &mut I, width: usize, height: usize) -> Option<Self> 
        where
            I: Iterator<Item=&'a T>,
            T: Clone + 'static {

        let size = width * height;
        let mut vec = Vec::with_capacity(size);

        for _ in 0 .. size {
            if let Some(element) = iter.next() {
                vec.push(element.clone());
            } else {
                return None;
            }
        }

        Some(Space {
            data: vec.into_boxed_slice(),
            width,
            height
        })
    }

    /// The width (X direction) of the Space
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// The height (Y direction) of the Space
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

    /// Sets the value for every position in the space
    /// based on its position
    /// 
    /// This function makes no guarantees about the order this occurs in
    #[inline]
    pub fn map(&mut self, mapper: fn(usize, usize) -> T) {
        let mut index = 0;

        for y in 0 .. self.height {
            for x in 0 .. self.width {
                self.data[index] = mapper(x, y);

                index += 1;
            }
        }
    }

    /// Updates the value for every position in the space
    /// based on its previous value and it's position
    /// 
    /// This function makes no guarantees about the order this occurs in
    #[inline]
    pub fn update(&mut self, updater: fn(&mut T, usize, usize)) {
        let mut index = 0;

        for y in 0 .. self.height {
            for x in 0 .. self.width {
                updater(&mut self.data[index], x, y);

                index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
