use std::fs;
use std::path::Path;
use std::error::Error;
use std::process;
use std::collections::HashMap;

fn main() {
    if let Err(ref e) = run(Path::new(".")) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(dir: &Path) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry
                            .file_name()
                            .into_string()
                            .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            
            let metadata = entry
                              .metadata()?;

            // let file_type_str = get_file_type(&metadata).unwrap_or("unknown");

            // println!("{:?}", file_type_str());
            
            println!("{}", file_name);
            
        }
    }
    Ok(())
}

// fn get_file_type(metadata: &fs::Metadata) -> Option<&'static str> {
//     if metadata.is_file() {
//         ("-")
//     } else if metadata.is_dir() {
//         "d"
//     } else if metadata.file_type().is_symlink() {
//         "s"
//     } else {
//         None
//     }
// }