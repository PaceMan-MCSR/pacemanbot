build:
	cargo build -r

run:
	cargo run

run-release:
	cargo run -r

deploy:
	git pull origin 1.15
	make build
	systemctl restart pacemanbot1.15

deploy-testing:
	git pull origin testing-1.15
	make build
	systemctl restart pacemanbot1.15

test:
	cargo test
