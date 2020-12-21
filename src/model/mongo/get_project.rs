use juniper::{GraphQLObject, GraphQLInputObject};
use uuid::Uuid;
use actix::Message;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, GraphQLObject)]
pub struct MongoProject {
    slides: Vec<Slide>,
    dependency: Vec<Plugin>,
    styles: Vec<Style>,
    relationship: Vec<Effect>
}

impl Default for MongoProject {
    fn default() -> Self {
        MongoProject {
            slides: Vec::with_capacity(0),
            dependency: Vec::with_capacity(0),
            styles: Vec::with_capacity(0),
            relationship: Vec::with_capacity(0)
        }
    }
}

#[derive(Deserialize, Serialize,GraphQLObject)]
struct Plugin {
    path: String,
    args: Args,
}
#[derive(Deserialize, Serialize,GraphQLObject)]
struct Args {
    // Hashmap of (value: type of value)
    targets: Vec<Target>
}

#[derive(Deserialize, Serialize, GraphQLObject)]
struct Effect {
    name: String,
    targets: Vec<Target>
}

#[derive(Deserialize, Serialize, GraphQLObject)]
struct Target {
    element_id: String,
    type_element: String
}

#[derive(Deserialize, Serialize, GraphQLObject)]
struct Style {
    path: String
}

#[derive(Deserialize, Serialize,  GraphQLObject)]
struct Slide {
    id: i32,
    layers: Vec<Layer>
}

#[derive(Deserialize, Serialize,  GraphQLObject)]
struct Layer {
    id: String,
    elements: Vec<Element>
}

#[derive(Deserialize, Serialize, GraphQLObject)]
struct Element {
    id: String,
    class: String,
    content: String,
    type_element: String,
    position: Point,
    url: Option<String>
}


#[derive(Deserialize, Serialize, GraphQLObject)]
struct Point {
    x: i32,
    y: i32
}

#[derive(GraphQLInputObject)]
pub(crate) struct GetProject {
    pub id: Uuid
}

impl Message for GetProject {
    type Result = Result<MongoProject,  Vec<String>>;
}