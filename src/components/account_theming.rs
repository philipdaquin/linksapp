use crate::components::account_sidebar::AccountSidebar;
use sycamore::prelude::*;

#[component(AccountTheming<G>)]
pub fn AccountTheming<G: Html>(cx: Scope) -> View<G> {
    view! {cx,


    section (class="section mt-60") {div (class="container mt-lg-3") {
          div (class="row") {
              AccountSidebar()

            div (class="col-lg-6 col-md-6 col-12 d-lg-block d-none") {
              h3 (class="title mb-0") {"Theming"}

              h4 (class="title mb-0 d-flex justify-content-center") {"Customize your page"}

              div (class="row row-cols-1 row-cols-md-3 g-4") {
                div (class="col") {
                  div (class="card") {
                    img (src="...", class="card-img-top", alt="...") {}
    div (class="card-body") {
                      h5 (class="card-title") {"Card title"}


                    }

                  }

                }

                div (class="col") {
                  div (class="card") {
                    img (src="...", class="card-img-top", alt="...") {}
    div (class="card-body") {
                      h5 (class="card-title") {"Card title"}


                    }

                  }

                }

                div (class="col") {
                  div (class="card") {
                    img (src="...", class="card-img-top", alt="...") {}
    div (class="card-body") {
                      h5 (class="card-title") {"Card title"}


                    }

                  }

                }

                div (class="col") {
                  div (class="card") {
                    img (src="...", class="card-img-top", alt="...") {}
    div (class="card-body") {
                      h5 (class="card-title") {"Card title"}


                    }

                  }

                }

              }

            }

            div (class="col-lg-4 col-12") {
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
                                    div (class="p-1 bd-highlight") {a (href="#", class="btn btn-outline-primary") { "MyLink" }
    }

                                    div (class="p-1 bd-highlight") {a (href="#", class="btn btn-outline-primary") { "New Product" }
    }

                                    div (class="p-1 bd-highlight") {a (href="#", class="btn btn-outline-primary") { "Read about this" }
    }

                                    div (class="mt-3 d-flex justify-content-between align-items-start") {
                                      img (src="assets/images/client/01.jpg", class="avatar avatar-small rounded") {}
    img (src="assets/images/client/01.jpg", class="avatar avatar-small rounded") {}
    img (src="assets/images/client/01.jpg", class="avatar avatar-small rounded") {}
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
