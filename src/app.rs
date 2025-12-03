use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title, Body, Html};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

use crate::routes::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // FIX: Added {..} to spread attributes, fixing EmptyPropsBuilder error
        <Html {..} lang="pt-BR" class="scroll-smooth"/>
        <Body {..} class="bg-[#0a0a0a] text-white selection:bg-blue-500 selection:text-white"/>
        
        // Sets the document title
        <Title text="Nicolas Almino - Fullstack & Audiovisual"/>

        // Inject global stylesheet and fonts
        // Usando Science Gothic como solicitado para um visual Tech/Brutalista moderno
        <Stylesheet id="font-science-gothic" href="https://fonts.googleapis.com/css2?family=Science+Gothic:wght@100..900&display=swap"/>
        
        // Fontes de ícones (Phosphor Icons ou similar via CDN para logos leves)
        // Usaremos SVGs inline no home.rs para performance máxima, mas deixo a opção aqui se expandir.
        
        <Stylesheet id="leptos" href="/pkg/nicolasalmino-site.css"/>

        <Router>
            <main class="w-full min-h-screen overflow-x-hidden">
                <Routes fallback=move || view! { <div class="h-screen w-full flex items-center justify-center"><h1>"404 - Not Found"</h1></div> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="h-screen w-full flex flex-col items-center justify-center bg-[#0a0a0a] text-white">
            <h1 class="text-6xl font-black mb-4">"404"</h1>
            <p class="text-xl text-gray-400">"Página não encontrada."</p>
            <a href="/" class="mt-8 px-6 py-3 bg-blue-600 rounded-full hover:bg-blue-500 transition-all">
                "Voltar ao Início"
            </a>
        </div>
    }
}