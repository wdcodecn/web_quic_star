# rs web quick start


# Prerequisite
sudo apt install build-essential

sudo apt-get install libpq-dev

cargo install diesel_cli --no-default-features --features postgres

cargo install diesel_cli_ext

# Usage

modify DATABASE_URL in [.env](..%2F.env)

diesel setup

create table in [schema.rs](schema.rs)

diesel migration generate --diff-schema create_table

diesel_ext -m -r

diesel migration run

add             
#[diesel(table_name = crate::schema::permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]        
to model ,

derive     WebApiGen ,

cargo run --package web_quic_star --bin example_app