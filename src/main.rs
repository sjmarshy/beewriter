extern crate notify;
extern crate regex;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::fs::{read_dir, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

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

fn get_text_files(p: &Path) -> io::Result<Vec<PathBuf>> {
    let dir = read_dir(p)?;
    let mut res = Vec::new();

    for dir_entry in dir {
        let entry = dir_entry?;
        let path = entry.path();
        let path2 = entry.path();
        let ext = match path2.extension() {
            Some(e) => e.to_str().unwrap(),
            None => "",
        };

        if ext == "txt" || ext == "md" {
            res.push(path);
        }
    }

    Ok(res)
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
    let init: usize = 0;

    let count: usize = match files {
        Ok(fs) => fs.iter()
            .map(|x| {
                // need to exclude non .txt, .md files!
                println!("{:?}", x);
                let y = count_words(x);
                println!("{:?}", y);
                y.unwrap()
            })
            .fold(init, |a: usize, x: usize| a + x),
        Err(e) => panic!("{:?}", e),
    };

    println!("{:?}", count)
}
