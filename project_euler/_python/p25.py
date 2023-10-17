# This is done not to litter the global memory space
class project_euler_utils:
    """Utility class for solving project_euler"""

    cache: dict[int, int]

    def __init__(self):
        self.cache = {}

    def fibonacci(self, num: int) -> int:
        """Returns the Nth fibonacci number"""

        value = self.cache.get(num)
        if value != None:
            return value

        value = num if num < 2 else self.fibonacci(num - 2) + self.fibonacci(num - 1)
        self.cache[num] = value
        return value


utils = project_euler_utils()


def thousand_digit_fibonacci():
    """Problem #25"""

    current = 0
    while True:
        if len(str(utils.fibonacci(current))) == 1000:
            return current
        else:
            current += 1


# Function calls below this

print(f"25: Found: {thousand_digit_fibonacci()}")
