extern crate restson;
#[macro_use]
extern crate serde_derive;

use restson::{Error, RestClient, RestPath};

// Data structure that matches with REST API JSON
#[derive(Serialize, Deserialize, Debug)]
struct HttpBinAnything {
    method: String,
    url: String,
}

// Path of the REST endpoint: e.g. http://<baseurl>/anything
impl RestPath<()> for HttpBinAnything {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("anything"))
    }
}

#[tokio::main]
async fn main() {
    // Create new client with API base URL
    let mut client = RestClient::new("http://httpbin.org").unwrap();

    // GET http://httpbin.org/anything and deserialize the result automatically
    let data: HttpBinAnything = client.get(()).await.unwrap();
    println!("{:?}", data);
}
