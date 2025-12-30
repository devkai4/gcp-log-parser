use std::env;

fn main() {
    println!("GCP Log Parser v0.1.0\n");
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <log-file.json>", args[0]);
        println!("Example: {} logs/sample.json", args[0]);
        return;
    }
    
    let file_path = &args[1];
    println!("📁 Log file: {}", file_path);
    println!("✅ Ready to parse (implementation coming soon...)");
}