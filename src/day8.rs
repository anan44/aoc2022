use crate::utils;

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<i32>>,
    max_x: usize,
    max_y: usize,
}

impl Forest {
    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        if self.tree_is_on_edge(x, y) {
            return true;
        }

        if self.tree_is_visible_from_left(x, y) {
            return true;
        }

        if self.tree_is_visible_from_right(x, y) {
            return true
        }

        if self.tree_is_visible_from_top(x, y) {
            return true
        }

        if self.tree_is_visible_from_bottom(x, y) {
            return true
        }

        return false;
    }
    fn tree_is_on_edge(&self, x: usize, y: usize) -> bool {
        return x == 0 ||
            y == 0 ||
            x == self.max_x ||
            y == self.max_y;
    }

    fn tree_is_visible_from_left(&self, x: usize, y: usize) -> bool {
        let tree_row = self.trees.get(y).unwrap();
        let trees_in_front = &tree_row[0..x];
        let tree_height = self.trees.get(y).unwrap().get(x).unwrap();
        return tree_is_visible(tree_height, trees_in_front);
    }

    fn tree_is_visible_from_right(&self, x: usize, y: usize) -> bool {
        let tree_row = self.trees.get(y).unwrap();
        let trees_in_front = &tree_row[x+1..self.max_x+1];
        let tree_height = self.trees.get(y).unwrap().get(x).unwrap();
        // println!("self: {}; x: {}; y: {}; front: {:?}", tree_height, x, y, trees_in_front);
        return tree_is_visible(tree_height, trees_in_front);
    }

    fn tree_is_visible_from_top(&self, x: usize, y: usize) -> bool {
        let tree_row: Vec<i32> = self.trees.iter()
            .map(|row| row.get(x).unwrap().clone())
            .collect();
        let trees_in_front = &tree_row[0..y];
        let tree_height = self.trees.get(y).unwrap().get(x).unwrap();
        return tree_is_visible(tree_height, trees_in_front);
    }

    fn tree_is_visible_from_bottom(&self, x: usize, y: usize) -> bool {
        let tree_row: Vec<i32> = self.trees.iter()
            .map(|row| row.get(x).unwrap().clone())
            .collect();
        let trees_in_front = &tree_row[y+1..self.max_y+1];
        let tree_height = self.trees.get(y).unwrap().get(x).unwrap();
        return tree_is_visible(tree_height, trees_in_front);
    }

    fn count_visible_trees(&self) -> i32 {
        let cords = get_cords(self.max_x + 1, self.max_y + 1);
        let visible_trees = cords.iter()
            .filter(|(x, y)| self.is_tree_visible(x.clone(), y.clone()))
            .count();
        return visible_trees as i32;
    }
}

fn tree_is_visible(height: &i32, trees_in_front: &[i32]) -> bool {
    let higher_trees: Vec<&i32> = trees_in_front.iter()
        .filter(|t| t.clone() >= height)
        .collect();

    let visible = higher_trees.len() == 0;
    return visible;

}

fn get_cords(x_len: usize, y_len: usize) -> Vec<(usize, usize)> {
    let mut cords: Vec<(usize, usize)> = Vec::new();
    for x in 0..x_len {
        for y in 0..y_len {
            let cord = (x, y);
            cords.push(cord);
        }
    }
    return cords;
}

fn row_to_numbers(row: String) -> Vec<i32> {
    return row.as_bytes()
        .into_iter()
        .map(|x| utils::byte_to_letter(x.clone())
            .parse()
            .unwrap())
        .collect();
}

fn new_forest(raw: Vec<String>) -> Forest {
    let trees: Vec<Vec<i32>> = raw.iter()
        .map(|x| row_to_numbers(x.clone()))
        .collect();

    let max_y = trees.len() - 1;
    let max_x = trees.get(0).unwrap().len() - 1;

    return Forest {
        trees,
        max_x,
        max_y,
    };
}

pub fn part1() {
    let raw = utils::read_lines("./inputs/day8.txt");
    let forest = new_forest(raw);
    println!("{:?}", forest.count_visible_trees())

}