use async_graphql::{EmptySubscription, Schema};

use crate::types::{Mutation, Query};

pub type GraphDemoSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema() -> GraphDemoSchema {
    Schema::build(Query, Mutation, EmptySubscription).finish()
}
