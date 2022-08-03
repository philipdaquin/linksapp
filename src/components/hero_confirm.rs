use sycamore::prelude::*;

#[component(HeroConfirm<G>)]
pub fn HeroConfirm<G: Html>(cx: Scope) -> View<G> {
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
                h4 (class="card-title text-center") {"Mylink: Confirmation"}


              }

            }

          }

        }

      }

    }

            }
}
