use leptos::html::{Div, Video};
use leptos::prelude::*;
use leptos_meta::Style;
use leptos_use::{UseElementVisibilityOptions, UseIntersectionObserverOptions, UseIntersectionObserverReturn, use_element_visibility_with_options, use_intersection_observer, use_intersection_observer_with_options};

// --- Data Structs ---

#[derive(Clone)]
struct ProfileData {
    name: &'static str,
    title: &'static str,
    email: &'static str,
    linkedin: &'static str,
}

#[derive(Clone)]
struct Experience {
    title: &'static str,
    company: &'static str,
    duration: &'static str,
    tasks: &'static [&'static str],
}

#[derive(Clone)]
struct SkillCategory {
    category: &'static str,
    items: &'static [&'static str],
}

// --- Data Definitions ---

const PROFILE_DATA: ProfileData = ProfileData {
    name: "Nicolas dos Santos Almino",
    title: "Produção de Mídia & Web Developer",
    email: "nicolas.almino@hotmail.com",
    linkedin: "https://www.linkedin.com/in/nicolasalmino",
};

const SUMMARY: &str = "Estudante de Ciências da Computação e Técnico em Desenvolvimento de Sistemas com experiência prática em produção de mídia digital (filmagem, edição) e gerenciamento de mídias sociais.";

const EXPERIENCES: [Experience; 2] = [
    Experience {
        title: "Editor de Mídia e Gestor de Comunidade (PJ)",
        company: "Grupo Camará Capoeira",
        duration: "09/2025 - Atual",
        tasks: &[
            "Responsável pela filmagem, produção e edição de vídeos para o Ponto de Cultura Camará.",
            "Gerenciamento estratégico das mídias sociais da associação.",
        ],
    },
    Experience {
        title: "Produtor de Mídia (Estágio)",
        company: "Ecossistema Fronteiras da Inovação",
        duration: "08/2025 - Atual",
        tasks: &[
            "Atuação na cobertura de eventos com filmagem, produção e edição de vídeos.",
            "Gerenciamento de mídias e desenvolvimento/entrega de website para hospedagem de conteúdo.",
        ],
    },
];

const SKILLS: [SkillCategory; 3] = [
    SkillCategory {
        category: "Produção de Mídia & Marketing",
        items: &[
            "Edição de Vídeo",
            "CapCut",
            "Blender (VSE)",
            "OpenShot",
            "Canva",
            "Gerenciamento de Mídias Sociais",
            "WordPress",
            "Elementor",
        ],
    },
    SkillCategory {
        category: "Background Técnico (TI)",
        items: &[
            "HTML5 & CSS3",
            "JavaScript",
            "React",
            "Node.js",
            "Python",
            "Rust",
            "Git",
            "Linux",
            "Docker",
        ],
    },
    SkillCategory {
        category: "Soft Skills",
        items: &[
            "Proatividade",
            "Liderança",
            "Solução de Problemas",
            "Aprendizado Rápido",
            "Adaptabilidade",
            "Gestão de Prazos",
            "Comunicação",
        ],
    },
];

// --- SVG Icon Components ---

#[component]
fn MailIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
            <polyline points="22,6 12,13 2,6"></polyline>
        </svg>
    }
}

#[component]
fn LinkedinIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path>
            <rect x="2" y="9" width="4" height="12"></rect>
            <circle cx="4" cy="4" r="2"></circle>
        </svg>
    }
}

#[component]
fn YoutubeIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2A29 29 0 0 0 12 2.15a29 29 0 0 0-8.6.27 2.78 2.78 0 0 0-1.94 2C2.15 8 2 12 2 12s0 4 .56 5.58a2.78 2.78 0 0 0 1.94 2A29 29 0 0 0 12 21.85a29 29 0 0 0 8.6-.27 2.78 2.78 0 0 0 1.94-2C22.85 16 23 12 23 12s0-4-.46-5.58z"></path>
            <polygon points="10 15 15 12 10 9 10 15"></polygon>
        </svg>
    }
}

#[component]
fn YoutubeArrowIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="youtube-arrow"
        >
            <line x1="7" y1="17" x2="17" y2="7"></line>
            <polyline points="7 7 17 7 17 17"></polyline>
        </svg>
    }
}

// --- Interaction Components ---

#[component]
pub fn ScrollReveal(
    children: Children,
    #[prop(optional, default = 0)] delay_ms: i64,
) -> impl IntoView {
    let el = NodeRef::<Div>::new();
    let (is_intersecting, set_intersecting) = signal(false);
    let first_call = RwSignal::new(true);

    // NEW: Wrap the observer in an effect to run on client only
    Effect::new(move |_| {
        // This code now only runs in the browser
        let UseIntersectionObserverReturn { stop, .. } = use_intersection_observer_with_options(
            el,
            move |entries, _observer| {
                if let Some(entry) = entries.first() {
                    if first_call.get_untracked() { // Use untracked read inside effect
                        first_call.set(false);
                    } else {
                        set_intersecting.set(entry.is_intersecting());
                    }
                }
            },
            UseIntersectionObserverOptions::default().thresholds(vec![0.8]),
        );

        // Stop the observer when the component is unmounted
        on_cleanup(move || stop());
    });

    let has_been_visible = RwSignal::new(false);

    Effect::new(move |_| {
        if is_intersecting.get() {
            has_been_visible.set(true);
        }
    });

    view! {
        <div
            node_ref=el
            class="reveal-container"
            class:visible=move || has_been_visible.get()
            style:transition-delay=move || format!("{}ms", delay_ms)
        >
            {children()}
        </div>
    }
}
// --- Page Section Components ---

#[component]
fn Header(data: ProfileData) -> impl IntoView {
    view! {
        <header style="position: fixed; top: 0; left: 0; right: 0; z-index: 50; background-color: rgba(10, 10, 10, 0.8); backdrop-filter: blur(4px); transition: background-color 0.3s;">
            <nav style="max-width: 1024px; margin: 0 auto; padding: 1.5rem; display: flex; justify-content: space-between; align-items: center;">
                <a
                    href="#"
                    style="font-size: 1.25rem; line-height: 1.75rem; font-weight: 700; color: white; text-decoration: none;"
                    class="header-link"
                >
                    {data.name}
                </a>
                <div style="display: flex; align-items: center; gap: 1.5rem;">
                    <a
                        href=format!("mailto:{}", data.email)
                        style="color: #d1d5db; text-decoration: none;"
                        class="header-link"
                        title="Email"
                    >
                        <MailIcon/>
                    </a>
                    <a
                        href=data.linkedin
                        target="_blank"
                        rel="noopener noreferrer"
                        style="color: #d1d5db; text-decoration: none;"
                        class="header-link"
                        title="LinkedIn"
                    >
                        <LinkedinIcon/>
                    </a>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <div style="text-align: center; position: relative;">
            <h1 style="font-size: 4.5rem; line-height: 1; font-weight: 900; color: white; letter-spacing: -0.025em; margin: 0; padding: 0;">
                "NICOLAS ALMINO"
            </h1>
            <h2 style="font-size: 1.875rem; line-height: 2.25rem; font-weight: 300; color: #93c5fd; margin-top: 1rem;">
                "Produção de Mídia & Web Developer"
            </h2>
            <p style="font-size: 1.125rem; line-height: 1.75rem; color: #9ca3af; max-width: 42rem; margin: 1.5rem auto 0;">
                "Programando ideias e editando sonhos."
            </p>
            <a
                href="#about"
                style="margin-top: 2.5rem; display: inline-block; background-color: #2563eb; color: white; font-weight: 700; padding: 0.75rem 2rem; border-radius: 9999px; font-size: 1.125rem; line-height: 1.75rem; text-decoration: none;"
                class="hero-button"
            >
                "Conheça meu trabalho"
            </a>
        </div>
    }
}

#[component]
fn AboutSection(text: &'static str) -> impl IntoView {
    view! {
        <section id="about" style="width: 100%;">
            <div class="about-grid">
                <div style="display: flex; justify-content: center;">
                    <img
                        src="/assets/nicolas.png"
                        alt="Foto de perfil de Nicolas Almino"
                        style="border-radius: 1rem; width: 100%; max-width: 300px; margin: 0 auto; aspect-ratio: 1/1; object-fit: cover; box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1); border: 2px solid rgba(255, 255, 255, 0.1);"
                    />
                </div>
                <div class="about-text-col">
                    <h3 style="font-size: 2.25rem; line-height: 2.5rem; font-weight: 700; color: white; margin-bottom: 1.5rem;">
                        "Sobre mim"
                    </h3>
                    <p style="font-size: 1.125rem; line-height: 1.75rem; color: #d1d5db; line-height: 1.7;">
                        {text}
                    </p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeaturedProjectSection() -> impl IntoView {
 let video_ref = NodeRef::<Video>::new();
    let (is_visible, set_visible) = signal(false);

   // Effect to play/pause video based on intersection
    Effect::new(move |_| {
        // ✅ MOVE THE OBSERVER CREATION *INSIDE* THE EFFECT
        let UseIntersectionObserverReturn { stop: _stop, .. } = use_intersection_observer_with_options(
            video_ref,
            move |entries, _| {
                // Update the signal based on the intersection state
                set_visible.set(entries[0].is_intersecting());
            },
            UseIntersectionObserverOptions::default()
                .thresholds(vec![0.1]), // Set your threshold
        );
        // --- END FIX ---

        if let Some(video) = video_ref.get() {
            if is_visible.get() {
                let _ = video.play();
            } else {
                let _ = video.pause();
                video.set_current_time(0.0); // Reset video to start
            }
        }
        
        // Also, add the cleanup function to stop the observer
        on_cleanup(move || _stop());
    });
    // Effect to play/pause video based on intersection
    Effect::new(move |_| {
        if let Some(video) = video_ref.get() {
            if is_visible.get() {
                // play() returns a Future in JS, in Leptos we just call it.
                // We ignore the result, as autoplay might be blocked.
                let _ = video.play();
            } else {
                let _ = video.pause();
                video.set_current_time(0.0); // Reset video to start
            }
        }
    });

    view! {
        <section id="featured-project" style="width: 100%; overflow: hidden;">
            <div class="featured-grid">
                <div>
                    <div style="margin-bottom: 1.5rem;">
                        <h4 style="font-family: 'Story Script', cursive; font-size: 3.75rem; line-height: 1; font-weight: 700; color: white; margin-bottom: 0.5rem;">
                            "Grupo Camará Capoeira"
                        </h4>
                        <p style="font-size: 1.5rem; line-height: 2rem; font-weight: 300; color: #d15d22;">
                            "Produção de mídia e eventos"
                        </p>
                    </div>
                    <p style="color: rgba(255, 255, 255, 0.9); font-size: 1.125rem; line-height: 1.75rem; line-height: 1.7; margin-bottom: 1.5rem;">
                        "Meu trabalho aqui é fazer produção de conteúdo relacionados ao Projeto Ponto de Cultura Camará, na organização Camará Capoeira. Envolve a filmagem, edição e divulgação do conteúdo, e posteriormente o impulsionamento pago desses vídeos nas redes sociais."
                    </p>
                    <a
                        href="https://www.youtube.com/@grupocamaracapoeira"
                        target="_blank"
                        rel="noopener noreferrer"
                        style="display: flex; align-items: center; gap: 0.5rem; margin-top: 1rem; transition: color 0.3s; color: #d15d22; text-decoration: none;"
                        class="youtube-link"
                    >
                        <YoutubeIcon/>
                        <span style="font-weight: 500; font-size: 1.125rem; line-height: 1.75rem;">
                            "YouTube"
                        </span>
                        <YoutubeArrowIcon/>
                    </a>
                </div>
                <div style="display: flex; align-items: center; justify-content: center;">
                    <video
                        node_ref=video_ref
                        style="width: 100%; max-width: 24rem; height: 100%; max-height: 600px; aspect-ratio: 9/16; object-fit: cover; border-radius: 0.5rem; box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1); border: 2px solid #d15d22;"
                        loop=true
                        muted=true
                        playsinline=true
                    >
                        <source src="/assets/camaracapoeira_1080p.mp4" type="video/mp4"/>
                        <source src="/assets/camaracapoeira_720p.mp4" type="video/mp4"/>
                        <source src="/assets/camaracapoeira_480p.mp4" type="video/mp4"/>
                        <source src="/assets/camaracapoeira_360p.mp4" type="video/mp4"/>
                        <source src="/assets/camaracapoeira_240p.mp4" type="video/mp4"/>
                        <source src="/assets/camaracapoeira_144p.mp4" type="video/mp4"/>
                        <div style="width: 100%; max-width: 24rem; height: 100%; max-height: 600px; aspect-ratio: 9/16; background-color: #374151; display: flex; align-items: center; justify-content: center;">
                            <p style="color: white;">
                                "Seu navegador não suporta o elemento "
                                <code>video</code>
                                "."
                            </p>
                        </div>
                    </video>
                </div>
            </div>
        </section>
    }
}

#[component]
fn EsportesNaTvSection() -> impl IntoView {
    view! {
        <section
            id="esportes-na-tv"
            style="width: 100%; text-align: center; transition: colors 0.3s;"
        >
            <div style="display: flex; flex-direction: column; align-items: center;">
                <img
                    src="/assets/esportesnatv.png"
                    alt="Logo EsportesNaTV"
                    style="border-radius: 1rem; width: 100%; max-width: 100px; margin: 0 auto; aspect-ratio: 1/1; object-fit: cover; box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1); border: 2px solid rgba(255, 255, 255, 0.1);"
                />
                <h3 style="font-size: 2.25rem; line-height: 2.5rem; font-weight: 700; color: white; margin-top: 2rem; margin-bottom: 1rem;">
                    "EsportesNaTV"
                </h3>
                <p style="font-size: 1.25rem; line-height: 1.75rem; color: #e5e7eb; max-width: 32rem; margin: 0 auto; line-height: 1.7;">
                    "Um guia completo para saber onde assistir seus jogos e eventos esportivos favoritos na televisão."
                </p>
                <a
                    href="https://esportesnatv.com.br"
                    target="_blank"
                    rel="noopener noreferrer"
                    style="margin-top: 2.5rem; display: inline-block; background-color: white; color: #13782a; font-weight: 700; padding: 0.75rem 2rem; border-radius: 9999px; font-size: 1.125rem; line-height: 1.75rem; text-decoration: none;"
                    class="esportes-button"
                >
                    "Visite-nos"
                </a>
            </div>
        </section>
    }
}

#[component]
fn ExperienceSection(data: &'static [Experience]) -> impl IntoView {
    view! {
        <section id="experience" style="width: 100%;">
            <h3 style="font-size: 2.25rem; line-height: 2.5rem; font-weight: 700; color: white; margin-bottom: 3rem; text-align: center;">
                "Experiência"
            </h3>
            <div style="display: flex; flex-direction: column; gap: 2.5rem;">
                {data
                    .iter()
                    .map(|job| {
                        view! {
                            <div class="job-card">
                                <div class="job-header">
                                    <h4 style="font-size: 1.5rem; line-height: 2rem; font-weight: 700; color: white; margin-bottom: 0.25rem;">
                                        {job.title}
                                    </h4>
                                    <span style="font-size: 0.875rem; line-height: 1.25rem; color: #93c5fd; flex-shrink: 0; margin-left: 1rem;">
                                        {job.duration}
                                    </span>
                                </div>
                                <span style="font-size: 1.125rem; line-height: 1.75rem; color: #d1d5db; font-weight: 500;">
                                    {job.company}
                                </span>
                                <ul style="list-style-type: disc; list-style-position: inside; margin-top: 1rem; color: #d1d5db; display: flex; flex-direction: column; gap: 0.25rem; padding-left: 0.5rem;">
                                    {job
                                        .tasks
                                        .iter()
                                        .map(|task| {
                                            view! { <li>{*task}</li> }
                                        })
                                        .collect_view()}
                                </ul>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

#[component]
fn SkillsSection(data: &'static [SkillCategory]) -> impl IntoView {
    view! {
        <section id="skills" style="width: 100%;">
            <h3 style="font-size: 2.25rem; line-height: 2.5rem; font-weight: 700; color: white; margin-bottom: 3rem; text-align: center;">
                "Habilidades"
            </h3>
            <div style="padding: 2rem 3rem; border: 1px solid #374151; border-radius: 0.5rem;">
                <div class="skills-grid">
                    {data
                        .iter()
                        .map(|skill_cat| {
                            view! {
                                <div>
                                    <h4 style="font-size: 1.25rem; line-height: 1.75rem; font-weight: 600; color: white; margin-bottom: 1.25rem; border-bottom: 1px solid #4b5563; padding-bottom: 0.5rem;">
                                        {skill_cat.category}
                                    </h4>
                                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
                                        {skill_cat
                                            .items
                                            .iter()
                                            .map(|item| {
                                                view! {
                                                    <span style="background-color: #374151; color: #bfdbfe; font-size: 0.875rem; line-height: 1.25rem; font-weight: 500; padding: 0.25rem 0.75rem; border-radius: 9999px;">
                                                        {*item}
                                                    </span>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}

// --- Main HomePage Component ---

#[component]
pub fn HomePage() -> impl IntoView {
    // Define background colors
    let default_bg_color = "#0a0a0a";
    let section_bg_color = "#0f172a";
    let featured_bg_color = "#332175";
    let esportes_bg_color = "#13782a";

    view! {
        // Style tag for global styles, fonts, and responsive CSS
        <Style>
            {
                r#"
                @import url('https://fonts.googleapis.com/css2?family=Story+Script&display=swap');
                
                :root {
                    --default-bg: #0a0a0a;
                    --section-bg: #0f172a;
                    --featured-bg: #332175;
                    --esportes-bg: #13782a;
                }

                body {
                    background-color: var(--default-bg);
                    color: white;
                    margin: 0;
                    -webkit-font-smoothing: antialiased;
                    -moz-osx-font-smoothing: grayscale;
                }
                
                html {
                    scroll-behavior: smooth;
                }

                ::selection {
                    background-color: #3b82f6;
                    color: white;
                }

                /* --- Hovers & Transitions --- */
                .header-link:hover {
                    color: white !important;
                }
                
                .hero-button {
                    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                    box-shadow: 0 0 #0000, 0 0 #0000;
                }
                .hero-button:hover {
                    background-color: #3b82f6;
                    box-shadow: 0 10px 15px -3px rgb(59 130 246 / 0.3), 0 4px 6px -4px rgb(59 130 246 / 0.3);
                    transform: scale(1.05);
                }
                
                .esportes-button {
                    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                    box-shadow: 0 0 #0000, 0 0 #0000;
                }
                .esportes-button:hover {
                    background-color: #e5e7eb;
                    box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
                    transform: scale(1.05);
                }

                .youtube-link .youtube-arrow {
                    opacity: 0;
                    transition: opacity 0.3s;
                }
                .youtube-link:hover .youtube-arrow {
                    opacity: 1;
                }
                .youtube-link:hover span {
                    color: white;
                }

                .job-card {
                    padding: 1.5rem; 
                    border: 1px solid #374151; 
                    border-radius: 0.5rem; 
                    transition: all 0.3s;
                }
                .job-card:hover {
                    border-color: rgba(59, 130, 246, 0.5);
                    box-shadow: 0 20px 25px -5px rgb(15 23 42 / 0.2), 0 8px 10px -6px rgb(15 23 42 / 0.2);
                }

                /* --- Responsive Grid Layouts --- */
                .about-grid {
                    display: grid;
                    grid-template-columns: 1fr;
                    gap: 3rem;
                    align-items: center;
                }
                .featured-grid {
                    display: grid;
                    grid-template-columns: 1fr;
                    gap: 3rem;
                    align-items: center;
                }
                .skills-grid {
                    display: grid;
                    grid-template-columns: 1fr;
                    gap: 2.5rem;
                }
                .job-header {
                    display: flex;
                    flex-direction: column;
                }

                @media (min-width: 768px) {
                    h1 {
                        font-size: 5.5rem; /* Larger on desktop */
                    }
                    .about-grid {
                        grid-template-columns: repeat(3, minmax(0, 1fr));
                    }
                    .about-text-col {
                        grid-column: span 2 / span 2;
                    }
                    .featured-grid {
                        grid-template-columns: repeat(2, minmax(0, 1fr));
                    }
                    .skills-grid {
                        grid-template-columns: repeat(3, minmax(0, 1fr));
                    }
                    .job-header {
                        flex-direction: row;
                        justify-content: space-between;
                        align-items: center;
                    }
                }
                "#
            }
        </Style>

        <Header data=PROFILE_DATA.clone()/>

        <main>
            // Camada Base: Hero Section (z-10)
            <section
                style=format!(
                    "min-height: 100vh; display: flex; align-items: center; justify-content: center; position: relative; z-index: 10; background-color: {};",
                    default_bg_color,
                )
            >
                <div style="padding: 1.5rem 2.5rem; max-width: 1024px; margin: 0 auto; width: 100%;">
                    <HeroSection/>
                </div>
            </section>

            // Container para as seções 'sticky'
            <div style="position: relative;">
                // Seção Sobre (z-20)
                <section
                    style=format!(
                        "min-height: 100vh; display: flex; align-items: center; justify-content: center; position: sticky; top: 0; z-index: 20; background-color: {};",
                        section_bg_color,
                    )
                >
                    <div style="padding: 1.5rem 2.5rem; max-width: 1024px; margin: 0 auto; width: 100%;">
                        <ScrollReveal delay_ms=0>
                            <AboutSection text=SUMMARY/>
                        </ScrollReveal>
                    </div>
                </section>

                // Seção Projeto Destaque (z-30)
                <section
                    style=format!(
                        "min-height: 100vh; display: flex; align-items: center; justify-content: center; position: sticky; top: 0; z-index: 30; background-color: {};",
                        featured_bg_color,
                    )
                >
                    <div style="padding: 1.5rem 2.5rem; max-width: 1024px; margin: 0 auto; width: 100%;">
                        <ScrollReveal delay_ms=100>
                            <FeaturedProjectSection/>
                        </ScrollReveal>
                    </div>
                </section>

                // Seção Esportes na TV (z-40)
                <section
                    style=format!(
                        "min-height: 100vh; display: flex; align-items: center; justify-content: center; position: sticky; top: 0; z-index: 40; background-color: {};",
                        esportes_bg_color,
                    )
                >
                    <div style="padding: 1.5rem 2.5rem; max-width: 1024px; margin: 0 auto; width: 100%;">
                        <ScrollReveal delay_ms=150>
                            <EsportesNaTvSection/>
                        </ScrollReveal>
                    </div>
                </section>

                // Seção Experiência (z-45)
                <section
                    style=format!(
                        "min-height: 100vh; display: flex; align-items: center; justify-content: center; position: sticky; top: 0; z-index: 45; background-color: {};",
                        section_bg_color,
                    )
                >
                    <div style="padding: 1.5rem 2.5rem; max-width: 1024px; margin: 0 auto; width: 100%;">
                        <ScrollReveal delay_ms=200>
                            <ExperienceSection data=&EXPERIENCES/>
                        </ScrollReveal>
                    </div>
                </section>

                // Seção Habilidades (z-46)
                <section
                    style=format!(
                        "min-height: 100vh; display: flex; align-items: center; justify-content: center; position: sticky; top: 0; z-index: 46; background-color: {};",
                        default_bg_color,
                    )
                >
                    <div style="padding: 1.5rem 2.5rem; max-width: 1024px; margin: 0 auto; width: 100%;">
                        <ScrollReveal delay_ms=300>
                            <SkillsSection data=&SKILLS/>
                        </ScrollReveal>
                    </div>
                </section>
            </div>
        </main>
    }
}
