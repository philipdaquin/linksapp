use sycamore::prelude::*;

#[component(HeroWidget<G>)]
pub fn HeroWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
    section (class="bg-home bg-primary d-flex align-items-center", style="background: url('assets/images/integration/bg.png') center center; height: auto;", id="home") {div (class="container") {
                    div (class="row mt-5 justify-content-center") {
                        div (class="col-lg-12 text-center mt-0 mt-md-5 pt-0 pt-md-5") {
                            div (class="title-heading") {

                                h4 (class="heading text-white title-dark my-3") {"Sharing your micro-page is never" br  {}
    b  {"Easier"}
     "and " b  {"Viral"}
    }

                                p (class="para-desc mx-auto text-white-50") {"Create a micro page and share your links, products, and reach millions in a second. Monitor your engagement live"}

                            }


                            div (class="text-center subcribe-form mt-4 pt-2") {
                                form  {
                                    input (type="url", id="url", class="border bg-white rounded-lg", style="opacity: 0.85;", placeholder="https://linksapp.test") {}
    button (type="submit", class="btn btn-pills btn-primary") {"Create a Link"}

                                }

                            }


                            div (class="row justify-content-center") {
                                div (class="col-lg-8 col-md-10") {
                                    div (class="home-dashboard") {
                                        img (src="assets/images/integration/hero.png", alt="", class="img-fluid") {}
    }

                                }

                            }

                        }

                    }

                }

            }
    div (class="position-relative") {
                div (class="shape integration-hero overflow-hidden text-light") {}

            }




            section (class="mt-5 pt-md-5") {div (class="container") {
                    div (class="row justify-content-center") {
                        div (class="col-lg-2 col-md-2 col-6 text-center py-4 py-sm-0") {
                            img (src="assets/images/client/amazon.svg", class="avatar avatar-ex-sm", alt="") {}
    }


                        div (class="col-lg-2 col-md-2 col-6 text-center py-4 py-sm-0") {
                            img (src="assets/images/client/google.svg", class="avatar avatar-ex-sm", alt="") {}
    }


                        div (class="col-lg-2 col-md-2 col-6 text-center py-4 py-sm-0") {
                            img (src="assets/images/client/lenovo.svg", class="avatar avatar-ex-sm", alt="") {}
    }


                        div (class="col-lg-2 col-md-2 col-6 text-center py-4 py-sm-0") {
                            img (src="assets/images/client/paypal.svg", class="avatar avatar-ex-sm", alt="") {}
    }


                        div (class="col-lg-2 col-md-2 col-6 text-center py-4 py-sm-0") {
                            img (src="assets/images/client/shopify.svg", class="avatar avatar-ex-sm", alt="") {}
    }


                        div (class="col-lg-2 col-md-2 col-6 text-center py-4 py-sm-0") {
                            img (src="assets/images/client/spotify.svg", class="avatar avatar-ex-sm", alt="") {}
    }

                    }

                }

            }

            }
}
