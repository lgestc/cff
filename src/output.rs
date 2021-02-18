use std::{collections::HashMap, path::PathBuf};

pub fn render_output(output: &HashMap<String, PathBuf>) {
    for (_, path) in output {
        println!("'{}'", path.to_str().unwrap())
    }
}
