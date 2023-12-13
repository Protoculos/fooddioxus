use crate::model::category_card::CategoryCard;
use dioxus::prelude::*;

#[component]
pub fn CategoryCard<'a>(cx: Scope, card: &'a CategoryCard) -> Element {
    render! {
        div { class: "{card.bg_color} flex py-3 rounded-lg overflow-hidden md:flex-1",
            div { class: "basis-1/3 relative",
                img {
                    class: "absolute w-28 -bottom-4 -left-4",
                    src: "{card.img.src}",
                    alt: "{card.img.alt}"
                }
            }
            div {
                div { class: "mb-2",
                    h4 { class: "card__title", "{card.title}" }
                    p { class: "text-xs", "{card.text}" }
                }
                a { class: "text-blackColor cursor-pointer", href: "#", "Buy online" }
            }
        }
    }
}
