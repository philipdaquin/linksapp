use sycamore::prelude::*;

#[component(AccountSidebar<G>)]
pub fn AccountSidebar<G: Html>(cx: Scope) -> View<G> {
    view! {cx,

               div (class="col-lg-2 col-md-6 col-12 d-lg-block d-none") {
        div (class="sidebar sticky-bar p-4 rounded shadow") {
            div (class="widget") {
                h5 (class="widget-title d-flex") {"Hits"}

                div (class="row mt-4") {
                    div (class="col-6 text-center") { i (data-feather="user-plus", class="fea icon-ex-md text-primary mb-1") {}

                        h5 (class="mb-0") {"2588"}

                    }



                }


            }

            div (class="widget mt-4 pt-2 ") {
                h5 (class="widget-title d-flex justify-content-center") {"Account"}

            }

            div (class="widget mt-4") {
                ul (class="list-unstyled sidebar-nav mb-0", id="navmenu-nav") {li (class="navbar-item account-menu px-0 mt-2") { a (href="account-messages.html", class="navbar-link d-flex rounded shadow align-items-center py-2 px-2") { span (class="h4 mb-0") {i (class="uil uil-envelope-star") {}
    }

                            h6 (class="mb-0 ms-1") {"links"}

                        }
     }

                    li (class="navbar-item account-menu px-0 mt-2") { a (href="account-payments.html", class="navbar-link d-flex rounded shadow align-items-center py-2 px-2") { span (class="h4 mb-0") {i (class="uil uil-transaction") {}
    }

                            h6 (class="mb-0 ms-1") {"Theming"}

                        }
     }

                    li (class="navbar-item account-menu px-0 mt-2") { a (href="account-setting.html", class="navbar-link d-flex rounded shadow align-items-center py-2 px-2") { span (class="h4 mb-0") {i (class="uil uil-setting") {}
    }

                            h6 (class="mb-0 ms-1") {"Settings"}

                        }
     }

                    li (class="navbar-item account-menu px-0 mt-2") { a (href="auth-login-three.html", class="navbar-link d-flex rounded shadow align-items-center py-2 px-2") { span (class="h4 mb-0") {i (class="uil uil-dashboard") {}
    }

                            h6 (class="mb-0 ms-1") {"Logout"}

                        }
     }

                }
    }

        }

    }


                }
}
