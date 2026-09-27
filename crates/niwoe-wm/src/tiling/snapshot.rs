//! Flat postorder interchange; callers own persistence and stable identities.
use super::{tree::Node, SplitDir, TilingLayout};
use smithay::desktop::Window;

pub enum LayoutNode<T> {
    Window(T),
    Split {
        dir: SplitDir,
        ratio: f32,
        left: usize,
        right: usize,
    },
}

impl TilingLayout {
    pub fn snapshot(&self) -> Vec<LayoutNode<Window>> {
        fn walk(node: &Node<Window>, result: &mut Vec<LayoutNode<Window>>) -> usize {
            let next = match node {
                Node::Leaf(w) => LayoutNode::Window(w.clone()),
                Node::Internal {
                    dir,
                    ratio,
                    left,
                    right,
                } => {
                    let left = walk(left, result);
                    let right = walk(right, result);
                    LayoutNode::Split {
                        dir: *dir,
                        ratio: *ratio,
                        left,
                        right,
                    }
                }
            };
            result.push(next);
            result.len() - 1
        }
        let mut result = Vec::new();
        if let Some(root) = &self.root {
            walk(root, &mut result);
        }
        result
    }

    /// Missing leaves collapse their parent. Invalid topology is rejected.
    pub fn restore(&mut self, saved: Vec<LayoutNode<Option<Window>>>) -> bool {
        let mut nodes: Vec<Option<Box<Node<Window>>>> = Vec::new();
        let mut used = vec![false; saved.len()];
        for (i, node) in saved.into_iter().enumerate() {
            let next = match node {
                LayoutNode::Window(w) => w.map(|w| Box::new(Node::Leaf(w))),
                LayoutNode::Split {
                    dir,
                    ratio,
                    left,
                    right,
                } => {
                    if left >= i
                        || right >= i
                        || left == right
                        || used[left]
                        || used[right]
                        || !ratio.is_finite()
                        || !(0.1..=0.9).contains(&ratio)
                    {
                        return false;
                    }
                    used[left] = true;
                    used[right] = true;
                    match (nodes[left].take(), nodes[right].take()) {
                        (Some(left), Some(right)) => Some(Box::new(Node::Internal {
                            dir,
                            ratio,
                            left,
                            right,
                        })),
                        (left, right) => left.or(right),
                    }
                }
            };
            nodes.push(next);
        }
        if used
            .iter()
            .enumerate()
            .any(|(i, used)| *used != (i + 1 < nodes.len()))
        {
            return false;
        }
        self.root = nodes.pop().flatten();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_leaves_collapse_without_inventing_windows() {
        let mut layout = TilingLayout::new();
        assert!(layout.restore(vec![
            LayoutNode::Window(None),
            LayoutNode::Window(None),
            LayoutNode::Split {
                dir: SplitDir::Vertical,
                ratio: 0.6,
                left: 0,
                right: 1
            },
        ]));
        assert!(layout.is_empty());
        assert!(layout.snapshot().is_empty());
    }

    #[test]
    fn invalid_topology_and_nonfinite_ratios_are_rejected() {
        let mut layout = TilingLayout::new();
        for ratio in [f32::NAN, f32::INFINITY, -1.0, 1.0] {
            assert!(!layout.restore(vec![
                LayoutNode::Window(None),
                LayoutNode::Window(None),
                LayoutNode::Split {
                    dir: SplitDir::Horizontal,
                    ratio,
                    left: 0,
                    right: 1
                },
            ]));
        }
        assert!(!layout.restore(vec![LayoutNode::Window(None), LayoutNode::Window(None)]));
        assert!(!layout.restore(vec![LayoutNode::Split {
            dir: SplitDir::Horizontal,
            ratio: 0.5,
            left: 0,
            right: 1
        }]));
        assert!(layout.is_empty());
    }
}
