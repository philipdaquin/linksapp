use sycamore::prelude::*;

#[component(HeroWidget<G>)]
pub fn HeroWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,


               section (class="bg-half-170 pb-0 bg-primary d-table w-100", style="background: url('assets/images/bg2.png') center center;") {div (class="container") {
            div (class="row align-items-center") {
                div (class="col-lg-7 col-md-6") {
                    div (class="title-heading mb-md-5 pb-md-5") {
                        h4 (class="text-white-50") {"Just one link"}

                        h4 (class="heading text-white mb-3 title-dark") { "For all your products" br  {}
     "and pages "}

                        p (class="para-desc text-white-50") {"Share them via your social channels, emails, forums and micro-pages to enhance visibility and drive more traffic."}

                        div (class="text-left subcribe-form mt-4 pt-2") {
                            form  {
                                input (type="url", id="url", class="border bg-white ", style="opacity: 0.85;", placeholder="@youlinkname") {}
    button (type="submit", class="btn  btn-primary") {"Get Started"}

                            }


                        }



                    }

                }



                div (class="col-lg-5 col-md-6 mt-5 mt-sm-0") {


                    div (class="smartphone") {
                        div (class="device-header") {
                            div (class="proximity") {}

                            div (class="camera") {}

                            div (class="speaker") {}

                        }

                        div (class="content") {

                            div (class="card team team-primary text-center bg-transparent border-0") {
                                div (class="card-body p-0") {
                                    div (class="position-relative") {
                                        img (src="assets/images/client/04.jpg", class="img-fluid avatar avatar-md-md rounded-circle shadow-lg", alt="") {}
    }

                                }

                            }


                            h3 (class="bg-soft-primary text-center") { "@myuser"}


                            div (class="d-flex flex-column bd-highlight mb-3 text-center") {
                                div (class="p-2 bd-highlight") {a (href="#", class="btn btn-outline-primary") { "MyLink" }
    }

                                div (class="p-2 bd-highlight") {a (href="#", class="btn btn-outline-primary") { "New Product" }
    }

                                div (class="p-2 bd-highlight") {a (href="#", class="btn btn-outline-primary") { "Read about this "}
    }

                            }



                        }

                        div (class="device-footer") {
                            div (class="home-button") {}



                        }

                    }

                }

            }


        }


    }


                }
}
