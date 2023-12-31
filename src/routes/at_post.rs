use ibex::prelude::*;

use crate::posts::{PostRef, Special};
use crate::views::{post_title, post_transcript, use_base};

pub fn at_post(post_ref: PostRef) -> Document {
    let post = post_ref.get();

    document! { [lang="en"] @use_base [
        &format!("{} [{}]", post.title, post.index),
        Some(&format!("static/posts/{}/esperanto.png", post.index)),
        post_ref.list(),
    ] {
        h2 { @post_title[&post_ref, false] }

        p ."details" {
            span ."navigate prev" {
                [:if let Some(prev) = &post_ref.prev() {
                    HEAD { script { [format!("register_navigation_key('{}', '{}', '{}')", url!(), "p", prev.index())] }}
                    a [href=url!(&prev.index())] { &laquo }
                }]
            }
            ~
            span ."text" {
                "[" span { [&post.index()] } "]"
                ~
                a [
                    href=format!("https://gocomics.com/garfield/{}", post.date.replace('-', "/")),
                    title="Spekti je GoComics.com",
                ] {
                    b { [&post.date] }
                }
            }
            ~
            span ."navigate next" {
                [:if let Some(next) = &post_ref.next() {
                    HEAD { script { [format!("register_navigation_key('{}', '{}', '{}')", url!(), "n", next.index())] }}
                    a [href=url!(&next.index())] { &raquo }
                }]
            }
        }

        div {
            img ."comic" [
                alt="Esperanto bildstrio",
                src=url!(format!("static/posts/{}/esperanto.png", &post.index)),
                height=400,
            ]/
            img ."comic" [
                alt="Angla bildstrio",
                src=url!(format!("static/posts/{}/english.png", &post.index)),
                height=400,
            ]/
        }

        div ."small gray" {
            [:if post.version > 0 {
                b { "Revizio:" }
                ~ [post.version]
            }]
            ~
            [:if post.is_old {
                "(olda)"
            }]
        }

        div ."navigate" {
            [:if let Some(prev) = &post_ref.prev_ref() {
                div ."prev" {
                    a [href=url!(&prev.get().index())] {
                        strong { "Antaŭa:" } ~
                        @post_title [prev, true]
                    }
                }
            }]
            [:if let Some(next) = &post_ref.next_ref() {
                div ."next" {
                    a [href=url!(&next.get().index())] {
                        strong { "Sekva:" } ~
                        @post_title [&next, true]
                    }
                }
            }]
        }

        div ."special" {
            [match &post.special {
                Some(Special::Halloween) => view! { "Feliĉan Halovenon!" },
                Some(Special::Christmas) => view! { "Feliĉan Kristnaskon!" },
                Some(Special::NewYears) => view! { "Feliĉan Novjaron!" },
                _ => view! {},
            }]
        }

        [:if let Some(transcript) = &post.transcript {
            @post_transcript [transcript]
        }]

        hr/

        div ."caption" {
            HEAD { script { [include_str!("../js/copy.js")] } }
            pre [onclick="copy(this)"] {
                [&post.title] ~ "💚" "&#10;&#10;"
                 "#esperanto #garfield #mondodakomiksoj"
                ~ "[" [&post.index()] "]"
            }
        }

        a ."source" [
            href=format!("https://github.com/darccyy/garfeo/tree/master/static/posts/{}", post.index),
        ] {
            "Vidu fonton"
        }
    }}
}
