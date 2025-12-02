alias c:=create
alias e:=edit
alias r:=run

current_year := `date +%Y`
current_day := `date +%d`

create day=("d"+current_day) year=current_year:
	mkdir -p {{year}}/{{day}}
	cargo generate template --name y{{year}}-{{day}} --destination {{year}}/{{day}} --init

edit input="eg" day=("d"+current_day) year=current_year:
	nvim {{year}}/{{day}}/{{input}}.txt

run input="eg" part="p1" day=("d"+current_day) year=current_year:
	cargo run --manifest-path={{year}}/{{day}}/Cargo.toml -- {{part}} {{year}}/{{day}}/{{input}}.txt
