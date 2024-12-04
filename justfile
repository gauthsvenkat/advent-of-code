alias c:=create
alias r:=run

current_year := "2024"

create day year=current_year:
	cargo generate template --name {{day}} --destination {{year}}

run day part input year=current_year:
	cargo run --manifest-path={{year}}/{{day}}/Cargo.toml -- {{part}} {{year}}/{{day}}/{{input}}.txt
