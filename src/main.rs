mod components;
mod error_pages;
mod templates;

use components::footer::FooterWidget;
use components::header::HeaderWidget;

use perseus::{web_log, Html, PerseusApp, PerseusRoot};

use std::collections::HashMap;

use std::path::Path;
use walkdir::WalkDir;

#[perseus::main(perseus_actix_web::dflt_server)]
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

        let k = Path::new("/assets").join(a.clone());

        let bf = Path::new(rp).join(a.clone());

        static_paths.insert(k.display().to_string(), bf.display().to_string());
        // log::debug!("Content -> {:#?}", static_paths);
        web_log!("Content -> {:#?}", static_paths);
    }

    PerseusApp::new()
.template(crate::templates::index::get_template)
.error_pages(crate::error_pages::get_error_pages)
.static_aliases(static_paths)
 .index_view(|cx| {
                sycamore::view! { cx,

                    head {

                            meta (http-equiv="X-UA-Compatible", content="IE=edge") 
                            meta (name="viewport", content="width=device-width, initial-scale=1, minimum-scale=1.0, shrink-to-fit=no") 
                            link (href="images/favicon.png", rel="icon") 
                            meta (name="viewport", content="width=device-width, initial-scale=1.0")
                            meta (name="description", content="LinksApp, Share all your pages in one link")
                            meta (name="keywords", content="linksapp, share links")
                            link (rel="shortcut icon", href="assets/images/favicon.ico") 

                            link (href="assets/css/bootstrap.min.css", class="theme-opt", rel="stylesheet", type="text/css") 
                            link (href="assets/css/icons.min.css", rel="stylesheet", type="text/css") 
                            link (href="assets/libs/unicons/css/line.css", type="text/css", rel="stylesheet") 
                            link (href="assets/css/style.min.css", class="theme-opt", rel="stylesheet", type="text/css") 

                        link (href="assets/libs/tiny-slider/tiny-slider.css", rel="stylesheet") 
                        link (href="assets/libs/tobii/css/tobii.min.css", rel="stylesheet") 

                    }

                    body {
                         PerseusRoot()
                        HeaderWidget()
                                                         FooterWidget()




    // a (id="back-to-top", data-toggle="tooltip", title="Back to Top", href="javascript:void(0)") {i (class="fa fa-chevron-up")     }




    // script (src="vendor/jquery/jquery.min.js")
    // script (src="vendor/bootstrap/js/bootstrap.bundle.min.js")
    // script (src="vendor/bootstrap-select/js/bootstrap-select.min.js")
    // script (src="vendor/owl.carousel/owl.carousel.min.js")
    // script (src="js/theme.js")


                    }
                }
            })
}
