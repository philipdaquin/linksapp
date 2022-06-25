mod error_pages;
mod templates;
use std::collections::HashMap;

use std::path::Path;
use walkdir::WalkDir;

use perseus::{web_log, Html, PerseusApp};

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    let mut static_paths: HashMap<String, String> = HashMap::new();

    let pathstr = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(pathstr);
    let target = p.join("static");
    let rp = target.file_name().unwrap();

    for x in WalkDir::new(&target)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file())
    {
        let a = x
            .path()
            .strip_prefix(&target)
            .unwrap()
            .to_str()
            .expect("invalid utf8 in path")
            .to_owned();
        let k = Path::new("/").join(a.clone());

        let bf = Path::new(rp).join(a.clone());

        static_paths.insert(k.display().to_string(), bf.display().to_string());
    }

    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        // .static_alias("/style.css", "static/style.css")
        .static_aliases(static_paths)
}
