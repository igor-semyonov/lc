from typing import Self


class TreeNode:
    def __repr__(self):
        return f"TreeNode(val={self.val}, left={self.left}, right={self.right})"

    def __eq__(self, other: Self):
        return walk_tree(self) == walk_tree(other)

    def __init__(
        self,
        val=0,
        left=None,
        right=None,
    ):
        self.val = val
        self.left = left
        self.right = right
        if left is not None:
            assert (
                left.val <= val
            ), f"Left child node {left.val} should be at most parent node {val}."
        if right is not None:
            assert (
                val <= right.val
            ), f"right child node {right.val} should be at most parent node {val}."


class Solution:
    def balanceBST(self, root: TreeNode):
        return balance_bst(root)


def walk_tree(root: TreeNode):
    if root is not None:
        elements = []
        left_walk = walk_tree(root.left)
        if left_walk is not None:
            elements += left_walk
        elements.append(root.val)
        right_walk = walk_tree(root.right)
        if right_walk is not None:
            elements += right_walk
        return elements


def tree_depth(root):
    if root.left is None:
        left_depth = 0
    else:
        left_depth = 1 + tree_depth(root.left)

    if root.right is None:
        right_depth = 0
    else:
        right_depth = 1 + tree_depth(root.right)

    return max(left_depth, right_depth)


def balance_bst(root: TreeNode) -> TreeNode:
    def balance_list(elements: list) -> TreeNode | None:
        if len(elements) == 0:
            return None
        idx_mid = len(elements) // 2
        return TreeNode(
            elements[idx_mid],
            balance_list(elements[:idx_mid]),
            balance_list(elements[idx_mid + 1 :]),
        )

    elements = walk_tree(root)
    return balance_list(elements)


def main():
    root = TreeNode(
        3,
        TreeNode(
            1,
            TreeNode(0, None, None),
            TreeNode(2, None, None),
        ),
        TreeNode(
            5,
            TreeNode(4, None, None),
            TreeNode(6, None, None),
        ),
    )
    root2 = TreeNode(
        0,
        None,
        TreeNode(
            1,
            None,
            TreeNode(
                2,
                None,
                TreeNode(
                    3,
                    None,
                    TreeNode(
                        4,
                        None,
                        TreeNode(
                            5,
                            None,
                            TreeNode(
                                6,
                                None,
                                None,
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )

    left_elements = walk_tree(root)
    print(left_elements)
    print(walk_tree(root2))

    depth = tree_depth(root)
    print(depth)
    depth = tree_depth(root2)
    print(depth)

    root3 = balance_bst(root2)
    print(root3 == root)


if __name__ == "__main__":
    main()
