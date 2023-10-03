use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use tokio;

// Define your database schema using Diesel's table macros.
table! {
    todos (id) {
        id -> Serial4,
        title -> Varchar,
        completed -> Bool,
    }
}

// Create a struct representing the "todos" table.
#[derive(Queryable, Insertable)]
#[table_name = "todos"]
struct Todo {
    title: String,
    completed: bool,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

async fn create_todo(conn: &PgConnection, title: &str, completed: bool) -> QueryResult<usize> {
    use todos::dsl::*;

    let new_todo = Todo {
        title: title.to_string(),
        completed,
    };

    diesel::insert_into(todos)
        .values(&new_todo)
        .execute(conn)
}

#[tokio::main]
async fn main() {
    let conn = establish_connection();

    // Create a new todo asynchronously
    match create_todo(&conn, "Buy groceries", false).await {
        Ok(_) => println!("Todo created successfully"),
        Err(e) => eprintln!("Error creating todo: {:?}", e),
    }
}