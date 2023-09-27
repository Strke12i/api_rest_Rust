use std::env;
extern crate dotenv;
use dotenv::dotenv;

extern crate bcrypt;
use bcrypt::{hash, DEFAULT_COST};

use crate::models::user_model::User;
use mongodb::bson::extjson::de::Error;
use mongodb::{
    results::InsertOneResult,
    sync::{Client, Collection},
    bson::{doc, oid::ObjectId},
};

/// MongoRepo é uma struct que representa o repositório de dados do MongoDB.
/// # Atributos
/// * `col` - Uma coleção de documentos do MongoDB.
pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    /// O método `init` é responsável por inicializar o repositório de dados do MongoDB.
    /// #Returns Uma instância de `MongoRepo`. 
    pub fn init() -> Self {
        dotenv().ok();
        let mongo_url = match env::var("MONGO_URL") {
            Ok(val) => val.to_string(),
            Err(_) => panic!("MONGO_URL must be set in .env file."),
        };

        let client = Client::with_uri_str(mongo_url.as_str()).expect("Failed to initialize client.");
        let db = client.database("rust_rocket");
        let col = db.collection("users");
        MongoRepo { col }
    }

    /// O método `create` é responsável por criar um novo usuário no MongoDB.
    /// # Arguments
    /// * `new_user` - Um novo usuário a ser criado.
    /// # Returns
    /// * `Result<InsertOneResult, Error>` - Um resultado de inserção de um documento no MongoDB.
    /// * `Error` - Caso ocorra algum erro.
    /// * `InsertOneResult` - Caso o usuário seja criado.
    /// * `Status::InternalServerError` - Caso ocorra algum erro interno.
    /// * `Status::Ok` - Caso o usuário seja criado com sucesso.
    /// * `Status::BadRequest` - Caso o id seja vazio.
    /// * `Status::NotFound` - Caso o usuário não seja encontrado.
    pub fn create(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let hashed_password = hash(new_user.password, DEFAULT_COST).unwrap();
        let new_doc = User {
            id: None,
            name: new_user.name,
            email: new_user.email,
            password: hashed_password,
        };

        let user = self.col.insert_one(new_doc, None).ok().expect("Failed to insert User.");
        Ok(user)
    }

    /// O método `get_user` é responsável por buscar um usuário no MongoDB.
    /// # Arguments
    /// * `id` - O id do usuário a ser buscado.
    /// # Returns
    /// * `Result<User, Error>` - Um resultado de busca de um documento no MongoDB.
    /// * `Error` - Caso ocorra algum erro.
    /// * `User` - Caso o usuário seja encontrado.
    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id.as_str()).unwrap();
        let filter = doc! {"_id": obj_id};
        let user = self.col.find_one(filter, None).ok().expect("Failed to get User.");

        Ok(user.unwrap())
    }

    /// O método `update_user` é responsável por atualizar um usuário no MongoDB.
    /// # Arguments
    /// * `id` - O id do usuário a ser atualizado.
    /// * `user` - O usuário a ser atualizado.
    /// # Returns
    /// * `Result<User, Error>` - Um resultado de atualização de um documento no MongoDB.
    /// * `Error` - Caso ocorra algum erro.
    /// * `User` - Caso o usuário seja atualizado.
    pub fn update_user(&self, id: &String, user: User) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id.as_str()).unwrap();
        let filter = doc! {"_id": obj_id};
        let update = doc! {"$set": {"name": user.name, "email": user.email, "password": user.password}};
        let user_updated = self.col.find_one_and_update(filter, update, None).ok().expect("Failed to update User.");

        Ok(user_updated.unwrap())
    }

    /// O método `delete_user` é responsável por deletar um usuário no MongoDB.
    /// # Arguments
    /// * `id` - O id do usuário a ser deletado.
    /// # Returns
    /// * `Result<User, Error>` - Um resultado de deleção de um documento no MongoDB.
    /// * `Error` - Caso ocorra algum erro.
    /// * `User` - Caso o usuário seja deletado.
    /// * `Status::NotFound` - Caso o usuário não seja encontrado.
    /// * `Status::InternalServerError` - Caso ocorra algum erro interno.
    /// * `Status::BadRequest` - Caso o id seja vazio.
    /// * `Status::Ok` - Caso o usuário seja deletado com sucesso.
    pub fn delete_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id.as_str()).unwrap();
        let filter = doc! {"_id": obj_id};

        let user_deleted = self.col.find_one_and_delete(filter, None).ok().expect("Failed to delete User.");
        Ok(user_deleted.unwrap())
    }


    /// O método `get_all_users` é responsável por buscar todos os usuários no MongoDB.
    /// # Returns
    /// * `Result<Vec<User>, Error>` - Um resultado de busca de todos os documentos no MongoDB.
    /// * `Error` - Caso ocorra algum erro.
    /// * `Vec<User>` - Caso os usuários sejam encontrados.
    /// * `Status::InternalServerError` - Caso ocorra algum erro interno.
    /// * `Status::Ok` - Caso os usuários sejam encontrados com sucesso.
    /// * `Status::NotFound` - Caso os usuários não sejam encontrados.
    /// * `Status::BadRequest` - Caso o id seja vazio.
    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self.col.find(None, None).ok().expect("Failed to get all Users.");
        let users = cursors.into_iter().map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
}  
    