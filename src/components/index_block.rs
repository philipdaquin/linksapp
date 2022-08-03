use sycamore::prelude::*;

#[component(BlockWidget<G>)]
pub fn BlockWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,

    section (class="py-4 bg-light") {div (class="container") {
              div (class="row justify-content-center") {
                div (class="col-lg-2 col-md-2 col-6 text-center py-4") {
                  img (src="assets/images/client/amazon.svg", class="avatar avatar-ex-sm", alt="")
    }


                div (class="col-lg-2 col-md-2 col-6 text-center py-4") {
                  img (src="assets/images/client/google.svg", class="avatar avatar-ex-sm", alt="")
    }


                div (class="col-lg-2 col-md-2 col-6 text-center py-4") {
                  img (src="assets/images/client/lenovo.svg", class="avatar avatar-ex-sm", alt="")
    }


                div (class="col-lg-2 col-md-2 col-6 text-center py-4") {
                  img (src="assets/images/client/paypal.svg", class="avatar avatar-ex-sm", alt="")
    }


                div (class="col-lg-2 col-md-2 col-6 text-center py-4") {
                  img (src="assets/images/client/shopify.svg", class="avatar avatar-ex-sm", alt="")
    }


                div (class="col-lg-2 col-md-2 col-6 text-center py-4") {
                  img (src="assets/images/client/spotify.svg", class="avatar avatar-ex-sm", alt="")
    }

              }

            }

          }
    section (class="section bg-light") {div (class="container") {
              div (class="row align-items-center mb-4 pb-2") {
                div (class="col-lg-6") {
                  div (class="section-title text-center text-lg-start") {

                    h4 (class="title mb-4 mb-lg-0") {"Latest updates " br  {}
    }

                  }

                }


                div (class="col-lg-6") {
                  div (class="section-title text-center text-lg-start") {
                    p (class="text-muted mb-0 mx-auto para-desc") {"Start working with " span (class="text-primary fw-bold") {"LinksApp"}
    " that can provide everything you need to generate awareness, drive traffic, connect."}

                  }

                }

              }


              div (class="row") {
                div (class="col-lg-4 col-md-6 mt-4 pt-2") {
                  div (class="card blog blog-primary rounded border-0 shadow") {
                    div (class="position-relative") {
                      img (src="assets/images/blog/01.jpg", class="card-img-top rounded-top", alt="...") {}
    div (class="overlay rounded-top") {}

                    }

                    div (class="card-body content") {
                      h5  {a (href="javascript:void(0)", class="card-title title text-dark") {"Drive traffic the novel way with less efforts"}
    }


                    }


                  }

                }


                div (class="col-lg-4 col-md-6 mt-4 pt-2") {
                  div (class="card blog blog-primary rounded border-0 shadow") {
                    div (class="position-relative") {
                      img (src="assets/images/blog/02.jpg", class="card-img-top rounded-top", alt="...") {}
    div (class="overlay rounded-top") {}

                    }

                    div (class="card-body content") {
                      h5  {a (href="javascript:void(0)", class="card-title title text-dark") {"Linksapp, all your links to a micro page"}
    }


                    }


                  }

                }


                div (class="col-lg-4 col-md-6 mt-4 pt-2") {
                  div (class="card blog blog-primary rounded border-0 shadow") {
                    div (class="position-relative") {
                      img (src="assets/images/blog/03.jpg", class="card-img-top rounded-top", alt="...") {}
    div (class="overlay rounded-top") {}

                    }

                    div (class="card-body content") {
                      h5  {a (href="javascript:void(0)", class="card-title title text-dark") {"Smartest Applications for Business"}
    }


                    }


                  }

                }

              }

            }

          }

            }
}
