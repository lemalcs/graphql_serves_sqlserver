use crate::schema::GraphDemoSchema;
use actix_web::{HttpResponse, Result, web};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/ws")
                .finish(),
        ))
}

pub async fn graphql_handler(
    schema: web::Data<GraphDemoSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
