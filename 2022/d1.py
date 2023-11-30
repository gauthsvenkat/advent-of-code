import sys

top_1 = 0
top_2 = 0
top_3 = 0
elf_agg = 0

for line in sys.stdin:
    if line == "\n":
        if elf_agg > top_1:
            top_3 = top_2
            top_2 = top_1
            top_1 = elf_agg
        elif elf_agg > top_2:
            top_3 = top_2
            top_2 = elf_agg
        elif elf_agg > top_3:
            top_3 = elf_agg
        elf_agg = 0
    else:
        elf_agg += int(line)

print(f"{top_1}")
print(f"{top_1 + top_2 + top_3}")
