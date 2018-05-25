#![feature(proc_macro)]
extern crate clap;
extern crate thunder;
use thunder::thunderclap;

use std::process::Command;

struct PassApp;

#[thunderclap]
impl PassApp {
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    fn get(item: &str, designation: Option<&str>) {
        println!("{:?} {:?}", item, designation);
        let mut command = Command::new("op");
        let mut_command = command
            .arg("get")
            .arg("item")
            .arg(item);

        if let Some(_designation) = designation {
        }

        let output = mut_command.output().expect("Failed to execute 'op'");
        println!("{:?} {:?}", output, mut_command);
    }
}

fn main() {
    PassApp::start();
}
