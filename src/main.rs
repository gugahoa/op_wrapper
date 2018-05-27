#![feature(proc_macro)]
extern crate clap;
extern crate thunder;
extern crate op;
extern crate serde_json;
extern crate spinner;
extern crate isatty;

use spinner::SpinnerBuilder;
use thunder::thunderclap;
use std::process::{Command, Stdio, exit};
use isatty::stdout_isatty;

fn validate_requirements(namespace: Option<String>) {
    let code = Command::new("op")
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute 'op'")
        .code()
        .expect("Received no exit status from 'op'");
    if code == 127 {
        eprintln!("'op' not found in $PATH");
        exit(1);
    }

    let namespace = match namespace {
        None => {
            eprintln!("Warning: Namespace not specified, using: my");
            "my".into()
        },
        Some(namespace) => namespace
    };

    let session_key = format!("OP_SESSION_{}", namespace);
    if std::env::var(&session_key).is_err() {
        eprintln!("Warning: 1Password not set for '{}'", namespace);
        let output = Command::new("op")
            .arg("signin")
            .arg(namespace)
            .arg("--output=raw")
            .stdin(Stdio::inherit())
            .output()
            .expect("Failed to execute 'op'");

        if !output.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            exit(output.status.code().unwrap());
        }

        let session_value = String::from_utf8_lossy(&output.stdout).to_string();
        std::env::set_var(session_key, session_value.trim());
    }
}

struct PassApp;

#[thunderclap(namespace: Option<String>: "1Password namespace to use", vault: Option<String>: "Specify Vault to look for items")]
impl PassApp {
    fn get(item: &str, designation: Option<&str>) {
        validate_requirements(Self::namespace());
        let sp = if stdout_isatty() {
            Some(SpinnerBuilder::new("Waiting on op-cli...".into()).start())
        } else { None };

        let mut command = Command::new("op");
        let command_builder = command
            .arg("get")
            .arg("item")
            .arg(item);

        if let Some(vault) = Self::vault() {
            command_builder.arg(format!("--vault={}", vault));
        }

        let output = command_builder.output().expect("Failed to execute 'op'");

        if let Some(s) = sp {
            s.done("Done!\n".into());
            s.close();
        }

        if !output.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            return;
        }

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
