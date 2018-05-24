#![feature(proc_macro)]
extern crate clap;
extern crate thunder;
use thunder::thunderclap;

struct PassApp;

#[thunderclap]
impl PassApp {
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
}

fn main() {
    PassApp::start();
}
