use std::io::{self, Write};
use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;


fn main() {
    let mut args = env::args().collect::<Vec<_>>();
    args.remove(0);
    let mut extensions = HashMap::new();
    for arg in args {
        let path = Path::new(&arg);

        match visit_dirs(&path, &mut extensions) {
            Ok(_) => (),
            Err(e) => {
                if let Err(err_stderr) =
                    writeln!(io::stderr(), "{}: {}", path.to_str().unwrap(), e) {
                    panic!("Failed write to stderr\
                    \nOriginal error output: {}\
                    \nSecondary error writing to stderr: {}",
                    e, err_stderr);
                }
            }
        }
    }
    let mut sorted_keys = extensions.keys().collect::<Vec<_>>();
    sorted_keys.sort();
    for key in sorted_keys {
        println!("{}", key);
        for path in extensions.get(key).expect("No paths") {
            println!("  {}", path);
        }
    }
}

fn visit_dirs(dir: &Path, extensions: &mut HashMap<String, Vec<String>>) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), extensions));
            } else {
                match entry.path().extension() {
                    Some(ext) => {
                        let paths = extensions.entry(ext.to_str().unwrap().to_string()).or_insert(Vec::new());
                        paths.push(entry.path().to_str().unwrap_or_default().to_string());
                    }
                    None => {
                        let paths = extensions.entry("NO_EXT".to_string()).or_insert(Vec::new());
                        paths.push(entry.path().to_str().unwrap().to_string());
                    }
                }
            }
        }
    } else {
        match dir.extension() {
            Some(ext) => {
                let paths = extensions.entry(ext.to_str().unwrap().to_string()).or_insert(Vec::new());
                paths.push(dir.to_str().unwrap().to_string());
            }
            None => {
                let paths = extensions.entry("NO_EXT".to_string()).or_insert(Vec::new());
                paths.push(dir.to_str().unwrap().to_string());
            }
        }
    }
    Ok(())
}
