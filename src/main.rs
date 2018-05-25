#![feature(proc_macro)]
extern crate clap;
extern crate thunder;
extern crate op;
extern crate serde_json;

use thunder::thunderclap;

use std::process::Command;

struct PassApp;

#[thunderclap]
impl PassApp {
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    fn get(item: &str, designation: Option<&str>) {
        let mut command = Command::new("op");
        let ref_command= command
            .arg("get")
            .arg("item")
            .arg(item);

        let output = ref_command.output().expect("Failed to execute 'op'");
        let item: op::Item = serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).expect("Failed to deserialize item");

        let designation = match designation {
            None => {
                item.details.fields
                    .iter()
                    .for_each(|f| println!("{}: {}", f.designation, f.value));
                return
            },
            Some(d) => d
        };

        let result = item.details.fields
            .iter()
            .find(|f| f.designation == designation);

        if let Some(field) = result {
            println!("{}", field.value);
        } else {
            eprintln!("designation {} not found in fields", designation);
        }
    }
}

fn main() {
    PassApp::start();
}
