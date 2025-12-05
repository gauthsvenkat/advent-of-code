alias c:=create
alias e:=edit

alias gi:=get-input
alias ge:=get-input

alias r:=run

alias si:=show-input
alias se:=show-example

alias su:= submit

current_year := `date +%Y`
current_day := `date +%d`

_ensure-dir day year:
	mkdir -p {{year}}/{{day}}

create day=("d"+current_day) year=current_year: (_ensure-dir day year)
	cargo generate template \
		--name y{{year}}-{{day}} \
		--destination {{year}}/{{day}} \
		--init
	just get-input {{day}} {{year}}
	just get-example {{day}} {{year}}

# input = {eg, in}
edit input day=("d"+current_day) year=current_year:
	@nvim {{year}}/{{day}}/{{input}}.txt

get-input day=("d"+current_day) year=current_year: (_ensure-dir day year)
	uv run --script aoc.py -- get-input \
		--day {{trim_start_match(day, "d")}} \
		--year {{year}} \
		--output {{year}}/{{day}}/in.txt

get-example day=("d"+current_day) year=current_year: (_ensure-dir day year)
	uv run --script aoc.py -- get-example \
		--day {{trim_start_match(day, "d")}} \
		--year {{year}} \
		--output {{year}}/{{day}}/eg.txt

run input="eg" part="p1" day=("d"+current_day) year=current_year quiet="false":
	cargo run \
		{{ if quiet == "true" { "--quiet" } else { "" } }} \
		--manifest-path={{year}}/{{day}}/Cargo.toml \
		-- {{part}} {{year}}/{{day}}/{{input}}.txt

show-input day=("d"+current_day) year=current_year:
	uv run --script aoc.py -- get-input \
		--day {{trim_start_match(day, "d")}} \
		--year {{year}}

show-example day=("d"+current_day) year=current_year:
	uv run --script aoc.py -- get-example \
		--day {{trim_start_match(day, "d")}} \
		--year {{year}}

submit part="p1" day=("d"+current_day) year=current_year:
	uv run --script aoc.py -- submit \
		`just run in {{part}} {{day}} {{year}} true` \
		--part {{part}} \
		--day {{trim_start_match(day, "d")}} \
		--year {{year}}

clean-data day=("d"+current_day) year=current_year:
	rm -f {{year}}/{{day}}/*.txt

clean-all-data:
	fd -t f -e txt --no-ignore -x rm
