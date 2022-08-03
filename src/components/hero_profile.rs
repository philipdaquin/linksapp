use sycamore::prelude::*;

#[component(HeroProfile<G>)]
pub fn HeroProfile<G: Html>(cx: Scope) -> View<G> {
    view! {cx,

               section (class="bg-profile d-table w-100 bg-primary", style="background: url('assets/images/account/bg.png') center center;") {div (class="container") {
        div (class="row") {
          div (class="col-lg-12") {
            div (class="card public-profile border-0 rounded shadow", style="z-index: 1;") {
              div (class="card-body") {
                div (class="row align-items-center") {
                  div (class="col-lg-2 col-md-3 text-md-start text-center") {
                    img (src="assets/images/client/05.jpg", class="avatar avatar-large rounded-circle shadow d-block mx-auto", alt="") {}
    }


                  div (class="col-lg-10 col-md-9") {
                    div (class="row align-items-end") {
                      div (class="col-md-7 text-md-start text-center mt-4 mt-sm-0") {
                        h3 (class="title mb-0") {"Flash Tech"}

                        small (class="text-muted h6 me-2") {"Web Developer"}

                        ul (class="list-inline mb-0 mt-3") {li (class="list-inline-item me-2") {a (href="javascript:void(0)", class="text-muted", title="Instagram") {i (data-feather="instagram", class="fea icon-sm me-2") {}
    "@flashtech"}
    }

                          li (class="list-inline-item") {a (href="javascript:void(0)", class="text-muted", title="Linkedin") {i (data-feather="linkedin", class="fea icon-sm me-2") {}
    "Flash Tech"}
    }

                        }
    }

                      div (class="col-md-5 text-md-end text-center") {
                        ul (class="list-unstyled social-icon social mb-0 mt-4") {li (class="list-inline-item") {a (href="javascript:void(0)", class="rounded", data-bs-toggle="tooltip", data-bs-placement="bottom", title="Add Friend") {i (class="uil uil-user-plus align-middle") {}
    }
    }

                          li (class="list-inline-item") {a (href="javascript:void(0)", class="rounded", data-bs-toggle="tooltip", data-bs-placement="bottom", title="Messages") {i (class="uil uil-comment align-middle") {}
    }
    }

                          li (class="list-inline-item") {a (href="javascript:void(0)", class="rounded", data-bs-toggle="tooltip", data-bs-placement="bottom", title="Notifications") {i (class="uil uil-bell align-middle") {}
    }
    }

                          li (class="list-inline-item") {a (href="account-setting.html", class="rounded", data-bs-toggle="tooltip", data-bs-placement="bottom", title="Settings") {i (class="uil uil-cog align-middle") {}
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

    }

                }
}
