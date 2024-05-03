use std::fs::File;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;
pub struct Searcher {
    paths: Vec<String>,
}

impl Searcher {
    pub fn new(positional: &Path) -> Searcher {
        let mut s = Searcher { paths: vec![] };
        if positional.is_file() {
            s.paths.push(positional.to_str().unwrap().to_string());
        } else if positional.is_dir() {
            let walk = WalkDir::new(positional);
            for it in walk.into_iter() {
                s.paths
                    .push(it.unwrap().path().to_str().unwrap().to_string());
            }
        }
        s
    }

    pub fn search(self) {
        for path in self.paths {
            let mut file = File::open(path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
        }
    }
}
