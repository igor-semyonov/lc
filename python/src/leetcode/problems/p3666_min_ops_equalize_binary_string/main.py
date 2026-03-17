class Solution:
    def minOperations(self, s: str, k: int) -> int:
        length = len(s)
        ones = s.count("1")
        zeros = length - ones

        #  possible_moves = list(range(-k, k + 1, 2))

        if zeros == 0:
            return 0

        if zeros == length and k == length:
            return 1

        if k == length and ones != length:
            return -1

        zeros_q_k = zeros // k
        zeros_r_k = zeros % k
        if k % 2 == 0:  #  k is even
            if zeros % 2 == 0:  #  even number of zeros
                if zeros >= k:
                    return zeros_q_k + (zeros_r_k > 0)
                else:
                    return 2
            else:  #  odd number of zeros
                return -1
        else:  #  k is odd
            if zeros % 2 == 0:  #  even number of zeros
                if k > zeros:
                    return 2
                elif k < zeros:
                    if zeros_q_k % 2 == 0:
                        return zeros_q_k + (zeros_r_k > 0) * 2
                    else:
                        return zeros_q_k + 1
                else:
                    pass  #  can't happen since k is odd and zeros is even
            else:  #  odd number of zeros
                return zeros_q_k + (zeros_r_k > 0)
