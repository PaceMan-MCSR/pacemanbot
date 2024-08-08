build:
	cargo build -r

run:
	cargo run

run-release:
	cargo run -r

deploy:
	git pull origin main
	make build
	systemctl restart pacemanbot

deploy-testing:
	git pull origin testing
	make build
	systemctl restart pacemanbot

test:
	cargo test
