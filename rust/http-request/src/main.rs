// use reqwest::Client;
// use reqwest::StatusCode;

// fn main() {
//     // https://api-member-stag.evosesports.com/v1/store/products/41/items/53/destination-inquiry
//     println!("Hello, world!");
//     let client = reqwest::Client::new();
//     let resp = client.post("https://api-member-stag.evosesports.com/v1/store/products/41/items/53/destination-inquiry")
//         .body(r#"{ "destination": "9854954"}"#)
//         .send()?;
    


//         //println!("{:?}", res);
// }

// use hyper::{Client, Uri};

// #[tokio::main]
// async fn main() {
//     let client = Client::new();

//     let url: Uri = "https://api-member-stag.evosesports.com/v1/store/products/41/items/53/destination-inquiry"
//         .parse()
//         .unwrap();

//     match client.get(url).await {
//         Ok(res) => println!("Response: {}", res.status()),
//         Err(err) => println!("Error: {}", err),
//     }
// }


// use http::{Request, Response, StatusCode};

// fn main() {
//     let request = Request::builder()
//     .method("POST")
//     .uri("https://api-member-stag.evosesports.com/v1/store/products/41/items/53/destination-inquiry")
//     .body(r#"{"destination": "9854954"}"#)
//     .unwrap();

//     let response = request.

//     println!("{:#?}", request.body())
// }


// use reqwest;

// fn main() {
//     let client = reqwest::Client::new();
//     let res = client.post("http://httpbin.org/post")
//         .body("the exact body that is sent")
//         .send();

// }

use reqwest::Client;
use reqwest::StatusCode;

fn main(){
    let client = Client::new();

    let resp = client.post("https://api-member-stag.evosesports.com/v1/store/products/41/items/53/destination-inquiry")
        .body(r#"{"destination": "2872317004369363999"}"#)
        .send()
        .unwrap();
    
    match resp.status() {
        StatusCode::OK => println!("success!"),
        StatusCode::PAYLOAD_TOO_LARGE => {
            println!("Request payload is too large!");
        }
        s => println!("Received response status: {:?}", s),
    };
}

