//! Задание 5: вид справа на бинарное дерево поиска (обход в ширину).

use labs::queue::Queue;

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

fn insert(root: Option<Box<Node>>, val: i32) -> Option<Box<Node>> {
    match root {
        None => Some(Box::new(Node::new(val))),
        Some(mut node) => {
            if val < node.value {
                node.left = insert(node.left.take(), val);
            } else {
                node.right = insert(node.right.take(), val);
            }
            Some(node)
        }
    }
}

/// Последний узел каждого уровня, сверху вниз.
fn right_side_view(root: &Node) -> Vec<i32> {
    let mut view = Vec::new();
    let mut queue: Queue<&Node> = Queue::new();
    queue.push(root);

    while !queue.is_empty() {
        let level_size = queue.len();

        for i in 0..level_size {
            let Some(current) = queue.pop() else { continue };

            if i == level_size - 1 {
                view.push(current.value);
            }
            if let Some(left) = current.left.as_deref() {
                queue.push(left);
            }
            if let Some(right) = current.right.as_deref() {
                queue.push(right);
            }
        }
    }

    view
}

fn main() {
    let mut root: Option<Box<Node>> = None;
    for val in [8, 5, 10, 3, 9, 15, 12] {
        root = insert(root, val);
    }

    if let Some(root) = root.as_deref() {
        let items: Vec<String> = right_side_view(root).iter().map(i32::to_string).collect();
        println!("Результат: {}", items.join(", "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build(values: &[i32]) -> Option<Box<Node>> {
        values.iter().fold(None, |root, &v| insert(root, v))
    }

    #[test]
    fn sample_tree() {
        let root = build(&[8, 5, 10, 3, 9, 15, 12]);
        assert_eq!(right_side_view(root.as_deref().unwrap()), vec![8, 10, 15, 12]);
    }

    #[test]
    fn left_leaning_tree_shows_left_nodes() {
        let root = build(&[5, 3, 1]);
        assert_eq!(right_side_view(root.as_deref().unwrap()), vec![5, 3, 1]);
    }
}
