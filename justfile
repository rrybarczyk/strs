r: 
	cargo build --release
	cat target/release/strs | ./target/release/strs

d: 
	cargo build
	cat target/debug/strs | ./target/debug/strs

d-wc: 
	cargo build
	cat target/debug/strs | ./target/debug/strs | wc -l

update-readme:
	echo '# strs' > README.md
	echo '*strs* is a modern alternative to [*strings*](https://linux.die.net/man/1/strings).' >> README.md
	echo '' >> README.md
	echo '```' >> README.md
	cargo run -- --help >> README.md
	echo '```' >> README.md

