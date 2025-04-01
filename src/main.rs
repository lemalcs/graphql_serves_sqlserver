mod db_connection;
mod schema;
mod startup;
mod types;

use crate::schema::create_schema;
use crate::startup::{graphiql, graphql_handler};
use actix_web::{App, HttpServer, guard, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // GraphQL schema for the GraphDemo database
    let schema = create_schema();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(graphql_handler))
            .service(web::resource("/").guard(guard::Get()).to(graphiql))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
