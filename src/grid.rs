use std::ops::{Index, IndexMut};

use itertools::iproduct;

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

#[allow(unused)]
impl<T> Grid<T> {
    pub fn get(&self, x: impl TryInto<usize>, y: impl TryInto<usize>) -> Option<&T> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;

        (x < self.width).then_some(())?;
        (y < self.height).then_some(())?;

        Some(&self.data[x + y * self.width])
    }

    pub fn get_mut(
        &mut self,
        x: impl TryInto<usize>,
        y: impl TryInto<usize>,
    ) -> Option<&mut T> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;

        (x < self.width).then_some(())?;
        (y < self.height).then_some(())?;

        Some(&mut self.data[x + y * self.width])
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        self.data.iter_mut()
    }

    pub fn iter_coords<I: TryFrom<usize>>(
        &self,
    ) -> impl Iterator<Item = (I, I, &T)> + '_ {
        iproduct!(0..self.height, 0..self.width)
            .zip(self.data.iter())
            .map(|((y, x), e)| {
                (
                    x.try_into().ok().expect("index range too small."),
                    y.try_into().ok().expect("index range too small."),
                    e,
                )
            })
    }

    pub fn iter_coords_mut<I: TryFrom<usize>>(
        &mut self,
    ) -> impl Iterator<Item = (I, I, &mut T)> + '_ {
        iproduct!(0..self.height, 0..self.width)
            .zip(self.data.iter_mut())
            .map(|((y, x), e)| {
                (
                    x.try_into().ok().expect("index range too small."),
                    y.try_into().ok().expect("index range too small."),
                    e,
                )
            })
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> + '_ {
        self.data.chunks_exact(self.width)
    }
}

#[allow(unused)]
impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Self {
            width,
            height,
            data: vec![value; width * height],
        }
    }

    pub fn from_rows_columns(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), width * height);
        Self {
            width,
            height,
            data,
        }
    }
}

impl<T, I1, I2> Index<(I1, I2)> for Grid<T>
where
    I1: TryInto<usize>,
    I2: TryInto<usize>,
{
    type Output = T;

    fn index(&self, (x, y): (I1, I2)) -> &Self::Output {
        self.get(x, y).expect("Index out of bounds.")
    }
}

impl<T, I1, I2> IndexMut<(I1, I2)> for Grid<T>
where
    I1: TryInto<usize>,
    I2: TryInto<usize>,
{
    fn index_mut(&mut self, (x, y): (I1, I2)) -> &mut Self::Output {
        self.get_mut(x, y).expect("Index out of bounds.")
    }
}
