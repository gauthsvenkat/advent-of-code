alias c:=create
alias e:=edit
alias r:=run

current_year := "2024"

create day year=current_year:
	cargo generate template --name {{day}} --destination {{year}}

edit day input year=current_year:
	nvim {{year}}/{{day}}/{{input}}.txt

run day part input year=current_year:
	cargo run --manifest-path={{year}}/{{day}}/Cargo.toml -- {{part}} {{year}}/{{day}}/{{input}}.txt
