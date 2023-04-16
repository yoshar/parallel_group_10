use std::{
    fs, 
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    thread,
    time::Duration,
    sync::Arc,
    sync::Mutex,
    sync::mpsc,

};





fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool =   ThreadPool::new(8);
    
    
    for stream in listener.incoming() {     

        pool.execute(move || {
            let stream = stream.unwrap();
            handle_connection(stream);
        });
    }
    
}



fn handle_connection(mut stream: TcpStream) {
    let buffered_reader = BufReader::new(&mut stream);
    let request = buffered_reader.lines().next().unwrap().unwrap();

    // let request: Vec <_> = buffered_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
      
    if request == "GET / HTTP/1.1" {
        let status = "HTTP/1.1 200 OK";
        let content = fs::read_to_string("index.html").unwrap();
        let length = content.len();

        let response =
            format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");

        

        stream.write_all(response.as_bytes()).unwrap();
    } else  if request == "GET /sleep HTTP/1.1" {
        thread::sleep(Duration::from_secs(4));

        let status = "HTTP/1.1 200 OK";
        let content = fs::read_to_string("index.html").unwrap();
        let length = content.len();

        let response =
            format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(response.as_bytes()).unwrap();

    } else {
        let status = "HTTP/1.1 404 NOT FOUND";
        let content = fs::read_to_string("404.html").unwrap();
        let length = content.len();

        let response =
            format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");
        
        stream.write_all(response.as_bytes()).unwrap();
    }

    println!("Request: {:#?}", request);
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);

            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

enum Message {
    NewJob(Job),
    Terminate,
}


impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // recieve the message from the sender
                let message = reciever.lock().unwrap().recv().unwrap();

                // if new job do the job
                // if terminate, stop waiting for jobs
                match message {
                    Message::NewJob(job) => {
                        job.call_box();

                        println!("Thread {} received job", id);
                    },
                    Message::Terminate => {
                        break;
                    },
                }
            }
        });

        Worker { 
            id, 
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    // close all threads
    fn drop(&mut self) {
        for _ in &mut self.workers {
            
            // send the termintion message to all threads
            self.sender.send(Message::Terminate).unwrap();
        }
        
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        println!("All threads are done");
    }
}