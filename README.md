# Advent of Code

My Advent of Code solutions in Rust, organized as a Cargo workspace.

## Setup

This project uses [advent-of-code-data](https://github.com/wimglenn/advent-of-code-data) (aocd) to fetch puzzle inputs, examples, and submit solutions automatically.

### Required Environment Variable

Make sure the Advent of Code session token is set (See [advent-of-code-data's quickstart](https://github.com/wimglenn/advent-of-code-data?tab=readme-ov-file#quickstart) on how to obtain the session token):

```bash
export AOC_SESSION="your_session_token_here"
```

## Usage

I use [just](https://github.com/casey/just) for common tasks:

### Create a new day
```bash
just create              # creates today's day (e.g., d01 for Dec 1st)
just create d05 2024     # creates a specific day/year
```

### Edit input files
```bash
just edit eg             # opens today's eg.txt in nvim
just edit in             # opens today's in.txt
just edit eg d05 2024    # opens specific day's example input
```

### Get input/example files
```bash
just get-input           # downloads today's input to in.txt
just get-input d05 2024  # downloads specific day's input

just get-example         # downloads today's example to eg.txt
just get-example d05 2024 # downloads specific day's example
```

### Run solutions
```bash
just run                 # runs today's part 1 with eg.txt
just run in p2           # runs today's part 2 with in.txt
just run eg p1 d05 2024  # runs specific day/part
```

### Show input/example (without saving)
```bash
just show-input          # displays today's input
just show-input d05 2024 # displays specific day's input

just show-example        # displays today's example
just show-example d05 2024 # displays specific day's example
```

### Submit solution
```bash
just submit              # submits today's part 1 solution
just submit p2           # submits today's part 2 solution
just submit p1 d05 2024  # submits specific day/part
```

### Test solutions
```bash
just test                # runs tests for today
just test d05 2024       # runs tests for specific day
```

### Clean data files
```bash
just clean-data          # removes today's *.txt files
just clean-data d05 2024 # removes specific day's *.txt files

just clean-all-data      # removes all *.txt files in the project
```
