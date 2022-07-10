mod components;
mod error_pages;
mod templates;

use crate::components::{footer::FooterWidget, header::HeaderWidget};
use std::collections::HashMap;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use perseus::{web_log, Html, PerseusApp, PerseusRoot};
#[perseus::main(perseus_actix_web::dflt_server)]

pub fn main<G: Html>() -> PerseusApp<G> {
    // console_log::init_with_level(log::Level::Debug).unwrap();
    let mut static_paths: HashMap<String, String> = HashMap::new();

    let pathstr = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(pathstr);
    let target = p.join("assets");
    let strip_path = PathBuf::from("static");

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

        let k = Path::new("/assets").join(a.clone());

        let bf = Path::new(rp).join(a.clone());

        static_paths.insert(k.display().to_string(), bf.display().to_string());
        // log::debug!("Content -> {:#?}", static_paths);
        // web_log!("Content -> {:#?}", static_paths);
    }

    PerseusApp::new()
        .template(crate::templates::index::get_template)
  // .template(crate::templates::about::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .static_aliases(static_paths)
        //.static_alias("style.css", "/style.css")
            .index_view(|cx| {
                sycamore::view! { cx,

                    head {

meta (charset="utf-8") 
meta (name="viewport", content="width=device-width, initial-scale=1.0") 
meta (name="description", content="Premium Bootstrap 5 Landing Page Template") 
meta (name="keywords", content="Saas, Software, multi-uses, HTML, Clean, Modern") 
meta (name="author", content="Shreethemes") 
meta (name="email", content="codelot@yahoo.com") 
meta (name="website", content="https://flashtechhub.com") 
link (rel="shortcut icon", href="assets/images/favicon.ico") 
link (href="assets/css/bootstrap.min.css", class="theme-opt", rel="stylesheet", type="text/css") 
link (href="assets/css/icons.min.css", rel="stylesheet", type="text/css") 
link (href="assets/libs/@iconscout/unicons/css/line.css", type="text/css", rel="stylesheet") 
link (href="assets/css/style.min.css", class="theme-opt", rel="stylesheet", type="text/css") 

link (href="assets/libs/tobii/css/tobii.min.css", rel="stylesheet") 

                    }
                    body {

                        // This creates an element into which our app will be interpolated
                        // This uses a few tricks internally beyond the classic `<div id="root">`, so we use this wrapper for convenience
                         PerseusRoot()
                         HeaderWidget()
                        // Because this is in the index view, this will be below every single one of our pages
                        // Note that elements in here can't be selectively removed from one page, it's all-or-nothing in the index view (it wraps your whole app)
                        // Note also that this won't be reloaded, even when the user switches pages
                        //  footer { "This is a footer!" }
                        FooterWidget()
                    }
                }
            })
}
