import sys


def parse_lines(lines: list[str]) -> list[tuple[str, list[int]]]:
    return list(
        map(
            lambda s: (s[0], [*map(int, s[1].split(","))]),
            map(lambda line: line.split(" "), lines),
        )
    )


input = parse_lines([line.strip() for line in sys.stdin if line != "\n"])

print(input)
