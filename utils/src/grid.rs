use std::{fmt, ops::{Index, IndexMut}, str::FromStr};

use crate::vector::Vec2;

pub type Pos = Vec2<isize>;

#[derive(Debug, PartialEq, Clone)]
pub struct Grid<T> {
    height: usize,
    width: usize,
    grid: Vec<T>
}

impl<T> Grid<T> {
    pub fn size(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    pub fn get(&self, p: &Pos) -> Option<&T> {
        if (p.y as usize) < self.height && (p.x as usize) < self.width {
            Some(&self[(p.y as usize, p.x as usize)])
        } else {
            None
        }
    }

    pub fn insert(&mut self, p: &Pos, v: T) {
        self[(p.y as usize, p.x as usize)] = v
    }

    pub fn find<'a>(&'a self, predicate: impl Fn(&T) -> bool + 'a) -> impl Iterator<Item=Pos> + 'a {
        self.grid
            .iter()
            .enumerate()
            .flat_map(move |(i, val)| {
                if predicate(val) {
                    Some(Pos{y: (i / self.width) as isize, x: (i % self.width) as isize})
                } else {
                    None
                }
            })
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self[(row, col)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Grid{height: value.len(), width: value[0].len(), grid: value.into_iter().flatten().collect()}
    }
}

impl FromStr for Grid<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s.lines()
            .map(|l| l.chars().collect())
            .collect();
        Ok(grid.into())
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.grid[row*self.width + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.grid[row*self.width + col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "12\n34\n";

    #[test]
    fn from_str_test() {
        let g_expected = Grid{
            height: 2,
            width: 2,
            grid: vec!['1', '2', '3', '4']
        };

        let g = Grid::from_str(TEST_STR).unwrap();
        assert_eq!(g, g_expected);
    }

    #[test]
    fn index_test() {
        let g = Grid::from_str(TEST_STR).unwrap();
        assert_eq!(g[(1, 1)], '4');
    }

    #[test]
    fn to_str_test() {
        let g = Grid::from_str(TEST_STR).unwrap();
        assert_eq!(g.to_string(), TEST_STR.to_string());
    }

    #[test]
    fn get_test() {
        let g = Grid::from_str(TEST_STR).unwrap();
        assert_eq!(g.get(&Pos::new(0, 0)), Some(&'1'));
        assert_eq!(g.get(&Pos::new(2, 0)), None);
    }

    #[test]
    fn insert_test() {
        let mut g = Grid::from_str(TEST_STR).unwrap();
        let p = Pos::new(1, 1);
        g.insert(&p, 'X');
        assert_eq!(g.get(&p), Some(&'X'));
    }

    #[test]
    fn find_test() {
        let g = Grid::from_str(TEST_STR).unwrap();
        assert_eq!(g.find(|x| x == &'2').collect::<Vec<_>>(), vec![Pos{y: 0, x:1}] );
    }

}

