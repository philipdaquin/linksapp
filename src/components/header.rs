use sycamore::prelude::*;

#[component(HeaderWidget<G>)]
pub fn HeaderWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
                header (
                    id="topnav", class="defaultscroll sticky") {div (class="container") {
                        a (class="logo", href="index.html") {
           span (class="logo-light-mode") {
             img (src="assets/images/logo-dark.png", class="l-dark", height="24", alt="") {}
               img (src="assets/images/logo-light.png", class="l-light", height="24", alt="") {}
     }
           img (src="assets/images/logo-light.png", height="24", class="logo-dark-mode", alt="") {}
     }


         div (class="menu-extras") {
           div (class="menu-item") {

             a (class="navbar-toggle", id="isToggle", onclick="toggleMenu()") {
               div (class="lines") {
                 span  {}

                 span  {}

                 span  {}

               }      }       }        }

         ul (class="buy-button list-inline mb-0") {li (class="list-inline-item ps-1 mb-0") {
             a (href="linksapp.test", target="_blank") {
                         }
     a (href="#", class="btn btn-outline-light mt-2 me-2") { "Login" }

           }

           li (class="list-inline-item ps-1 mb-0") {
             a (href="linksapp.test", target="_blank") {
                         }
     a (href="#", class="btn btn-pills btn-primary") { "Register" }
           }

         }
     div (id="navigation") {

           ul (class="navigation-menu nav-light") {li  {a (href="index.html", class="sub-menu-item") {"Home"}
     }

             li (class="has-submenu parent-parent-menu-item") {
               a (href="index-social-integration") {"Start Linking"}
    span  {}         }

                     li (class="has-submenu parent-parent-menu-item") {
               a (href="javascript:void(0)") {"Linkology"}
     span  {}             }

             li (class="has-submenu parent-parent-menu-item") {
               a (href="javascript:void(0)") {"Latest"}
     span  {}       }          }
     }
       }

     }

                 }
}
