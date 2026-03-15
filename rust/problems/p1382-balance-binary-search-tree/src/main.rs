use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    #[rustfmt::skip]
    let root = TreeNode{
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode{
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 6,
                left: None,
                right: None,
            }))),
        }))),
    };

    let sorted_elements = walk_tree(&root);
    println!("{sorted_elements:?}");

    let mut root = TreeNode {
        val: 10,
        left: None,
        right: None,
    };
    insert(
        &mut root, 5,
    );
    insert(
        &mut root, 6,
    );
    insert(
        &mut root, 8,
    );
    insert(
        &mut root, 9,
    );
    insert(
        &mut root, 52,
    );
    insert(
        &mut root, 58,
    );
    root.insert(42);

    let sorted_elements = walk_tree(&root);
    println!("{sorted_elements:?}");

    println!(
        "{}",
        root.depth()
    );
    #[rustfmt::skip]
    let root = TreeNode {
        val: 5,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode{
            val: 6,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 7,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 8,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 9,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode{
                            val: 10,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode{
                                val: 11,
                                left: None,
                                right: None,
                            }))),
                        }))),
                    }))),
                }))),
            }))),
        }))),
    };

    println!(
        "{:?}",
        root.walk_tree()
    );

    println!(
        "{}",
        root.depth()
    );
    let balanced_root = root.balance();
    println!(
        "{}",
        balanced_root.depth()
    );
    println!(
        "{:?}",
        balanced_root.walk_tree()
    );
}

#[derive(Clone, Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}
#[allow(dead_code)]
impl TreeNode {
    fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        Self {
            val,
            left,
            right,
        }
    }

    /// the -1's are so that the depth of a 1 node tree is 0. Change the -1's to 0's to set the
    /// depth of a 1 node tree to 1. In this case, you can also remove the `as i32`
    fn depth(&self) -> u32 {
        (1 + match &self.left {
            None => -1,
            Some(left) => left
                .borrow()
                .depth() as i32,
        }
        .max(
            match &self.right {
                None => -1,
                Some(right) => right
                    .borrow()
                    .depth() as i32,
            },
        )) as u32
    }

    fn walk_tree(&self) -> Vec<i32> {
        fn walk_tree_borrowed<T>(
            root: T,
            result: &mut Vec<i32>,
        ) where
            T: std::ops::Deref<Target = TreeNode>,
        {
            if let Some(left) = &root.left {
                walk_tree_borrowed(
                    left.borrow(),
                    result,
                );
            }
            result.push(root.val);

            if let Some(right) = &root.right {
                walk_tree_borrowed(
                    &*right.borrow(),
                    result,
                );
            }
        }

        let mut result = vec![];
        walk_tree_borrowed(
            self,
            &mut result,
        );
        result
    }

    fn insert(&mut self, new_value: i32) {
        if new_value < self.val {
            if let Some(left) = &self.left {
                insert(
                    left.borrow_mut(),
                    new_value,
                );
            } else {
                self.left = Some(
                    Rc::new(
                        RefCell::new(
                            TreeNode {
                                val: new_value,
                                left: None,
                                right: None,
                            },
                        ),
                    ),
                );
            }
        } else {
            if let Some(right) = &self.right {
                insert(
                    right.borrow_mut(),
                    new_value,
                );
            } else {
                self.right = Some(
                    Rc::new(
                        RefCell::new(
                            TreeNode {
                                val: new_value,
                                left: None,
                                right: None,
                            },
                        ),
                    ),
                );
            }
        }
    }

    fn balance(self) -> Self {
        fn balance_list(
            elements: &[i32],
        ) -> Option<TreeNode> {
            if elements.len() == 0 {
                return None;
            }
            let idx_mid = elements.len() / 2;
            Some(
                TreeNode {
                    val: elements[idx_mid],
                    left: balance_list(
                        &elements[..idx_mid],
                    )
                    .map(|t| Rc::new(RefCell::new(t))),
                    right: balance_list(
                        &elements[idx_mid + 1..],
                    )
                    .map(|t| Rc::new(RefCell::new(t))),
                },
            )
        }
        let elements = self.walk_tree();
        balance_list(&elements).unwrap()
    }

    #[allow(
        dead_code,
        unused_variables
    )]
    fn find(&self, val: i32) -> Option<&Self> {
        None
    }

    // fn delete(&mut self, del_value: i32) {
    //     let mut target_node =
    // Rc::new(RefCell::new(self));     loop {
    //         if del_value == target_node.borrow().val {
    //             break;
    //         } else if del_value <
    // target_node.borrow().val {             if let
    // Some(left) = target_node.borrow().left {
    //                 target_node = left;
    //             } else {
    //                 return;
    //             }
    //         } else {
    //         }
    //     }
    // }
}

fn walk_tree(root: &TreeNode) -> Vec<i32> {
    fn walk_tree_borrowed<T>(
        inner_root: T,
        result: &mut Vec<i32>,
    ) where
        T: std::ops::Deref<Target = TreeNode>,
    {
        if let Some(left) = &inner_root.left {
            walk_tree_borrowed(
                left.borrow(),
                result,
            );
        }
        result.push(inner_root.val);

        if let Some(right) = &inner_root.right {
            walk_tree_borrowed(
                &*right.borrow(),
                result,
            );
        }
    }

    let mut result = vec![];
    walk_tree_borrowed(
        root,
        &mut result,
    );
    result
}

fn insert<T>(mut root: T, new_value: i32)
where
    T: std::ops::DerefMut<Target = TreeNode>,
{
    if new_value < root.val {
        if let Some(left) = &root.left {
            insert(
                left.borrow_mut(),
                new_value,
            );
        } else {
            root.left = Some(
                Rc::new(
                    RefCell::new(
                        TreeNode {
                            val: new_value,
                            left: None,
                            right: None,
                        },
                    ),
                ),
            );
        }
    } else {
        if let Some(right) = &root.right {
            insert(
                right.borrow_mut(),
                new_value,
            );
        } else {
            root.right = Some(
                Rc::new(
                    RefCell::new(
                        TreeNode {
                            val: new_value,
                            left: None,
                            right: None,
                        },
                    ),
                ),
            );
        }
    }
}

#[cfg(test)]
mod tests;
