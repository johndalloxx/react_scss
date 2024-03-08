use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // Retrieve the filename argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let name = &args[1];

    // Generate .tsx file content
    let tsx_content = format!(
        "import React from 'react';

import styles from './{name}.module.scss';

type {name}Props = {{

}};

const {name} = ({{ }}: {name}Props) => {{
    return (
        <div>
        
        </div>
    );
}};

export default {name};
    ",
        name = name
    );

    // Generate file paths
    let tsx_filename = format!("{}.tsx", name);
    let scss_filename = format!("{}.module.scss", name);

    let tsx_path = Path::new(&tsx_filename);
    let scss_path = Path::new(&scss_filename);

    // Create and write to .tsx file
    let mut tsx_file = File::create(&tsx_path).expect("Failed to create .tsx file");
    tsx_file
        .write_all(tsx_content.as_bytes())
        .expect("Failed to write to .tsx file");

    // Create and leave the .module.scss file empty
    let _scss_file = File::create(&scss_path).expect("Failed to create .module.scss file");

    println!(
        "Files created successfully: {} and {}",
        tsx_path.display(),
        scss_path.display()
    );
}
