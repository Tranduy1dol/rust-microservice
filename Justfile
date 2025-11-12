# Create table
create-table name:
    sea-orm-cli migrate generate -d ./crates/migration {{ name }}

# Migrate up
migrate-up:
    sea-orm-cli migrate -d ./crates/migration up

# Migrate down
migrate-down:
    sea-orm-cli migrate -d ./crates/migration down

# Migrate refresh
migrate-refresh:
    sea-orm-cli migrate -d ./crates/migration refresh

# Generate entities
entity-generate:
    sea-orm-cli generate entity -o ./crates/entities/src

# Format codes
fmt:
    taplo fmt -c ./taplo/taplo.toml && cargo fmt --all && cargo clippy --workspace --all-features -- -D warnings
