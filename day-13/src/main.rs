struct Grid {
    grid: Vec<bool>,
    width: i32,
    height: i32,
}

impl Grid {
    fn parse(grid_str: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut grid = Vec::new();
        for line in grid_str.lines() {
            for tile in line.chars() {
                grid.push(tile == '#');
            }
            width = line.len() as i32;
            height += 1;
        }
        Self { grid, width, height }
    }

    fn get(&self, x: i32, y: i32) -> bool {
        assert!(x >= 0 && x < self.width && y >= 0 && y < self.height);
        self.grid[(y * self.width + x) as usize]
    }

    fn col_diffs(&self, x1: i32, x2: i32) -> usize {
        if x1 < 0 || x1 >= self.width || x2 < 0 || x2 >= self.width {
            return 0;
        }
        (0..self.height).filter(|&y| self.get(x1, y) != self.get(x2, y)).count()
    }

    fn row_diffs(&self, y1: i32, y2: i32) -> usize {
        if y1 < 0 || y1 >= self.height || y2 < 0 || y2 >= self.height {
            return 0;
        }
        (0..self.width).filter(|&x| self.get(x, y1) != self.get(x, y2)).count()
    }
}

fn mirror_point(target_smudges: usize, len: i32, diffs: impl Fn(i32, i32) -> usize) -> Option<i32> {
    for i in 0..len - 1 {
        let mut smudges = 0;
        let mut left = i;
        let mut right = i + 1;
        while left >= 0 && right < len {
            smudges += diffs(left, right);
            left -= 1;
            right += 1;
        }
        if smudges == target_smudges {
            return Some(i);
        }
    }
    None
}

fn mirror_summary(grids: &str, smudges: usize) -> i32 {
    let mut sum = 0;
    for grid in grids.split("\n\n") {
        let grid = Grid::parse(grid);
        if let Some(x) = mirror_point(smudges, grid.width, |x1, x2| grid.col_diffs(x1, x2)) {
            sum += x + 1;
            continue;
        }
        if let Some(y) = mirror_point(smudges, grid.height, |y1, y2| grid.row_diffs(y1, y2)) {
            sum += (y + 1) * 100;
            continue;
        }
    }
    sum
}

fn part_1(input: String) -> i32 {
    mirror_summary(&input, 0)
}

fn part_2(input: String) -> i32 {
    mirror_summary(&input, 1)
}

aoc::main!();
