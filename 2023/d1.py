import sys

agg = 0
for line in sys.stdin:
    line = line.strip().replace("one", "1").replace("two", "2").replace("three", "3").replace("four", "4").replace("five", "5").replace("six", "6").replace("seven", "7").replace("eight", "8").replace("nine", "9")

    nums = ''.join(c for c in line if c.isdigit())
    
    if nums:
        num = nums[0] + nums[-1]
        agg += int(num)

        print(f"{num=} {agg=}")

print(agg)
