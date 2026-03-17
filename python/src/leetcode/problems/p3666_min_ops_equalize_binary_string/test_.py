from leetcode.problems.p3666_min_ops_equalize_binary_string import Solution


def test_examples():
    example_strings = [
        "110",
        "0101",
        "101",
    ]
    example_k = [1, 3, 2]
    answers = [1, 2, -1]
    for idx, (s, k, expected) in enumerate(zip(example_strings, example_k, answers)):
        assert Solution().minOperations(s, k) == expected, f"Example {idx + 1} incorrect."

def test_1():
    assert Solution().minOperations("0", 1) == 1, "Obvious"
