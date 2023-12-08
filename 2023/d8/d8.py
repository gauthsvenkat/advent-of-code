import sys
from typing import List, Dict, Tuple, Iterable
from itertools import cycle
from math import lcm

lines: List[List[str]] = [
    "".join([c for c in line if c.isalpha() or c.isspace() or c.isdigit()]).split()
    for line in sys.stdin
    if line != "\n"
]

direction_sequence: Iterable = cycle(lines.pop(0).pop())

node_maps: Dict[str, Tuple[str, str]] = {n: (l, r) for n, l, r in lines}  # noqa: E741


def get_num_steps(start_node: str) -> int:
    current_node: str = start_node
    steps: int = 0
    while not current_node.endswith("Z"):
        instruction: str = next(direction_sequence)
        current_node = node_maps[current_node][0 if instruction == "L" else 1]

        steps += 1
    return steps

p1 = get_num_steps("AAA")
print(p1)

p2 = lcm(*(get_num_steps(node) for node in node_maps if node.endswith("A")))
print(p2)
