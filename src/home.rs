use comrak::Options;
use maud::{html, Markup, PreEscaped};

const ABOUT_ME: &str = include_str!("../assets/aboutme.md");

pub fn homepage() -> Markup {
    let about_me = comrak::markdown_to_html(ABOUT_ME, &Options::default());
    html! {
        div class="content" {
            section {(PreEscaped(about_me)) }
            (socials())
            (specs())
        }
    }
}

fn socials() -> Markup {
    let table = html! {
        table {
            tbody {
                tr {
                    td { "discord:" }
                    td { a href="https://discord.com/users/519526987090362403" { "biggerben" }}
                }
            }
        }
    };

    html! {
        section {
            fieldset {
                legend { "socials:" }
                (table)
            }
        }
    }
}

fn specs() -> Markup {
    html! {
        section {
            fieldset {
                legend { "specs:" }
                ul {
                    li {
                        details {
                            summary { "mac book (primary)" }
                            table {
                                tr {
                                    td {"cpu"}
                                    td{"M1 Pro (3.2GHz)"}
                                }
                                tr {
                                    td { "ram" }
                                    td { "16GiB" }
                                }
                                tr {
                                    td { "os" }
                                    td { "macOS Sequoia" }
                                }
                            }
                        }
                    }
                    li {
                        details {
                            summary { "linux laptop" }
                            table {
                                tr {
                                    td { "model" }
                                    td { "HP compaq 6910p" }
                                }
                                tr {
                                    td { "cpu" }
                                    td { "Intel Core 2 Duo" }
                                }
                                tr {
                                    td {"ram"}
                                    td{"4GiB DDR3"}
                                }
                                tr {
                                    td{"os"}
                                    td {"Void"}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
