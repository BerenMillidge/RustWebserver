
// A simple multithreaded webserver implementation in rust, implemented from scratch from the raw TCP/HTML.
// Contains a simple threadpool implementation from scratch.

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::sync::mpsc; 
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;


fn my_handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let simplestOk = "HTTP/1.1 200 OK\r\n\r\n";

    let filename = "hello.html";
    println!("about to open file!");
    let mut f = fs::File::open(filename).expect("File not found!");
    println!("opened file!");
    let mut contents=  String::new();
    let fileContents = f.read_to_string(&mut contents).expect("File read failed!");
    println!("{}", fileContents.clone());
    let fileResponse = format!("HTTP/1.1 200 OK\r\n\r\n {}", fileContents);
    println!("about to printn file response!");
    println!("{}",fileResponse.clone());
    stream.write(fileResponse.as_bytes()).unwrap(); 
    stream.flush().unwrap(); 
}

fn handle_connection_unrefactored(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
	    let contents = fs::read_to_string("hello.html").unwrap();
	    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
	    stream.write(response.as_bytes()).unwrap();
	    stream.flush().unwrap();
	} else {
		let status_line = "HTTP/1.1 404 NOT FOUND \r\n\r\n";
		let contents = fs::read_to_string("404.html").unwrap();
		let response = format!("{}{}", status_line, contents);
		stream.write(response.as_bytes()).unwrap();
		stream.flush().unwrap();
	}

}

fn handle_connection(mut stream: TcpStream) {

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
	let port = "127.0.0.1:7878";
	let listener = TcpListener::bind(port).unwrap(); 
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		println!("Connection establisehd!");
		handle_connection(stream);
	}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

pub struct ThreadPool {
	threads: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}

impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);
		let mut threads = Vec::with_capacity(size);

		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));

		for i in 0..size {
			threads.push(Worker::new(i, Arc::clone(&receiver)));
		}
		ThreadPool {
			threads
		}
	}
	pub fn execute<F>(&self, f:F)
		where F: FnOnce() + Send + 'static {
			let job = Box::new(f);
			self.sender.send(Message::NewJob(job)).unwrap();

		}

}

enum Message {
	NewJob(Job),
	Terminate,
}


impl Drop for ThreadPool {
	fn drop(&mut self){ 
		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);

			for _ in &nut self.workers {
				self.sender.send(Message::Terminate).unwrap();

			}

			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
	}
}

struct Job;

pub struct Worker {
	id: u32,
	handle: Option<thread::JoinHandle<()>>
}
impl Worker {
	pub fn new(id: u32, receiver:: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let threadHandle = spawn::thread(move || {
			loop {
				let message = receiver.lock().unwrap().recv().unwrap();
				println!("Worker {} got a job; executing...", id);
				match message {
					Message::NewJob(job) => {
						(*job)();
					},
					Message:Terminate => {
						println!("Worker {} was told to terminate.", id);
						break; 
					}
				}
			}
		});
		return Worker {
			id,
			Some(threadHandle),
		}
	}
}