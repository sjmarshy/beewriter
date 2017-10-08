extern crate notify;
extern crate regex;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::fs::{read_dir, ReadDir, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

/*
fn watch() -> notify::Result<()> {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(2)));

    // TODO: env variable for dir to watch
    try!(watcher.watch("/Users/sam/Dropbox/notes", RecursiveMode::NonRecursive));

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
*/

fn get_file_path(dir: ReadDir) -> Vec<io::Result<PathBuf>> {
    dir.map(|entry| entry.and_then(|p| Ok(p.path()))).collect::<Vec<io::Result<PathBuf>>>()
}

fn get_files(p: &Path) -> Vec<io::Result<PathBuf>> {
    read_dir(p).and_then(|dir| Ok(get_file_path(dir))).unwrap_or(Vec::new())
}

fn is_txt_or_md(pb: &PathBuf) -> bool {
    let ext = pb.extension().and_then(|e| e.to_str()).unwrap_or("");

    ext == "txt" || ext == "md"
}

fn get_text_files(p: &Path) -> io::Result<Vec<PathBuf>> {
    get_files(p)
        .into_iter()
        .filter(|path_buf| path_buf.as_ref().and_then(|pb| Ok(is_txt_or_md(pb))).unwrap_or(false))
        .collect::<Result<Vec<_>, _>>()
}

fn count_words(file_path: &PathBuf) -> io::Result<usize> {
    let word_re = Regex::new(r"[a-z']+").unwrap();
    let mut f = File::open(file_path)?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    let buffer = s.to_string();
    let ms = word_re.find_iter(&buffer);

    Ok(ms.count())
}

fn main() {
    let path = Path::new("/Users/sam/Dropbox/notes");
    let files = get_text_files(path);

    let initial_count: usize = files.unwrap_or(Vec::new())
        .into_iter()
        .map(|file| count_words(&file))
        .fold(0, |a, x| x.unwrap_or(0) + a);

    println!("words: {:?}", initial_count);
}
