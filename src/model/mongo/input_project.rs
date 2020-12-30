use uuid::Uuid;
use actix::{Message};
use serde::{Deserialize, Serialize};
use juniper::GraphQLInputObject;
use mongodb::results::{UpdateResult, InsertOneResult};
use mongodb::error::{Error};


#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct InputMongoProject {
    slides: Vec<InputSlide>,
    dependency: Vec<InputPlugin>,
    styles: Vec<InputStyle>,
    relationship: Vec<InputEffect>
}

impl Default for InputMongoProject {
     fn default() -> Self {
        InputMongoProject {
            slides: Vec::with_capacity(0),
            dependency: Vec::with_capacity(0),
            styles: Vec::with_capacity(0),
            relationship: Vec::with_capacity(0)
        }
    }
}

#[derive(Deserialize, Serialize,GraphQLInputObject)]
struct InputPlugin {
    path: String,
    args: InputArgs,
}
#[derive(Deserialize, Serialize,GraphQLInputObject)]
struct InputArgs {
    // Hashmap of (value: type of value)
    targets: Vec<InputTarget>
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
struct InputEffect {
    name: String,
    targets: Vec<InputTarget>
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
struct InputTarget {
    element_id: String,
    type_element: String
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
struct InputStyle {
    path: String
}

#[derive(Deserialize, Serialize,  GraphQLInputObject)]
struct InputSlide {
    id: i32,
    class: String,
    layers: Vec<InputLayer>
}

#[derive(Deserialize, Serialize,  GraphQLInputObject)]
struct InputLayer {
    id: String,
    class: String,
    elements: Vec<InputElement>
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
struct InputElement {
    id: String,
    class: String,
    content: Option<String>,
    type_element: String,
    position: InputPoint,
    url: Option<String>
}


#[derive(Deserialize, Serialize, GraphQLInputObject)]
struct InputPoint {
    x: i32,
    y: i32
}


pub(crate) struct NewProject (pub Uuid);

impl Message for NewProject {
    type Result = Result<InsertOneResult, Error>;
}

#[derive(GraphQLInputObject)]
pub(crate) struct SaveProject{
    pub id: Uuid,
    pub inner: InputMongoProject
}

impl Message for SaveProject {
    type Result = Result<UpdateResult, Error>;
}

