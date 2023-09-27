// File: user_api.rs

use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::{results::InsertOneResult, bson::oid::ObjectId};
use rocket::{http::Status, serde::json::Json, State};

/// O método `create_user` é responsável por criar um novo usuário no MongoDB.
/// # Arguments
/// * `db` - Uma instância de `MongoRepo`.
/// * `new_user` - Um novo usuário a ser criado.
/// # Returns
/// * `Result<Json<InsertOneResult>, Status>` - Um resultado de inserção de um documento no MongoDB.
/// * `Status::InternalServerError` - Caso ocorra algum erro interno.
/// * `Status::Ok` - Caso o usuário seja criado com sucesso.
#[post("/user",data="<new_user>")]
pub fn create_user(db: &State<MongoRepo>, new_user: Json<User>) -> Result<Json<InsertOneResult>, Status> {
    let user = User {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };

    let result = db.create(user);

   match result {
       Ok(user) => Ok(Json(user)),
       Err(_) => Err(Status::InternalServerError),
   }
}

/// O método `get_user` é responsável por buscar um usuário no MongoDB.
/// # Arguments
/// * `db` - Uma instância de `MongoRepo`.
/// * `id` - O id do usuário a ser buscado.
/// # Returns
/// * `Result<Json<User>, Status>` - Um resultado de busca de um documento no MongoDB.
/// * `Status::InternalServerError` - Caso ocorra algum erro interno.
/// * `Status::Ok` - Caso o usuário seja encontrado.
/// * `Status::BadRequest` - Caso o id seja vazio.
/// * `User` - Caso o usuário seja encontrado.
/// * `Status::NotFound` - Caso o usuário não seja encontrado.
#[get("/user/<id>")]
pub fn get_user(db: &State<MongoRepo>, id: String) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let user = db.get_user(&id);
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
        
}

/// O método `update_user` é responsável por atualizar um usuário no MongoDB.
/// # Arguments
/// * `db` - Uma instância de `MongoRepo`.
/// * `id` - O id do usuário a ser atualizado.
/// * `new_user` - O usuário a ser atualizado.
/// # Returns
/// * `Result<Json<User>, Status>` - Um resultado de atualização de um documento no MongoDB.
/// * `Status::InternalServerError` - Caso ocorra algum erro interno.
/// * `Status::Ok` - Caso o usuário seja atualizado.
/// * `Status::BadRequest` - Caso o id seja vazio.
/// * `User` - Caso o usuário seja atualizado.
#[put("/user/<id>", data = "<new_user>")]
pub fn update_user(db: &State<MongoRepo>, id: String, new_user: Json<User>) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let user = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };

    let result = db.update_user(&id, user);

    match result {
        Ok(update) => {
            if update.id.is_some(){
                let updated_user_info = db.get_user(&id);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                }
            }else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

/// O método `delete_user` é responsável por deletar um usuário no MongoDB.
/// # Arguments
/// * `db` - Uma instância de `MongoRepo`.
/// * `id` - O id do usuário a ser deletado.
/// # Returns
/// * `Result<Json<&str>, Status>` - Um resultado de deleção de um documento no MongoDB.
/// * `Status::InternalServerError` - Caso ocorra algum erro interno.
/// * `Status::Ok` - Caso o usuário seja deletado com sucesso.
/// * `Status::BadRequest` - Caso o id seja vazio.
/// * `Status::NotFound` - Caso o usuário não seja encontrado.
/// * `&str` - Caso o usuário seja deletado com sucesso.
#[delete("/user/<id>")]
pub fn delete_user(db: &State<MongoRepo>, id: String) -> Result<Json<&str>, Status> {
    if id.is_empty(){
        return Err(Status::BadRequest);
    }
    let result = db.delete_user(&id);
    match result {
        Ok(user) => {
            if user.id.is_some(){
                return Ok(Json("User deleted successfully."));
            }else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

/// O método `get_all_users` é responsável por buscar todos os usuários no MongoDB.
/// # Arguments
/// * `db` - Uma instância de `MongoRepo`.
/// # Returns
/// * `Result<Json<Vec<User>>, Status>` - Um resultado de busca de todos os documentos no MongoDB.
/// * `Status::InternalServerError` - Caso ocorra algum erro interno.
/// * `Status::Ok` - Caso os usuários sejam encontrados.
/// * `Vec<User>` - Caso os usuários sejam encontrados.
/// * `Status::NotFound` - Caso os usuários não sejam encontrados.
/// * `Status::BadRequest` - Caso o id seja vazio.
#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}