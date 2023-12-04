import sys

input = [list(line.strip()) for line in sys.stdin]
input = [line for line in input if line]
num_row = len(input)
num_col = len(input[0])

# Make an empty mask (we can use this to make a mask of the spots
# that are valid
symbol_mask = [["."] * len(line) for line in input]

f = lambda c: "M" if c.isdigit() else "."


def propagate_mask(mask, input, row, col):
    mask[row][col] = f(input[row][col])

    # Check right
    for col_offset, char in enumerate(input[row][col:]):
        if char == ".":
            break
        mask[row][col + col_offset] = f(input[row][col + col_offset])

    # Check left
    for col_offset, char in enumerate(input[row][:col+1][::-1], start=1):
        if char == ".":
            break
        mask[row][col - col_offset] = f(input[row][col - col_offset])

    return mask


# from top to bottom
for row, line in enumerate(input):
    # from left to right
    for col, char in enumerate(line):
        # if characters is not a digit or the period character
        # then it is a special character
        # and we make a mask over its area of influence
        if not (char.isdigit() or (char == ".")):
            if row != 0:  # up
                symbol_mask = propagate_mask(symbol_mask, input, row - 1, col)
            if row != num_row - 1:  # down
                symbol_mask = propagate_mask(symbol_mask, input, row + 1, col)
            if col != 0:  # left
                symbol_mask = propagate_mask(symbol_mask, input, row, col - 1)
            if col != num_col - 1:  # right
                symbol_mask = propagate_mask(symbol_mask, input, row, col + 1)
            if row != 0 and col != 0:  # up left
                symbol_mask = propagate_mask(symbol_mask, input, row - 1, col - 1)
            if row != 0 and col != num_col - 1:  # up right
                symbol_mask = propagate_mask(symbol_mask, input, row - 1, col + 1)
            if row != num_row - 1 and col != 0:  # down left
                symbol_mask = propagate_mask(symbol_mask, input, row + 1, col - 1)
            if row != num_row - 1 and col != num_col - 1:  # down right
                symbol_mask = propagate_mask(symbol_mask, input, row + 1, col + 1)

input = [
    [char if symbol_mask[row][col] == "M" else "." for col, char in enumerate(line)]
    for row, line in enumerate(input)
]

for line in input:
    print(line)

numbers = [
    int(number) for line in input for number in "".join(line).split(".") if number
]

print(numbers)

print(sum(numbers))
