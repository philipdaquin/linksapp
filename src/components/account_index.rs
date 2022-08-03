use crate::components::account_sidebar::AccountSidebar;
use sycamore::prelude::*;

#[component(AccountWidget<G>)]
pub fn AccountWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,

               section (class="section mt-60") {div (class="container mt-lg-3") {
              div (class="row") {
                  // Account Sidebar
                      AccountSidebar()
                    div (class="col-lg-6 col-md-6 col-12 d-lg-block d-none") {
                        h3 (class="title mb-0") {"Links"}


                        ul (class="list-group ") {li (class="list-group-item d-flex justify-content-between align-items-start") {
                                div (class="ms-2 me-auto") {
                                    div (class="fw-bold") {"My Products"}

                                   " products.com"
                                }

                                span (class="badge bg-primary rounded-pill") {"14"}

                            }


                            li (class="list-group-item d-flex justify-content-between align-items-start") {
                                div (class="ms-2 me-auto") {
                                    div (class="fw-bold") {"Subheading"}

                                    "Content for list item"
                                }

                                span (class="badge bg-primary rounded-pill") {"14"}

                            }

                            li (class="list-group-item d-flex justify-content-between align-items-start") {
                                div (class="ms-2 me-auto") {
                                    div (class="fw-bold") {"Subheading"}

                                    "Content for list item"
                                }

                                span (class="badge bg-primary rounded-pill") {"14"}

                            }

                            li (class="list-group-item d-flex justify-content-between align-items-start") {
                              img (src="assets/images/client/01.jpg", class="avatar avatar-small rounded") {}
    img (src="assets/images/client/01.jpg", class="avatar avatar-small rounded") {}
    img (src="assets/images/client/01.jpg", class="avatar avatar-small rounded") {}
    span (class="badge bg-primary rounded-pill") {"14"}

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

                                    div (class="p-1 bd-highlight") {a (href="#", class="btn btn-outline-primary") { "Check This" }
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
