build:
	cargo build -r

run:
	cargo run

run-release:
	cargo run -r

deploy:
	git pull origin AA
	make build
	systemctl restart pacemanbotaa

test:
	cargo test
