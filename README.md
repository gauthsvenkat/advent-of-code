# Advent of Code

My Advent of Code solutions in Rust, organized as a Cargo workspace.

## Usage

I use [just](https://github.com/casey/just) for common tasks:

### Create a new day
```bash
just create              # creates today's day (e.g., d01 for Dec 1st)
just create d05 2024     # creates a specific day/year
```

### Edit input files
```bash
just edit                # opens today's eg.txt in nvim
just edit in             # opens today's in.txt
just edit eg d05 2024    # opens specific day's example input
```

### Run solutions
```bash
just run                 # runs today's part 1 with eg.txt
just run in p2           # runs today's part 2 with in.txt
just run eg p1 d05 2024  # runs specific day/part
```
