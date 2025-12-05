#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "advent-of-code-data>=2.1.0",
#   "typer>=0.12.0",
# ]
# ///

"""
Advent of Code Data helper

A script to interact with Advent of Code puzzles using the aocd library.
Supports getting input data, retrieving examples, and submitting solutions.
"""

import os
from pathlib import Path
from typing import Literal

import typer
from aocd import submit
from aocd.models import Puzzle

app = typer.Typer(
    help="Advent of Code Data Manager - Get inputs, examples, and submit solutions"
)


def _check_session() -> None:
    if "AOC_SESSION" not in os.environ:
        raise ValueError(
            "AOC_SESSION environment variable is not set. "
            "Please set it with your Advent of Code session token."
        )


@app.command(name="get-input")
def get_input(
    day: int | None = typer.Option(
        None, "--day", "-d", help="Day of the puzzle (1-25), defaults to today"
    ),
    year: int | None = typer.Option(
        None, "--year", "-y", help="Year of the puzzle, defaults to current year"
    ),
    output: str | None = typer.Option(
        None, "--output", "-o", help="Output file path to save the input"
    ),
) -> None:
    """Get puzzle input data for the specified day and year."""
    _check_session()

    # Use Puzzle class for consistency with get_example
    puzzle = Puzzle(year=year, day=day)
    data = puzzle.input_data

    if output:
        # Save to file
        output_path = Path(output)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text(data)
        typer.secho(f"✓ Saved input to: {output}", fg=typer.colors.GREEN)
    else:
        # Print to stdout
        typer.echo(data)


@app.command(name="get-example")
def get_example(
    day: int | None = typer.Option(
        None, "--day", "-d", help="Day of the puzzle (1-25), defaults to today"
    ),
    year: int | None = typer.Option(
        None, "--year", "-y", help="Year of the puzzle, defaults to current year"
    ),
    output: str | None = typer.Option(
        None, "--output", "-o", help="Output file path to save the example"
    ),
) -> None:
    """Get example data for the specified day and year."""
    _check_session()

    # Create Puzzle instance (aocd handles None defaults automatically)
    puzzle = Puzzle(year=year, day=day)

    if not puzzle.examples:
        typer.secho(
            "No example data found for this puzzle",
            fg=typer.colors.YELLOW,
            err=True,
        )
        raise typer.Exit(1)

    if output:
        # Save each example to numbered files
        output_path = Path(output)
        output_path.parent.mkdir(parents=True, exist_ok=True)

        for idx, example in enumerate(puzzle.examples, start=1):
            # Generate numbered filename: eg.txt -> eg1.txt, eg2.txt
            numbered_path = output_path.with_stem(f"{output_path.stem}{idx}")
            numbered_path.write_text(example.input_data)
            typer.secho(
                f"✓ Saved example {idx} to: {numbered_path}", fg=typer.colors.GREEN
            )
    else:
        # Print all examples to stdout with separators and answers
        for idx, example in enumerate(puzzle.examples, start=1):
            if idx > 1:
                typer.echo("")  # Blank line between examples
            typer.echo(f"--- Example {idx} ---\n")
            typer.echo(example.input_data)
            typer.echo(f"\nanswer_p1: {example.answer_a or '-'}")
            typer.echo(f"answer_p2: {example.answer_b or '-'}")


@app.command(name="submit")
def submit_solution(
    answer: str = typer.Argument(..., help="The answer to submit"),
    part: Literal["p1", "p2"] = typer.Option(
        ..., "--part", "-p", help="Part of the puzzle ('p1' or 'p2')"
    ),
    day: int | None = typer.Option(
        None, "--day", "-d", help="Day of the puzzle (1-25), defaults to today"
    ),
    year: int | None = typer.Option(
        None, "--year", "-y", help="Year of the puzzle, defaults to current year"
    ),
) -> None:
    """Submit a solution for the specified puzzle."""
    _check_session()

    part_map = {"p1": "a", "p2": "b"}

    # Submit the answer (aocd handles None defaults automatically)
    typer.echo(f"Submitting answer '{answer}' for part {part}...")
    submit(answer, part=part_map[part], day=day, year=year)


if __name__ == "__main__":
    app()
