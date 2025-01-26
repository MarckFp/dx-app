#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

use dioxus::prelude::*;
// Components
use views::{Blog, Home};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/icons/favicon.ico") }
        document::Link {
            rel: "icon",
            href: asset!("/assets/icons/favicon-32x32.png"),
            sizes: "32x32",
        }
        document::Link {
            rel: "icon",
            href: asset!("/assets/icons/favicon-16x16.png"),
            sizes: "16x16",
        }
        document::Link {
            rel: "apple-touch-icon",
            href: asset!("/assets/icons/apple-touch-icon.png"),
            sizes: "180x180",
        }
        document::Link { rel: "manifest", href: asset!("/assets/manifest.json") }
        document::Stylesheet { href: asset!("/assets/styling/main.css") }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: "https://cdn.jsdelivr.net/npm/flowbite@2.5.2/dist/flowbite.min.css" }

        Router::<Route> {}
    }
}
