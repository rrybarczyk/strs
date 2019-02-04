r: 
	cargo build --release
	cat target/release/strs | ./target/release/strs

d: 
	cargo build
	cat target/debug/strs | ./target/debug/strs

d-wc: 
	cargo build
	cat target/debug/strs | ./target/debug/strs | wc -l
