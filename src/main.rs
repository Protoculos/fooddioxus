#![allow(non_snake_case)]

mod models;
mod ui;
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use models::food_cards::{BEVERAGE, BURGERS, SNACKS};
use models::review_card::REVIEWERS;
use ui::main::food_card_ui::FoodCard;
use ui::main::reviewers_card_ui::ReviewersCard;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let menu_hidden = use_state(cx, || "hidden".to_string());
    let menu = vec!["Home", "About", "Menu", "Review", "Contact"];
    let tabs = vec!["All", "Food", "Snack", "Beverage"];
    let selected_snippet = use_state(cx, || 0);

    render! {
        // Header ----------------------------------
        header { class: "bg-primaryColor fixed top-0 left-0 w-full z-50",
            nav { class: "container relative h-14 flex justify-between items-center",
                div {
                    a { href: "#", class: "text-2xl uppercase font-oswald",
                        "Bur"
                        span { class: "text-2xl uppercase text-secondaryColor", "ger" }
                    }
                }

                div { class: "{menu_hidden} absolute top-0 left-0 w-full py-14 bg-primaryColor border-b border-secondaryColor md:block md:static md:py-0 md:border-none md:w-auto md:ml-auto",
                    ul { class: "flex flex-col text-center gap-5 md:flex-row",
                        for item in menu {
                            li { onclick: move |_| { menu_hidden.set("hidden".to_string()) },
                                a {
                                    class: " hover:text-secondaryColor ease-in duration-200",
                                    href: "#{item.to_lowercase()}",
                                    "{item}"
                                }
                            }
                        }
                    }
                    div {
                        class: "absolute top-[0.7rem] right-4 cursor-pointer md:hidden",
                        onclick: move |_| { menu_hidden.set("hidden".to_string()) },
                        svg {
                            class: "h-8 w-8 fill-current text-white",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            path { d: "m12 10.586 4.95-4.95 1.415 1.415-4.95 4.95 4.95 4.95-1.415 1.414-4.95-4.95-4.95 4.95-1.413-1.415 4.95-4.95-4.95-4.95L7.05 5.638l4.95 4.95Z" }
                        }
                    }
                }
                div { class: "flex items-center gap-5",
                    svg {
                        class: "cursor-pointer ml-4 h-6 w-6 fill-current text-white",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        path { d: "M10 7a7 7 0 0 0 12 4.9v.1c0 5.523-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2h.1A6.98 6.98 0 0 0 10 7Zm-6 5a8 8 0 0 0 15.062 3.762A9 9 0 0 1 8.238 4.938 7.999 7.999 0 0 0 4 12Z" }
                    }
                    div { onclick: move |_| { menu_hidden.set("".to_string()) },
                        svg {
                            class: "cursor-pointer ml-4 h-6 w-6 fill-current text-white md:hidden",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            path { d: "M3 4h18v2H3V4Zm0 7h12v2H3v-2Zm0 7h18v2H3v-2Z" }
                        }
                    }
                }
            }
        }

        main {
            // Home ----------------------------------------------
            section { id: "home",
                div { class: "container flex flex-col items-center gap-10 md:flex-row",
                    div { class: "mx-auto md:basis-1/2 lg:basis-2/5 animate-movingY", img {
                        class: "w-60 md:w-full",
                        src: "images/home-image.png",
                        alt: "home image"
                    } }
                    div { class: "text-center md:basis-1/2 md:text-start lg:basis-3/5",
                        h1 { class: "home__title", "HAPPY TUMMY WITH TASTY BURGER." }
                        div { class: "separator mx-auto md:mx-0" }
                        p { class: "paragraph",
                            "The ultimate destination for burger fans who want to indulge in mouth-watering and satisfying burgers. We use only fresh and quality ingredients to make our burgers, and we offer a variety of options to suit your taste. Come and visit us today, or order online and get a free drink. You will love our burgers."
                        }
                        div { class: "text-base flex items-center justify-center gap-4 py-10 md:justify-start md:gap-20",
                            div { class: "text-center flex flex-col items-center justify-center",
                                svg {
                                    class: "fa-solid h-10 w-10 fill-current text-secondaryColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 448 512",
                                    path { d: "M416 0c-16 0-128 32-128 176v112c0 35.3 28.7 64 64 64h32v128c0 17.7 14.3 32 32 32s32-14.3 32-32V32c0-17.7-14.3-32-32-32zM64 16C64 7.8 57.9 1 49.7.1S34.2 4.6 32.4 12.5L2.1 148.8C.7 155.1 0 161.5 0 167.9c0 45.9 35.1 83.6 80 87.7V480c0 17.7 14.3 32 32 32s32-14.3 32-32V255.6c44.9-4.1 80-41.8 80-87.7 0-6.4-.7-12.8-2.1-19.1L191.6 12.5c-1.8-8-9.3-13.3-17.4-12.4S160 7.8 160 16v134.2c0 5.4-4.4 9.8-9.8 9.8-5.1 0-9.3-3.9-9.8-9L127.9 14.6C127.2 6.3 120.3 0 112 0S96.8 6.3 96.1 14.6L83.7 151c-.5 5.1-4.7 9-9.8 9-5.4 0-9.8-4.4-9.8-9.8V16zm48.3 152h-.6l.3-.7.3.7z" }
                                }
                                br {}
                                "Delicious"
                            }
                            div { class: "text-center flex flex-col items-center justify-center",
                                svg {
                                    class: "fa-solid h-10 w-10 fill-current text-secondaryColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 384 512",
                                    path { d: "M192 512C86 512 0 426 0 320 0 228.8 130.2 57.7 166.6 11.7c6-7.5 14.9-11.7 24.5-11.7h1.8c9.6 0 18.5 4.2 24.5 11.7C253.8 57.7 384 228.8 384 320c0 106-86 192-192 192zM96 336c0-8.8-7.2-16-16-16s-16 7.2-16 16c0 61.9 50.1 112 112 112 8.8 0 16-7.2 16-16s-7.2-16-16-16c-44.2 0-80-35.8-80-80z" }
                                }
                                br {}
                                "Fresh"
                            }
                            div { class: "text-center flex flex-col items-center justify-center",
                                svg {
                                    class: "fa-solid h-10 w-10 fill-current text-secondaryColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 448 512",
                                    path { d: "M0 32c477.6 0 366.6 317.3 367.1 366.3L448 480h-26l-70.4-71.2c-39 4.2-124.4 34.5-214.4-37C47 300.3 52 214.7 0 32zm79.7 46c-49.7-23.5-5.2 9.2-5.2 9.2 45.2 31.2 66 73.7 90.2 119.9 31.5 60.2 79 139.7 144.2 167.7 65 28 34.2 12.5 6-8.5-28.2-21.2-68.2-87-91-130.2-31.7-60-61-118.6-144.2-158.1z" }
                                }
                                br {}
                                "Organic"
                            }
                        }
                        a { class: "btn btn-primary", href: "#", "lern more" }
                    }
                }
            }
            // Category --------------------------------------------
            section { id: "category",
                div { class: "container flex flex-col gap-5 md:flex-row",
                    // card 1
                    div { class: "bg-secondaryColor flex py-3 rounded-lg overflow-hidden md:flex-1",
                        div { class: "basis-1/3 relative",
                            img {
                                class: "absolute w-28 -bottom-4 -left-4",
                                src: "images/burger-1.png",
                                alt: "category image"
                            }
                        }
                        div {
                            div { class: "mb-2",
                                h4 { class: "card__title", "Food" }
                                p { class: "text-xs", "Lorem ipsum dolor sit" }
                            }
                            a { class: "text-blackColor cursor-pointer", href: "#", "Buy online" }
                        }
                    }
                    // card 2
                    div { class: "bg-redColor flex py-3 rounded-lg overflow-hidden md:flex-1",
                        div { class: "basis-1/3 relative",
                            img {
                                class: "absolute w-28 -bottom-4 -left-4",
                                src: "images/snack-1.png",
                                alt: "category image"
                            }
                        }
                        div {
                            div { class: "mb-2",
                                h4 { class: "card__title", "Food" }
                                p { class: "text-xs", "Lorem ipsum dolor sit" }
                            }
                            a { class: "text-secondaryColor cursor-pointer", href: "#", "Buy online" }
                        }
                    }
                    // card 3
                    div { class: "bg-greenColor flex py-3 rounded-lg overflow-hidden md:flex-1",
                        div { class: "basis-1/3 relative",
                            img {
                                class: "absolute w-28 -bottom-4 -left-4",
                                src: "images/beverage-2.png",
                                alt: "category image"
                            }
                        }
                        div {
                            div { class: "mb-2",
                                h4 { class: "card__title", "Food" }
                                p { class: "text-xs", "Lorem ipsum dolor sit" }
                            }
                            a { class: "text-secondaryColor cursor-pointer", href: "#", "Buy online" }
                        }
                    }
                }
            }
            // Promo
            section { id: "promo",
                div { class: "container flex flex-col gap-5 lg:gap-10 lg:flex-row",
                    // card 1
                    div { class: "bg-primaryColorLight flex flex-col p-5 rounded-lg md:flex-row md:items-center lg:flex-row-reverse lg:flex-1",
                        img {
                            class: "w-40 mx-auto hover:animate-movingY md:mx-5",
                            src: "images/promo-1.png",
                            alt: "promo image"
                        }
                        div { class: "space-y-2 pt-5 md:pt-0",
                            p { class: "text-xs text-secondaryColor", "Payday promo" }
                            h3 { class: "card__title", "GET A 10% DISCOUNT ON PAYDAY WEEK" }
                            p { class: "paragraph",
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                            }
                            a { class: "text-xs text-secondaryColor", href: "#", "Buy online" }
                        }
                    }
                    // card 2
                    div { class: "bg-primaryColorLight flex flex-col p-5 rounded-lg md:flex-row md:items-center lg:flex-row-reverse lg:flex-1",
                        img {
                            class: "w-40 mx-auto hover:animate-movingY  md:mx-5",
                            src: "images/promo-2.png",
                            alt: "promo image"
                        }
                        div { class: "space-y-2 pt-5 md:pt-0",
                            p { class: "text-xs text-secondaryColor", "Payday promo" }
                            h3 { class: "card__title", "GET A 10% DISCOUNT ON PAYDAY WEEK" }
                            p { class: "paragraph",
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                            }
                            a { class: "text-xs text-secondaryColor", href: "#", "Buy online" }
                        }
                    }
                }
            }
            // About
            section { id: "about",
                div { class: "container flex flex-col gap-10 md:flex-row",
                    div { class: "flex-1", img {
                        class: "rounded-lg",
                        src: "images/about.jpg",
                        alt: "about image"
                    } }
                    div { class: "flex-1",
                        h2 { class: "section__title",
                            "FIND FOOD AND DRINKS, ALL-IN-ONE PLACE FOR YOUR BEST TASTE."
                        }
                        div { class: "separator" }
                        p { class: "paragraph",
                            "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes."
                        }
                        ul { class: "grid grid-cols-2 py-5 space-y-1",
                            li { class: "flex flex-row items-center gap-1 text-xs text-paragraphColor",
                                svg {
                                    class: "h-4 w-4 fill-current text-secondaryColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 448 512",
                                    path { d: "M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z" }
                                }
                                p { "Best Price" }
                            }
                            li { class: "flex flex-row items-center gap-1 text-xs text-paragraphColor",
                                svg {
                                    class: "h-4 w-4 fill-current text-secondaryColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 448 512",
                                    path { d: "M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z" }
                                }
                                "Fresh Ingredient"
                            }
                            li { class: "flex flex-row items-center gap-1 text-xs text-paragraphColor",
                                svg {
                                    class: "h-4 w-4 fill-current text-secondaryColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 448 512",
                                    path { d: "M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z" }
                                }
                                "Best Service"
                            }
                            li { class: "flex flex-row items-center gap-1 text-xs text-paragraphColor",
                                svg {
                                    class: "h-4 w-4 fill-current text-secondaryColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 448 512",
                                    path { d: "M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z" }
                                }
                                "Health Protocol"
                            }
                        }
                        a { class: "btn btn-primary", href: "", "About us" }
                    }
                }
            }
            // Menu
            section { id: "menu",
                div { class: "container",
                    div { class: "max-w-md mx-auto text-center",
                        h2 { class: "section__title", "OUR BEST MENU" }
                        div { class: "separator mx-auto" }
                        p { class: "paragraph",
                            "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa."
                        }
                        div { class: "tabs_wrap",
                            ul { class: "flex flex-wrap justify-center gap-3 py-10",
                                tabs.iter().enumerate().map(|(id, _)| {
                                    let selected = **selected_snippet == id;

                                    let bg_selected = match selected {
                                        true => "btn bg-secondaryColorLight active",
                                        false => "btn bg-primaryColorLight",
                                    };
                                    render! {
                                        li { class: "{bg_selected}",
                                        onclick: move |_| selected_snippet.set(id),
                                        "{tabs[id]}",
                                    }

                                    }
                                })
                            }
                        }
                    }
                    div { class: "menu__items",
                        ul { class: "grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-4 lg:gap-12",
                            match *selected_snippet.get() {
                        1 => {
                            render!{
                                BURGERS.iter().enumerate().map(|(_, card)| {
                                    render!{
                                        FoodCard  {
                                            card: card
                                        }
                                    }
                                })
                                }
                        },
                        2 => {
                            render!{
                                SNACKS.iter().enumerate().map(|(_, card)| {
                                    render!{
                                        FoodCard  {
                                            card: card
                                        }
                                    }
                                })
                                }
                        },
                        3 => {
                            render!{
                                BEVERAGE.iter().enumerate().map(|(_, card)| {
                                    render!{
                                        FoodCard  {
                                            card: card
                                        }
                                    }
                                })
                                }
                        },
                        _ => {
                            render!{
                                BURGERS.iter().enumerate().map(|(_, card)| {
                                    render!{
                                        FoodCard  {
                                            card: card
                                        }
                                    }
                                })
                                SNACKS.iter().enumerate().map(|(_, card)| {
                                    render!{
                                        FoodCard  {
                                            card: card
                                        }
                                    }
                                })
                                BEVERAGE.iter().enumerate().map(|(_, card)| {
                                    render!{
                                        FoodCard  {
                                            card: card
                                        }
                                    }
                                })
                                }
                        }
                    }
                        }
                    }
                }
            }
            // Review
            section { id: "review", class: "bg-primaryColorLight py-20",
                div { class: "container",
                    div { class: "max-w-md mx-auto text-center",
                        h2 { class: "section__title", "CUSTOMER REVIEW" }
                        div { class: "separator mx-auto" }
                        p { class: "paragraph",
                            "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa."
                        }
                    }
                    div { class: "swiper py-10",
                        ul { class: "swiper-wrapper",
                            render!{
                                REVIEWERS.iter().enumerate().map(|(_, card)| {
                            render!{
                                ReviewersCard  {
                                    card: card
                                }
                            }
                        })
                        }
                        }
                    }
                }
            }
        }
    }
}
