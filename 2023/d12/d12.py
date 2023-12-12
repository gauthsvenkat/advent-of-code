import sys


def parse_lines(lines: list[str]) -> list[tuple[str, list[int]]]:
    return list(
        map(
            lambda s: (s[0], [*map(int, s[1].split(","))]),
            map(lambda line: line.split(" "), lines),
        )
    )



input_p1 = parse_lines([line.strip() for line in sys.stdin if line != "\n"])
input_p2 = [("?".join([springs_str]*5), sizes*5) for springs_str, sizes in input_p1]



def get_contiguous_groups(springs_str: str) -> list[str]:
    return [sub_str for sub_str in springs_str.split(".") if sub_str != ""]


def find_sizes_of_contiguous_groups(springs_str: str) -> list[int]:
    return [len(sub_str) for sub_str in get_contiguous_groups(springs_str)]


# Note - Replacing "?" with "#" does not change the size
# Replace "?" wiht "." can change size and the length of size
def recursor(springs_str: str, sizes: list[int]) -> int:
    if "?" not in springs_str:
        return find_sizes_of_contiguous_groups(springs_str) == sizes

    # for sub_str, size in zip(get_contiguous_groups(springs_str), sizes):
    #     if "?" in sub_str:
    #         break
    #     if len(sub_str) != size:
    #         return 0

    return recursor(springs_str.replace("?", "#", 1), sizes) + recursor(
        springs_str.replace("?", ".", 1), sizes
    )


# recursor(*input[0])

# for springs_str, sizes in input_p2:
#     print(springs_str, sizes)

p1 = sum(recursor(springs_str, sizes) for springs_str, sizes in input_p1)
print(p1)
