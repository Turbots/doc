use std::{env, process};
use diagnosis::diagnosis::Diagnosis;
use doc::Config;

mod diagnosis;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if config.command == "diagnose" || config.command == "d" {
        let diagnosis = Diagnosis::new();
        diagnosis.perform();
    } else {
        println!("Tanzu Doctor\n\nUSAGE:\n    doc <command>\n\nCOMMANDS:\n    diagnose, d\t\tPerforms diagnosis of all Kubernetes clusters found in kubeconfig.");
        process::exit(1);
    }
}
