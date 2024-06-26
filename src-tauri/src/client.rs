pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::greeter_client::GreeterClient;
use hello_world::{DataRequest, HelloRequest};
use tauri::Manager;
use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[allow(dead_code)]
#[tauri::command]
pub async fn greet_client(name: &str) -> Result<String, String> {
    println!("greet_client: {}", &name);
    let mut client = GreeterClient::connect("http://[::1]:50051")
        .await
        .map_err(|e| format!("{:?}", e))?;
    let request = tonic::Request::new(HelloRequest { name: name.into() });
    let response = client
        .say_hello(request)
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(response.into_inner().message)
}

#[allow(dead_code)]
#[tauri::command]
pub async fn stream_client(window: Window, name: String) {
    let mut client = GreeterClient::connect("http://[::1]:50051")
        .await
        .map_err(|e| format!("{:?}", e))
        .unwrap();
    let request = tonic::Request::new(DataRequest {
        message: name.into(),
    });

    let mut stream = client.stream_data(request).await.unwrap().into_inner();
    while let Some(res) = stream.message().await.unwrap() {
        let _ = window.emit("greet", Payload { message: res.data });
        // println!("{:?}", res);
    }
}

// #[derive(Debug, Default)]
// pub struct MyGreeter {}
//
// #[tonic::async_trait]
// impl Greeter for MyGreeter {
//     async fn say_hello(
//         &self,
//         request: Request<HelloRequest>,
//     ) -> Result<Response<HelloReply>, Status> {
//         println!("Got a request: {:?}", request);
//         let reply = HelloReply {
//             message: format!("Hello from grpc {}!", request.get_ref().name),
//         };
//         Ok(Response::new(reply))
//     }
// }

// #[allow(dead_code)]
// #[tauri::command]
// pub async fn start_grpc_server() {
//     let addr = "[::1]:50051".parse().unwrap();
//     let greeter = MyGreeter::default();
//
//     Server::builder()
//         .add_service(GreeterServer::new(greeter))
//         .serve(addr)
//         .await
//         .unwrap();
// }
