use std::array;

use super::{aabb::Aabb, entry::Entry};

pub struct Grid<const X: usize, const Y: usize> {
    entries: Vec<Entry>,
    cells: [[Vec<usize>; X]; Y],
    aabb: Aabb,
}

impl<const X: usize, const Y: usize> Grid<X, Y> {
    pub fn new(entries: Vec<Entry>) -> Self {
        let aabb = Aabb::merged(entries.iter().map(|e| e.aabb)).unwrap_or_default();
        let mut cells : [[Vec<usize>; X]; Y] = array::from_fn(|_| array::from_fn(|_| vec![]));

        let cell_width = aabb.extent().x / X as f64;
        let cell_height = aabb.extent().y / Y as f64;

        for (index, entry) in entries.iter().enumerate() {
            let min_x = ((entry.aabb.left() - aabb.left()) / cell_width).floor() as usize;
            let mut max_x = ((entry.aabb.right() - aabb.left()) / cell_width).floor() as usize;
            let min_y = ((entry.aabb.bottom() - aabb.bottom()) / cell_height).floor() as usize;
            let mut max_y = ((entry.aabb.top() - aabb.bottom()) / cell_height).floor() as usize;

            if max_x == X {
                max_x -= 1;
            }
            if max_y == X {
                max_y -= 1;
            }

            for x in min_x..max_x+1 {
                for y in min_y..max_y+1 {
                    cells[x][y].push(index);
                }
            }
        }

        Self {
            entries,
            cells,
            aabb,
        }
    }

    pub fn collect_collisions(&self, found_collisions : &mut Vec<(usize, usize)>) {
        for row in &self.cells {
            for cell in row {
                for i in 0..cell.len() {
                    for j in 0..cell.len() {
                        if i == j {
                            continue;
                        }

                        let lhs = &self.entries[cell[i]];
                        let rhs = &self.entries[cell[j]];
                        if lhs.aabb.intersects(&rhs.aabb) {
                            found_collisions.push((lhs.id, rhs.id));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_grow() {
        let e = Entry::default();
        let _grid = Grid::<3,3>::new(vec![e]);
    }
}
