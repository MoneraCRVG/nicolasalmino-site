// File: /home/user/RustroverProjects/fronteirasdainovacao/src/components/header/navbar.rs
use crate::components::stacks::hstack::{AlignItems, HStack};
use leptos::either::Either;
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct DropdownItem {
    text: String,
    href: String,
}

#[component]
fn DropdownMenu(
    title: String,
    items: Vec<DropdownItem>,
    #[prop(optional, into)] redirect_href: Option<String>,
) -> impl IntoView {
    let is_open = RwSignal::new(false);
    let menu_id = format!("{}-dropdown", title.to_lowercase().replace(' ', "-"));
    let items = StoredValue::new(items);

    let title_element = if let Some(href) = redirect_href {
        Either::Left(view! {
            <a
                href=href
                class="navbar-link"
                aria-haspopup="true"
                aria-expanded=move || is_open.get().to_string()
                aria-controls=menu_id.clone()
                on:mouseover=move |_| is_open.set(true)
            >
                {title.clone()}
            </a>
        })
    } else {
        Either::Right(view! {
            <button
                type="button"
                class="navbar-link"
                aria-haspopup="true"
                aria-expanded=move || is_open.get().to_string()
                aria-controls=menu_id.clone()
                on:click=move |_| is_open.update(|v| *v = !*v)
                on:mouseover=move |_| is_open.set(true)
            >
                {title.clone()}
            </button>
        })
    };

    view! {
        <div class="navbar-dropdown-container" on:mouseleave=move |_| is_open.set(false)>
            {title_element}
            // FIX: Replaced .get_untracked() with .get() to make the visibility reactive.
            <Show when=move || is_open.get()>
                <div id=menu_id.clone() class="navbar-dropdown" role="menu">
                    <For
                        each=move || items.with_value(|items| items.clone())
                        key=|item| item.href.clone()
                        children=move |item| {
                            view! {
                                <a
                                    href=item.href
                                    class="navbar-dropdown-item"
                                    role="menuitem"
                                    on:click=move |_| is_open.set(false)
                                >
                                    {item.text}
                                </a>
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let institucional_items = vec![
        DropdownItem {
            text: "NOSSA HISTÓRIA".to_string(),
            href: "/nossa-historia".to_string(),
        },
        DropdownItem {
            text: "TERRITÓRIOS DE ATUAÇÃO".to_string(),
            href: "/territorios-atuacao".to_string(),
        },
        DropdownItem {
            text: "QUEM SOMOS".to_string(),
            href: "/quem-somos".to_string(),
        },
    ];

    let agenda_items = vec![
        DropdownItem {
            text: "CALENDÁRIO DE EVENTOS".to_string(),
            href: "/calendario-eventos".to_string(),
        },
    ];

    view! {
        <nav role="navigation" aria-label="Menu principal">
            <HStack class="navbar" align=AlignItems::Center spacing="0.5rem".to_string()>
                <a href="/blog" class="navbar-link">
                    "BLOG"
                </a>
                <a href="/a-igr" class="navbar-link">
                    "A IGR"
                </a>
                <DropdownMenu title="INSTITUCIONAL".to_string() items=institucional_items/>
                <a href="/projetos" class="navbar-link">
                    "PROJETOS"
                </a>
                <DropdownMenu
                    title="AGENDA".to_string()
                    items=agenda_items
                    redirect_href="/agenda"
                />
                <a href="/midia" class="navbar-link">
                    "MÍDIA"
                </a>
            </HStack>
        </nav>
    }
}

