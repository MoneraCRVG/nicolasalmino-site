pub mod navbar;

use crate::components::header::client::navbar::Navbar;
use crate::components::stacks::vstack::{AlignItems, VStack};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_meta::Style;

/// Mobile-first responsive header component
#[component]
pub fn Header(
    #[prop(optional, into)]
    class: TextProp,
) -> impl IntoView {
    view! {
        <Style>
            "
            /* Mobile-first base styles */
            .site-header {
                padding: 0.75rem 1rem;
                background-color: var(--background-color, #fff);
                transition: background-color 0.3s ease, box-shadow 0.3s ease;
                display: flex;
                justify-content: center;
            }

            .site-header-content {
                width: 100%;
                max-width: 1200px;
            }

            .site-header img {
                height: 50px;
                width: auto;
                display: block;
            }

            /* Mobile-optimized navigation */
            .navbar {
                flex-wrap: wrap;
                justify-content: center;
                gap: 0.5rem;
            }

            .navbar-link {
                padding: 0.5rem 0.75rem;
                font-size: 0.8rem;
                font-weight: 600;
                color: #374151;
                text-decoration: none;
                border-radius: 8px;
                background-color: rgba(255, 255, 255, 0.4);
                border: 1px solid rgba(255, 255, 255, 0.5);
                box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
                -webkit-backdrop-filter: blur(4px);
                backdrop-filter: blur(4px);
                transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
                white-space: nowrap;
                display: flex;
                align-items: center;
                min-height: 44px; /* Mobile touch target */
            }

            /* Tablet improvements */
            @media (min-width: 768px) {
                .site-header {
                    padding: 1rem 1.5rem;
                }
                
                .site-header img {
                    height: 60px;
                }
                
                .navbar-link {
                    font-size: 0.875rem;
                    padding: 0.5rem 1rem;
                }
            }

            /* Desktop enhancements */
            @media (min-width: 1024px) {
                .site-header .site-header-content {
                    flex-direction: row;
                    justify-content: space-between;
                    align-items: center;
                }
                
                .navbar {
                    gap: 0.75rem;
                }
            }
            "
        </Style>

        <header class=format!("site-header {}", class.get())>
            <VStack
                class="site-header-content"
                align=AlignItems::Center
                spacing="0.75rem".to_string()
            >
                <a href="/" aria-label="Home page">
                    <img src="/assets/logo.jpeg" alt="Logotipo da Agência Fronteiras da Inovação"/>
                </a>
                <Navbar/>
            </VStack>
        </header>
    }
}
