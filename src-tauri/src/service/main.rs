use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

trait New {
    fn new() -> Self;
}

#[allow(dead_code)]
pub struct MyGreeter {
    state: String,
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl New for MyGreeter {
    fn new() -> MyGreeter {
        let (tx, rx) = mpsc::channel();
        let rec = Arc::new(Mutex::new(rx));
        let greet = MyGreeter {
            rx: rec,
            state: "".to_string(),
        };
        thread::spawn(move || loop {
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let message = format!("[{}] hello from the thread!", timestamp);
            println!("loop in thread");
            println!("message: {}", &message);
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(1000))
        });
        return greet;
    }
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);
        let reveiver = self.rx.lock().unwrap();
        let val = reveiver.recv().unwrap();
        let reply = HelloReply {
            message: format!("{} {}!", val, request.get_ref().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::new();
    println!("grpc server starting ..");

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
