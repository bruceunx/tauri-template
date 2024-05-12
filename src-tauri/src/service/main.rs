use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};
use tokio::sync::{mpsc, Mutex};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{DataRequest, DataResponse, HelloReply, HelloRequest};
use tokio_stream::{wrappers::ReceiverStream, Stream};
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
    should_stop: Arc<AtomicBool>,
    should_send: Arc<AtomicBool>,
}

impl New for MyGreeter {
    fn new() -> MyGreeter {
        let should_stop = Arc::new(AtomicBool::new(false));
        let should_send = Arc::new(AtomicBool::new(false));

        let should_send_clone = should_send.clone();

        let (tx, rx) = mpsc::channel(1);
        let rec = Arc::new(Mutex::new(rx));
        let greet = MyGreeter {
            rx: rec,
            state: "".to_string(),
            should_stop,
            should_send,
        };

        tokio::spawn(async move {
            loop {
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let message = format!("{} hello from the thread!", timestamp);
                println!("server message: {}", &message);

                if should_send_clone.load(Ordering::Relaxed) {
                    println!("send to stream ->");
                    tx.send(message).await.unwrap();
                }
                thread::sleep(Duration::from_millis(1000))
            }
        });
        return greet;
    }
}
//
#[tonic::async_trait]
impl Greeter for MyGreeter {
    type StreamDataStream = Pin<Box<dyn Stream<Item = Result<DataResponse, Status>> + Send>>;

    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        self.should_stop.store(true, Ordering::Relaxed);
        self.should_send.store(false, Ordering::Relaxed);

        let mut reveiver = self.rx.lock().await;
        let val = reveiver.recv().await.unwrap();
        let reply = HelloReply {
            message: format!("{} {}!", val, request.get_ref().name),
        };

        Ok(Response::new(reply))
    }

    async fn stream_data(
        &self,
        request: Request<DataRequest>,
    ) -> Result<Response<Self::StreamDataStream>, Status> {
        println!("Got a request: {:?}", request);

        let (tx, rx) = mpsc::channel(30);
        let should_stop = self.should_stop.clone();
        should_stop.store(false, Ordering::Relaxed);

        let should_send = self.should_send.clone();
        should_send.store(true, Ordering::Relaxed);

        let rx_clone = self.rx.clone();

        tokio::spawn(async move {
            let mut receiver = rx_clone.lock().await;
            loop {
                if should_stop.load(Ordering::Relaxed) {
                    break;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let message = receiver.recv().await.unwrap();
                println!("stream - {}", &message);
                let result = tx
                    .send(Ok(DataResponse {
                        data: format!("{} from stream, {}", timestamp, message),
                    }))
                    .await;
                match result {
                    Ok(_) => (),
                    Err(_) => {
                        should_send.store(false, Ordering::Relaxed);
                        break;
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
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
