alias c:=create
alias r:=run

current_year := "2024"

create day year=current_year:
	cargo new {{year}}/{{day}}
	touch {{year}}/{{day}}/eg.txt
	touch {{year}}/{{day}}/input.txt

run day input="example" year=current_year:
	cargo run --manifest-path={{year}}/{{day}}/Cargo.toml -- {{year}}/{{day}}/{{input}}.txt
