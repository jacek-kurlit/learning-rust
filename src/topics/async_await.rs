#![allow(unused_variables)]
use std::{fs::File, future::Future, pin::Pin};

fn main() {
    println!("hello world");
    let x = foo();
}

async fn foo() -> usize {
    let mut network = read_from_network();
    let mut terminal = read_from_terminal();
    //NOTE: select basacily polls al features and execut the first one that is ready
    //you can use loop here but bear in mind that some futures may be half finished
    //and while other may complete leading to half finished states
    //Other case is select branch may be already completed and we poll it again causing problems
    //in such case future fuse should be used
    loop {
        select! {
        //NOTE: we make it &mut because await normaly takes the ownersjip and we couldnt use this
        //in loop
            stream <- (&mut network).awit => {
                //do stuff on network read
            }
            line <- (&mut terminal).await => {
                //do stuff on terminal read
            }
        }
    }
}

async fn await_for_multiple() {
    let files: Vec<_> = (0..3)
        .map(|i| File::open(format!("file_{}.txt", i)))
        .collect();
    //there is join!
    //NOTE: this will order results which costs performance
    //unordered futures may be usesd
    join_all!(files).await;
}

async fn read_from_network() -> String {
    todo!()
}

async fn read_from_terminal() -> String {
    todo!()
}

//this means:
#[allow(clippy::manual_async_fn)]
fn foo2() -> impl Future<Output = usize> {
    async {
        println!("foo");
        foo().await;
        0
    }
}


async fn server() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let mut accept = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
        while while let Ok(stream) = accept.wait  {
            //NOTE: spawn is important here
            //simple await would make it run on single thrad all the time!
            //sawn is not spawning a new thread but just add future for runtime to pick up by free
            //threads
           let join_handle = tokio::spawn(handle_request(stream));
            //if spawn return results we can get it with handle.
            //dropping it does nothing to thread
        }
    });
}

async fn handle_request(stream: tokio::net::TcpStream) {
    //do stuff
}


struct Request;
struct Response;

trait Service {
    //NOTE: why is this probematic for rust?
    //Because rust would like to know size of response and async makes it harder
    //as Future is not sized and compiler does not know how much memory it needs
    async fn call(&mut self, req: Request) -> Response;
}

#[async_trait]
trait Service2 {
    async fn call(&mut self, req: Request) -> Response;
    //NOTE: This will produce this code
    //Is not optimal as rust must do heap allocations and dynamic dispatch
    async fn call2(&mut self, req: Request) -> Pin<Box<dyn Future<Output = Response>>> {
        todo!()
    }
    
}


/// What is the difference between std Mutex and tokio Mutex?
/// tokio Mutex is async aware and std Mutex is not
/// std Mutex simply blocks so using it between .await may cause dead locks
/// On the other hand tokio Mutex is much more complex and may need additional time to aquire and
/// unlock mutex
