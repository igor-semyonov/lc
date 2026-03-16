from itertools import chain, product
from typing import List


class Solution:
    def getBiggestThree(self, grid: List[List[int]]) -> List[int]:
        rhombus_max_sums = []
        n_cols = len(grid[0])
        n_rows = len(grid)
        for idx_r_top, idx_c_top in product(
            range(n_rows),
            range(n_cols),
        ):
            rhombus_max_size = min(
                [
                    n_cols - 1 - idx_c_top,  #  right
                    (n_rows - 1 - idx_r_top) // 2,  #  bottom
                    idx_c_top,  #  left
                ]
            )
            rhombus_sums = [
                sum(
                    [
                        grid[idx_r][idx_c]
                        for idx_r, idx_c in chain(
                            zip(  #  top -> right
                                range(
                                    idx_r_top,
                                    idx_r_top + rhombus_size,
                                ),
                                range(
                                    idx_c_top,
                                    idx_c_top + rhombus_size,
                                ),
                            ),
                            zip(  #  right -> bottom
                                range(
                                    idx_r_top + rhombus_size,
                                    idx_r_top + 2 * rhombus_size,
                                ),
                                range(
                                    idx_c_top + rhombus_size,
                                    idx_c_top,
                                    -1,
                                ),
                            ),
                            zip(  #  bottom -> left
                                range(
                                    idx_r_top + 2 * rhombus_size,
                                    idx_r_top + rhombus_size,
                                    -1,
                                ),
                                range(
                                    idx_c_top,
                                    idx_c_top - rhombus_size,
                                    -1,
                                ),
                            ),
                            zip(  #  left -> top
                                range(
                                    idx_r_top + rhombus_size,
                                    idx_r_top,
                                    -1,
                                ),
                                range(
                                    idx_c_top - rhombus_size,
                                    idx_c_top,
                                ),
                            ),
                        )
                    ]
                )
                for rhombus_size in range(1, rhombus_max_size + 1)
            ]
            rhombus_sums.append(grid[idx_r_top][idx_c_top])

            for rhombus_sum in rhombus_sums:
                if len(rhombus_max_sums) == 0:
                    rhombus_max_sums.append(rhombus_sum)
                else:
                    min_rhombus_max_sum = min(rhombus_max_sums)
                    if rhombus_sum > min_rhombus_max_sum:
                        if len(rhombus_max_sums) < 3:
                            rhombus_max_sums.append(rhombus_sum)
                        else:
                            rhombus_max_sums[
                                rhombus_max_sums.index(min(rhombus_max_sums))
                            ] = rhombus_sum

        rhombus_max_sums.sort()
        rhombus_max_sums.reverse()
        return rhombus_max_sums


def main():
    pass


if __name__ == "__main__":
    main()
