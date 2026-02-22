database_path := x'~/.local/share/scribble/scribble.db'
database_url := "sqlite://" + database_path

install-sqlx: 
	cargo install sqlx-cli

add-migration name:
	sqlx migrate add -r {{name}}

migrate:
	sqlx migrate run --database-url {{database_url}}

revert:
	sqlx migrate revert --database-url {{database_url}}

sqlx-prepare:
	cargo sqlx prepare --bin scribble

run *args:
	cargo run -- {{args}}

build:
	cargo build

release:
	cargo build --release
	cp target/release/scribble ~/.local/bin

alias r := run
alias m := migrate
alias am := add-migration
