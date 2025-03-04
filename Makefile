build:
	cargo build -r

run:
	cargo run

run-release:
	cargo run -r

deploy:
	git pull origin 1.7
	make build
	systemctl restart pacemanbot1.7

deploy-testing:
	git pull origin testing-1.7
	make build
	systemctl restart pacemanbot1.7

test:
	cargo test
