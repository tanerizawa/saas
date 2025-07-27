use std::env;
use std::process::Command;

fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args...]", args[0]);
        eprintln!("Commands: build, run, test, bench, cover, format, check, clean");
        std::process::exit(1);
    }

    let command = &args[1];
    
    match command.as_str() {
        "build" => run_build(),
        "run" => run_server(),
        "test" => run_tests(),
        "bench" => run_benchmarks(),
        "cover" => run_coverage(),
        "format" => run_format(),
        "check" => run_check(),
        "clean" => run_clean(),
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}

fn run_build() {
    println!("Building project...");
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .status()
        .expect("Failed to execute build command");
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn run_server() {
    println!("Running server...");
    let status = Command::new("cargo")
        .args(["run", "--bin", "server"])
        .status()
        .expect("Failed to execute run command");
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn run_tests() {
    println!("Running tests...");
    
    // Check if we should run a specific test or all tests
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        // Run specific test
        let specific_test = &args[2];
        let status = Command::new("./scripts/run-tests.sh")
            .args(["--specific", specific_test])
            .status()
            .expect("Failed to execute test command");
        
        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
    } else {
        // Run all tests
        let status = Command::new("./scripts/run-tests.sh")
            .status()
            .expect("Failed to execute test command");
        
        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
    }
}

fn run_benchmarks() {
    println!("Running benchmarks...");
    let status = Command::new("./scripts/run-tests.sh")
        .args(["--benchmarks"])
        .status()
        .expect("Failed to execute benchmark command");
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn run_coverage() {
    println!("Running test coverage...");
    let status = Command::new("./scripts/run-tests.sh")
        .args(["--coverage"])
        .status()
        .expect("Failed to execute coverage command");
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn run_format() {
    println!("Formatting code...");
    let status = Command::new("cargo")
        .args(["fmt"])
        .status()
        .expect("Failed to execute format command");
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn run_check() {
    println!("Running code quality checks...");
    let status = Command::new("./scripts/quality-check.sh")
        .status()
        .expect("Failed to execute quality check command");
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn run_clean() {
    println!("Cleaning project...");
    let status = Command::new("cargo")
        .args(["clean"])
        .status()
        .expect("Failed to execute clean command");
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
