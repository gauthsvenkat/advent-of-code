import sys

agg = 0

for line in sys.stdin:
    for r in (
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "4"),
        ("five", "5e"),
        ("six", "6"),
        ("seven", "7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ):
        line = line.replace(*r)

    if not (nums := "".join(c for c in line if c.isdigit())):
        continue

    agg += int(nums[0] + nums[-1])

print(agg)
