use sycamore::prelude::*;

#[component(HeroSignup<G>)]
pub fn HeroSignup<G: Html>(cx: Scope) -> View<G> {
    view! {cx,

               section (class="bg-half-170 pb-0 bg-primary d-table w-100", style="background: url('assets/images/bg2.png') center center;") {div (class="container") {
        div (class="row align-items-center") {
          div (class="col-lg-7 col-md-6") {
            div (class="me-lg-5") {
              img (src="assets/images/user/signup.svg", class="img-fluid d-block mx-auto", alt="") {}
    }

          }

          div (class="col-lg-5 col-md-6") {
            div (class="card shadow rounded border-0") {
              div (class="card-body") {
                h4 (class="card-title text-center") {"Mylink: Signup"}

                form (class="login-form mt-4") {
                  div (class="row") {
                    div (class="col-md-6") {
                      div (class="mb-3") {
                        label (class="form-label") {"First name" span (class="text-danger") {"*"}
    }

                        div (class="form-icon position-relative") {
                          i (data-feather="user", class="fea icon-sm icons") {}

                          input (type="text", class="form-control ps-5", placeholder="First Name", name="s", required=false) {}
    }

                      }

                    }


                    div (class="col-md-6") {
                      div (class="mb-3") {
                        label (class="form-label") {"Last name" span (class="text-danger") {"*"}
    }

                        div (class="form-icon position-relative") {
                          i (data-feather="user-check", class="fea icon-sm icons") {}

                          input (type="text", class="form-control ps-5", placeholder="Last Name", name="s", required=false) {}
    }

                      }

                    }


                    div (class="col-md-12") {
                      div (class="mb-3") {
                        label (class="form-label") {"Your Email" span (class="text-danger") {"*"}
    }

                        div (class="form-icon position-relative") {
                          i (data-feather="mail", class="fea icon-sm icons") {}

                          input (type="email", class="form-control ps-5", placeholder="Email", name="email", required=false) {}
    }

                      }

                    }


                    div (class="col-md-12") {
                      div (class="mb-3") {
                        label (class="form-label") {"Password" span (class="text-danger") {"*"}
    }

                        div (class="form-icon position-relative") {
                          i (data-feather="key", class="fea icon-sm icons") {}

                          input (type="password", class="form-control ps-5", placeholder="Password", required=false) {}
    }

                      }

                    }


                    div (class="col-md-12") {
                      div (class="mb-3") {
                        div (class="form-check") {
                          input (class="form-check-input", type="checkbox", value="", id="flexCheckDefault") {}
    label (class="form-check-label", for="flexCheckDefault") {"I Accept" a (href="#", class="text-primary") {"Terms And Condition"}
    }

                        }

                      }

                    }


                    div (class="col-md-12") {
                      div (class="d-grid") {
                        button (class="btn btn-primary") {"Register"}

                      }

                    }


                    div (class="col-lg-12 mt-4 text-center") {
                      h6  {"Or Signup With"}

                      div (class="row") {
                        div (class="col-6 mt-3") {
                          div (class="d-grid") {
                            a (href="javascript:void(0)", class="btn btn-light") {i (class="mdi mdi-facebook text-primary") {}
     "Facebook"}

                          }

                        }


                        div (class="col-6 mt-3") {
                          div (class="d-grid") {
                            a (href="javascript:void(0)", class="btn btn-light") {i (class="mdi mdi-google text-danger") {}
     "Google"}

                          }

                        }

                      }

                    }


                    div (class="mx-auto") {
                      p (class="mb-0 mt-3") {small (class="text-dark me-2") {"Already have an account ?"}
     a (href="auth-login.html", class="text-dark fw-bold") {"Sign in"}
    }

                    }

                  }

                }

              }

            }

          }

        }

      }

    }

                }
}
