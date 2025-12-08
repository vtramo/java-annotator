use std::fs::{DirEntry, File, read_dir};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JavaFile(String);

impl JavaFile {
    pub fn path(&self) -> &str {
        &self.0
    }
}

pub(crate) fn collect_java_files(path: &str) -> Result<Vec<JavaFile>, std::io::Error> {
    let java_file_paths: Vec<_> = collect_paths(path)?
        .into_iter()
        .filter(|path| path.ends_with(".java"))
        .map(JavaFile)
        .collect();

    Ok(java_file_paths)
}

pub(crate) fn collect_paths(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;
    if metadata.is_dir() {
        let paths: Vec<String> = read_dir(path)?
            .flat_map(|entry| collect_paths_entry_dir(&entry?))
            .flatten()
            .collect();
        Ok(paths)
    } else {
        Ok(vec![path.to_string()])
    }
}

fn collect_paths_entry_dir(entry_dir: &DirEntry) -> Result<Vec<String>, std::io::Error> {
    let mut paths: Vec<String> = vec![];

    let metadata = entry_dir.metadata()?;
    if metadata.is_dir() {
        let dir_paths: Vec<String> = read_dir(entry_dir.path())?
            .flat_map(|entry| collect_paths_entry_dir(&entry?))
            .flatten()
            .collect();
        paths.extend(dir_paths);
    } else {
        return Ok(vec![entry_dir.path().display().to_string()]);
    }

    Ok(paths)
}
