use sycamore::prelude::*;

#[component(FooterWidget<G>)]
pub fn FooterWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,

        footer (class="footer footer-bar") {div (class="footer-py-30") {
                div (class="container text-center") {
                    div (class="row align-items-center") {
                        div (class="col-sm-3") {
                            div (class="text-sm-start") {
                                a (href="#", class="logo-footer") {
                                    img (src="assets/images/logo-light.png", height="24", alt="") {} }      }           }


                        div (class="col-sm-6 mt-4 mt-sm-0 pt-2 pt-sm-0") {
                            div (class="text-center") {
                                p (class="mb-0") {  "Afidegnum"  i (class="mdi mdi-heart text-danger") {}
         "by" a (href="linksapp.app", target="_blank", class="text-reset") {"LinksApp"}
        }

                            }

                        }


                        div (class="col-sm-3 mt-4 mt-sm-0 pt-2 pt-sm-0") {
                            ul (class="list-unstyled social-icon foot-social-icon text-sm-end mb-0") {li (class="list-inline-item mb-0") {a (href="javascript:void(0)", class="rounded") {i (data-feather="facebook", class="fea icon-sm fea-social") {}
        }
        }

                                li (class="list-inline-item mb-0") {a (href="javascript:void(0)", class="rounded") {i (data-feather="instagram", class="fea icon-sm fea-social") {}
        }
        }

                                li (class="list-inline-item mb-0") {a (href="javascript:void(0)", class="rounded") {i (data-feather="twitter", class="fea icon-sm fea-social") {}
        }
        }

                                li (class="list-inline-item mb-0") {a (href="javascript:void(0)", class="rounded") {i (data-feather="linkedin", class="fea icon-sm fea-social") {}
        }
        }

                            }
        }

                    }

                }

            }

        }


    // // cookings popup
    // div (class="card cookie-popup shadow rounded py-3 px-4") {
    //     p (class="text-muted mb-0") {"This website uses cookies to provide you with a great user experience. By using it, you accept our" a (href="https://linksapp.app", target="_blank", class="text-success h6") {"use of cookies"}
    // }

    //     div (class="cookie-popup-actions text-end") {
    //         button  {i (class="uil uil-times text-dark fs-4") {}
    // }

    //     }

    // }

     // scroll to top
    a (href="#", onclick="topFunction()", id="back-to-top", class="back-to-top fs-5") {i (data-feather="arrow-up", class="fea icon-sm icons align-middle")  }


       // script (src="assets/libs/bootstrap/js/bootstrap.bundle.min.js")
       // script (src="assets/libs/feather-icons/feather.min.js")
       // script (src="assets/libs/tiny-slider/min/tiny-slider.js") {}
       // script (src="assets/libs/tobii/js/tobii.min.js") {}
       // script (src="assets/js/plugins.init.js") {}
       // script (src="assets/js/app.js") {}

                    }
}
