# Create table
create-table name:
    sea-orm-cli migrate generate -d ./migration {{name}}

# Migrate up
migrate-up:
    sea-orm-cli migrate -d ./migration up

# Migrate down
migrate-down:
    sea-orm-cli migrate -d ./migration down

# Generate entities
entity-generate:
    sea-orm-cli generate entity -o entities/src
