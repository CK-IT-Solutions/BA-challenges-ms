alias r := run
alias c := check
alias t := test
alias p := psql
alias m := migrate
alias e := entity

_default:
    @just --list

run *args:
    cargo run --locked {{args}}

check *args:
    cargo clippy --locked {{args}}

test *args:
    cargo test --locked {{args}}

psql *args:
    psql "$DATABASE_URL" {{args}}

migrate *args:
    sea migrate {{args}}

entity:
    sea generate entity -l -o entity/src --with-copy-enums
