use clap::Parser;
use std::path::PathBuf;

mod annotator;
mod declaration;
mod io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Java source: a single .java file or a directory
    /// containing files (recursively processed).
    path: PathBuf,

    /// List of annotations to check for (e.g., @A @B).
    /// Specify the argument multiple times: -a @A -a @B
    #[arg(short, long, value_name = "ANNOTATION")]
    annotations: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = match args.path.to_str() {
        None => Err("Invalid path")?,
        Some(path) => path,
    };

    let java_files = io::collect_java_files(path)?;

    let java_annotations = args
        .annotations
        .iter()
        .map(|s| annotator::JavaAnnotation::new(s))
        .collect::<Result<Vec<_>, _>>()?;

    let edits = annotator::annotate_java_files(&java_files, &java_annotations);
    for edit in edits.iter() {
        if edit.is_modified() {
            println!("{}", edit);
        }
    }

    Ok(())
}
