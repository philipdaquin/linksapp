use sycamore::prelude::*;

#[component(BlockWidget<G>)]
pub fn BlockWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,

    section (class="section overflow-hidden") {div (class="container") {
                    div (class="row justify-content-center") {
                        div (class="col-12 text-center") {
                            div (class="section-title mb-4 pb-2") {

                                h4 (class="title mb-4") {"What you really need!" }

                                p (class="text-muted para-desc mx-auto mb-0") {"A Micro-page from" span (class="text-primary fw-bold") {"LinksApp"}
     "that can provide everything you need to generate awareness, drive traffic, connect and make quick sales."}

                            }

                        }

                    }


                    div (class="row") {
                        div (class="col-md-4 col-12 mt-4 pt-2") {
                            div (class="card text-center rounded border-0") {
                                div (class="card-body") {
                                    div (class="p-3 bg-light rounded shadow d-inline-block") {
                                        img (src="assets/images/icon/art-and-design.svg", class="avatar avatar-small", alt="") {}
    }

                                    div (class="mt-4") {
                                        h5  {a (href="javascript:void(0)", class="text-dark") {"Design & Branding"}
    }

                                        p (class="text-muted mt-3 mb-0") {"Create a unique design and branding matching your pages and products and share to your audience."}

                                    }

                                }

                            }

                        }


                        div (class="col-md-4 col-12 mt-4 pt-2") {
                            div (class="card text-center bg-primary bg-gradient rounded border-0") {
                                div (class="card-body") {
                                    div (class="p-3 bg-light rounded shadow d-inline-block") {
                                        img (src="assets/images/icon/smartphone.svg", class="avatar avatar-small", alt="") {}
    }

                                    div (class="mt-4") {
                                        h5  {a (href="javascript:void(0)", class="text-white title-dark") {"SEO Compliant"}
    }

                                        p (class="text-white-50 mt-3 mb-0") {"LinksApp offers you a fully optimized SEO page which drives traffic instantly to your micro-page."}

                                    }

                                }

                            }

                        }


                        div (class="col-md-4 col-12 mt-4 pt-2") {
                            div (class="card text-center rounded border-0") {
                                div (class="card-body") {
                                    div (class="p-3 bg-light rounded shadow d-inline-block") {
                                        img (src="assets/images/icon/clock.svg", class="avatar avatar-small", alt="") {}
    }

                                    div (class="mt-4") {
                                        h5  {a (href="javascript:void(0)", class="text-dark") {"High Performance"}
    }

                                        p (class="text-muted mt-3 mb-0") {"Your micro-page loads instantly with links you created from other sources."}

                                    }

                                }

                            }

                        }

                    }

                }


                div (class="container mt-100 mt-60") {
                    div (class="row align-items-center") {
                        div (class="col-lg-6 col-md-5") {
                            div (class="app-feature-shape-left position-relative") {
                                div (class="text-center text-md-start") {
                                    img (src="assets/images/app/classic02.png", class="img-fluid", alt="") {}
    }

                            }

                        }


                        div (class="col-lg-6 col-md-7 mt-5 mt-sm-0") {
                            div (class="section-title") {
                                h1 (class="title mb-3") {"We Build High Performing" br  {}
     "Application"}

                                p (class="para-desc text-muted") {"Launch your campaign and benefit from our expertise on designing and managing conversion centered bootstrap v5 html page."}

                                ul (class="list-unstyled text-muted") {li (class="mb-1") {span (class="text-primary h5 me-2") {i (class="uil uil-check-circle align-middle") {}
    }
    "Digital Marketing Solutions for Tomorrow"}

                                    li (class="mb-1") {span (class="text-primary h5 me-2") {i (class="uil uil-check-circle align-middle") {}
    }
    "Our Talented & Experienced Marketing Agency"}

                                    li (class="mb-1") {span (class="text-primary h5 me-2") {i (class="uil uil-check-circle align-middle") {}
    }
    "Create your own skin to match your brand"}

                                }
    div (class="mt-4") {
                                    a (href="javascript:void(0)", class="mt-3 h6 text-primary") {"Find Out More" i (class="uil uil-angle-right-b") {}
    }

                                }

                            }

                        }

                    }

                }


                div (class="container mt-100 mt-60") {
                    div (class="row align-items-center") {
                        div (class="col-lg-6 col-md-7 order-2 order-md-1 mt-5 mt-sm-0") {
                            div (class="section-title") {
                                h1 (class="title mb-3") {"Easy And Best Solution" br  {}
    " For Your App"}

                                p (class="para-desc text-muted") {"Launch your Lorem ipsum dolor, sit amet consectetur adipisicing elit. Eveniet eligendi expedita ducimus fuga sed possimus veritatis eum voluptates. Ab ex odio sed atque. Quam delectus, voluptatibus rem harum nihil minus. campaign and benefit from our expertise on designing and managing conversion centered bootstrap v5 html page."}

                                div (class="mt-4") {
                                    a (href="javascript:void(0)", class="btn btn-primary") {"Learn More" i (class="uil uil-angle-right-b") {}
    }

                                }

                            }

                        }


                        div (class="col-lg-6 col-md-5 order-1 order-md-2") {
                            div (class="app-feature-shape-right position-relative") {
                                div (class="text-center text-md-end") {
                                    img (src="assets/images/app/classic03.png", class="img-fluid", alt="") {}
    }

                            }

                        }

                    }

                }


                div (class="container mt-100 mt-60") {
                    div (class="row align-items-center") {
                        div (class="col-lg-6 col-md-5") {
                            div (class="app-feature-shape-left position-relative") {
                                div (class="text-center text-md-start") {
                                    img (src="assets/images/app/classic04.png", class="img-fluid", alt="") {}
    }

                            }

                        }


                        div (class="col-lg-6 col-md-7 mt-5 mt-sm-0") {
                            div (class="section-title") {
                                h1 (class="title mb-3") {"Beautiful, Simple &" br  {}
     "Easy to Use"}

                                p (class="para-desc text-muted") {"Launch your campaign and benefit from our expertise on designing and managing conversion centered bootstrap v5 html page."}

                                ul (class="list-unstyled text-muted") {li (class="mb-1") {span (class="text-primary h5 me-2") {i (class="uil uil-check-circle align-middle") {}
    }
    "Digital Marketing Solutions for Tomorrow"}

                                    li (class="mb-1") {span (class="text-primary h5 me-2") {i (class="uil uil-check-circle align-middle") {}
    }
    "Our Talented & Experienced Marketing Agency"}

                                    li (class="mb-1") {span (class="text-primary h5 me-2") {i (class="uil uil-check-circle align-middle") {}
    }
    "Create your own skin to match your brand"}

                                }
    div (class="mt-4") {
                                    a (href="javascript:void(0)", class="btn btn-primary") {"Read More" i (class="uil uil-angle-right-b") {}
    }

                                }

                            }

                        }

                    }

                }

            }


        }
}
