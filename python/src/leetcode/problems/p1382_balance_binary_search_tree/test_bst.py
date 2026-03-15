from leetcode.problems.p1382_balance_binary_search_tree.main import TreeNode


def test_walk():
    root = TreeNode(0, None, None)
    root.left = TreeNode(
        -2,
        TreeNode(-3, None, None),
        TreeNode(-1, None, None),
    )
    root.right = TreeNode(
        2,
        TreeNode(1, None, None),
        TreeNode(3, None, None),
    )

    elements = root.walk()
    assert elements == list(range(-3, 4)), ""


def test_depth():
    root = TreeNode(0, None, None)
    root.left = TreeNode(
        -2,
        TreeNode(-3, None, None),
        TreeNode(-1, None, None),
    )
    root.right = TreeNode(
        2,
        TreeNode(1, None, None),
        TreeNode(3, None, None),
    )
    assert root.depth() == 2, "The depth should be 2"


def test_balance():
    root = TreeNode(
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
                        ),
                    ),
                ),
            ),
        ),
    )

    assert root.walk() == root.balance().walk(), "same elements before and after balancing"
    assert root.depth() == 5, "Unbalanced depth is 5"
    assert root.balance().depth() == 2, "balanced depth is 2"
