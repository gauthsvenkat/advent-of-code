alias c:=create
alias e:=edit
alias g:=get-input
alias r:=run
alias s:=show-example

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

get-input day=("d"+current_day) year=current_year: (_ensure-dir day year)
	nix-shell -p aocd --run "aocd {{trim_start_match(day, "d")}} {{year}}" \
		> {{year}}/{{day}}/in.txt \
		|| echo "Warning: Failed to download input data!"


clean-data day=("d"+current_day) year=current_year:
	rm -f {{year}}/{{day}}/*.txt

clean-all-data:
	fd -t f -e txt --no-ignore -x rm

# input = {eg, in}
edit input day=("d"+current_day) year=current_year:
	@nvim {{year}}/{{day}}/{{input}}.txt

run input="eg" part="p1" day=("d"+current_day) year=current_year:
	cargo run \
		--manifest-path={{year}}/{{day}}/Cargo.toml \
		-- {{part}} {{year}}/{{day}}/{{input}}.txt

# `aocd` shows example in a different format than input. So, can't automatically download :(

show-example day=("d"+current_day) year=current_year:
	nix-shell -p aocd --run "aocd {{year}} {{trim_start_match(day, "d")}} -e"
