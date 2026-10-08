use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::{Parser, Subcommand, ValueEnum};
use guard_core::{
    CompatibilityImpact, CompatibilityReport, CompatibilityResult, ContractInterface,
    compare_interfaces, parse_contract_interface_file,
};

#[derive(Debug, Parser)]
#[command(name = "stellar-upgrade-guard")]
#[command(about = "Inspect and compare Soroban contract specifications")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Inspect the contractspecv0 interface embedded in a Soroban WASM file.
    Inspect {
        /// Path to the compiled Soroban WASM file.
        wasm: PathBuf,

        /// Output format.
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
    /// Compare two local Soroban WASM contract interfaces.
    Compare {
        /// Path to the previous compiled Soroban WASM file.
        #[arg(long)]
        old: PathBuf,

        /// Path to the candidate compiled Soroban WASM file.
        #[arg(long)]
        new: PathBuf,

        /// Output format.
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode, Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::Inspect { wasm, format } => {
            let interface = parse_contract_interface_file(wasm)?;
            match format {
                OutputFormat::Text => print_text(&interface),
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&interface)?),
            }
            Ok(ExitCode::SUCCESS)
        }
        Command::Compare { old, new, format } => {
            let old_interface = parse_contract_interface_file(&old)?;
            let new_interface = parse_contract_interface_file(&new)?;
            let report = compare_interfaces(&old_interface, &new_interface);
            match format {
                OutputFormat::Text => print_compare_text(&old, &new, &report),
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
            }
            Ok(compare_exit_code(report.result))
        }
    }
}

fn print_text(interface: &ContractInterface) {
    println!("Contract Interface");
    print_functions(interface);
    print_types(interface);
    print_errors(interface);
    print_events(interface);
}

fn print_functions(interface: &ContractInterface) {
    println!();
    println!("Functions:");
    if interface.functions.is_empty() {
        println!("- none");
        return;
    }
    for function in &interface.functions {
        let inputs = function
            .inputs
            .iter()
            .map(|input| format!("{}: {}", input.name, input.type_ref.0))
            .collect::<Vec<_>>()
            .join(", ");
        let outputs = function
            .outputs
            .iter()
            .map(|output| output.0.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        if outputs.is_empty() {
            println!("- {}({inputs})", function.name);
        } else {
            println!("- {}({inputs}) -> {outputs}", function.name);
        }
    }
}

fn print_types(interface: &ContractInterface) {
    println!();
    println!("Types:");
    let mut printed = false;
    for item in &interface.structs {
        printed = true;
        println!("- struct {}", item.name);
    }
    for item in &interface.enums {
        printed = true;
        println!("- enum {}", item.name);
    }
    for item in &interface.unions {
        printed = true;
        println!("- union {}", item.name);
    }
    if !printed {
        println!("- none");
    }
}

fn print_errors(interface: &ContractInterface) {
    println!();
    println!("Errors:");
    if interface.errors.is_empty() {
        println!("- none");
        return;
    }
    for error in &interface.errors {
        for case in &error.cases {
            println!("- {}.{} = {}", error.name, case.name, case.value);
        }
    }
}

fn print_events(interface: &ContractInterface) {
    println!();
    println!("Events:");
    if interface.events.is_empty() {
        println!("- none");
        return;
    }
    for event in &interface.events {
        println!("- {} ({})", event.name, event.data_format);
    }
}

fn print_compare_text(old: &Path, new: &Path, report: &CompatibilityReport) {
    println!("Stellar Upgrade Guard");
    println!();
    println!("Comparing:");
    println!("{}", old.display());
    println!("-> {}", new.display());
    print_findings("Breaking changes", CompatibilityImpact::Breaking, report);
    print_findings("Unknown changes", CompatibilityImpact::Unknown, report);
    print_findings("Warnings", CompatibilityImpact::Warning, report);
    print_findings(
        "Compatible changes",
        CompatibilityImpact::Compatible,
        report,
    );
    println!();
    match report.result {
        CompatibilityResult::Breaking => println!("Result: BREAKING"),
        CompatibilityResult::Unknown => println!("Result: UNKNOWN"),
        CompatibilityResult::Warning => println!("Result: WARNING"),
        CompatibilityResult::Compatible => println!("Result: COMPATIBLE"),
    }
}

fn print_findings(title: &str, impact: CompatibilityImpact, report: &CompatibilityReport) {
    println!();
    println!("{title}:");
    let findings = report
        .findings
        .iter()
        .filter(|finding| finding.impact == impact)
        .collect::<Vec<_>>();
    if findings.is_empty() {
        println!("- none");
        return;
    }
    for finding in findings {
        println!("- {}", finding.message);
    }
}

fn compare_exit_code(result: CompatibilityResult) -> ExitCode {
    match result {
        CompatibilityResult::Compatible | CompatibilityResult::Warning => ExitCode::SUCCESS,
        CompatibilityResult::Breaking | CompatibilityResult::Unknown => ExitCode::from(1),
    }
}
