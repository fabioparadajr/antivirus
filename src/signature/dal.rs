/*
db.createUser({
  user: "av",
  pwd: "av",
  roles: [ { role: "readWrite", db: "antivirus" } ]
})


antivirus> db.malware.find()
[
  {
    _id: ObjectId('679a898b72fe83e009544ca7'),
    name: '6DruzpERXVmNY8S.exe',
    signature: '877d00f6fad980ae8bd9c1712e0f79f40a2d3dae02ddb8f73174ea08723eb818'
  }
]

 */

use bson::de::from_document;
use mongodb::{bson::{doc, Document, bson}, Client, options::{ClientOptions, FindOptions}};
use futures::stream::StreamExt;
use futures::TryStreamExt;
use tokio;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
struct FileDocument {

    name: String,
    signature: String,
}


pub async fn connect_database(hash: &str) -> mongodb::error::Result<()> {
    // Definir a URL de conexão com o MongoDB, incluindo as credenciais de autenticação
    let username = "av"; // Nome de usuário
    let password = "av"; // Senha do usuário
    let db_name = "antivirus"; // O banco de dados que você deseja acessar
    let auth_db = "admin"; // O banco de dados de autenticação (normalmente "admin")
    //let hash = "877d00f6fad980ae8bd9c1712e0f79f40a2d3dae02ddb8f73174ea08723eb818";
    // Criar a string de conexão com autenticação
    let uri = format!(
        "mongodb://{}:{}@localhost:27017/{}?authSource={}",
        username, password, db_name, auth_db
    );

    // Configurar as opções do cliente
    let client_options = ClientOptions::parse(&uri)
        .await
        .expect("Fail to parse connection");

    // Criar o cliente MongoDB
    let client = Client::with_options(client_options).expect("Fail to create mongodb client");

    // Conectar ao banco de dados
    let db = client.database(db_name);

    let filter = doc! { "signature" : hash };
    let collection = db.collection::<Document>("malware");

    let hash_find = collection.find(filter).await;

    let mut hashers = match hash_find {
        Ok(c) => c,
        Err(e) => {

            eprintln!("Cant execute query: {:?}", e);
            return Ok(());
        }
    };
    let results = match hashers.try_collect::<Vec<_>>().await {
        Ok(results) => results,
        Err(e) => {

            eprintln!("Cant collect data {:?}", e);
            vec![]
        }
    };

    let file_docs: Vec<FileDocument> = results.into_iter()
        .filter_map(|doc| {

            from_document(doc).ok()
        })
        .collect();

    let mut name_final_answer = String::new();
    let mut hash_final_answer = String::new();
    for file_doc in file_docs {
        name_final_answer = file_doc.name;

        hash_final_answer = file_doc.signature;
    }

    if hash_final_answer == hash {
        return Ok(());
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Hash mismatch").into());
    }


}
