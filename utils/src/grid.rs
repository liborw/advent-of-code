use std::{fmt, ops::Index, str::FromStr};


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Pos {
    row: usize,
    col: usize
}

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

    pub fn get(&self, p: Pos) -> Option<&T> {
        if p.row < self.height && p.col < self.width {
            Some(&self[(p.row, p.col)])
        } else {
            None
        }
    }

    pub fn find<'a>(&'a self, predicate: impl Fn(&T) -> bool + 'a) -> impl Iterator<Item=Pos> + 'a {
        self.grid
            .iter()
            .enumerate()
            .flat_map(move |(i, val)| {
                if predicate(val) {
                    Some(Pos{row: i / self.width, col: i % self.width})
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
    fn find_test() {
        let g = Grid::from_str(TEST_STR).unwrap();
        assert_eq!(g.find(|x| x == &'2').collect::<Vec<_>>(), vec![Pos{row: 0, col:1}] );

    }

}

