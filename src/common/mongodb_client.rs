use mongodb::Client;


pub struct MongoDBClient{
    pub connection_string: String,
    pub username: String,
    pub passwd: String,
    pub db: String
}

impl MongoDBClient{
    pub fn new(&self, connection_string: &str, username: &str, passwd: &str, db: &str) -> Self{
       MongoDBClient{connection_string: connection_string.to_string(), username: username.to_string(), passwd: passwd.to_string(), db:db.to_string()}
    }
    pub async fn connect(&self)->Result<(), Box<dyn std::error::Error>>{
        //Connect to the mongo instance
        let client = Client::with_uri_str(&self.connection_string).await?;
        let db = client.database(&self.db);
        Ok(())
    }
}