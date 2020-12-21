use uuid::Uuid;
use actix::{Message};
use serde::{Deserialize, Serialize};
use juniper::GraphQLInputObject;
use mongodb::results::{UpdateResult, InsertOneResult};
use mongodb::error::{Error};


#[derive(Deserialize, Serialize, GraphQLInputObject)]
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

#[derive(Deserialize, Serialize,GraphQLInputObject)]
struct Plugin {
    path: String,
    args: Args,
}
#[derive(Deserialize, Serialize,GraphQLInputObject)]
struct Args {
    // Hashmap of (value: type of value)
    targets: Vec<Target>
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
struct Effect {
    name: String,
    targets: Vec<Target>
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
struct Target {
    element_id: String,
    type_element: String
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
struct Style {
    path: String
}

#[derive(Deserialize, Serialize,  GraphQLInputObject)]
struct Slide {
    id: i32,
    class: String,
    layers: Vec<Layer>
}

#[derive(Deserialize, Serialize,  GraphQLInputObject)]
struct Layer {
    id: String,
    class: String,
    elements: Vec<Element>
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
struct Element {
    id: String,
    class: String,
    content: Option<String>,
    type_element: String,
    position: Point,
    url: Option<String>
}


#[derive(Deserialize, Serialize, GraphQLInputObject)]
struct Point {
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
    pub inner: MongoProject
}

impl Message for SaveProject {
    type Result = Result<UpdateResult, Error>;
}

