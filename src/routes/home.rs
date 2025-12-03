use crate::components::stacks::hstack::{HStack, AlignItems as HAlign};
use crate::components::stacks::vstack::{VStack, AlignItems as VAlign};
use leptos::html::{Div, Video, Section};
use leptos::prelude::*;
use leptos::ev::MouseEvent;
use leptos_meta::Style;
use leptos_use::{
    use_intersection_observer_with_options,
    UseIntersectionObserverOptions,
    UseIntersectionObserverReturn,
};

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
struct SkillItem {
    name: &'static str,
    // Changed to icon_path for SVG placeholders
    icon_path: &'static str, 
}

#[derive(Clone)]
struct SkillCategory {
    category: &'static str,
    items: &'static [SkillItem],
}

// --- Data Definitions ---

const PROFILE_DATA: ProfileData = ProfileData {
    name: "NICOLAS ALMINO",
    title: "FULLSTACK & AUDIOVISUAL",
    email: "nicolas.almino@hotmail.com",
    linkedin: "https://www.linkedin.com/in/nicolasalmino",
};

// Refactored Skills Data with Image Placeholders
const SKILLS: [SkillCategory; 3] = [
    SkillCategory {
        category: "Engenharia de Software",
        items: &[
            SkillItem { name: "Rust (Leptos/Wasm)", icon_path: "rust_logo" },
            SkillItem { name: "React / Node.js", icon_path: "react_logo" },
            SkillItem { name: "Python / Flask", icon_path: "python_logo" },
            SkillItem { name: "Linux / VPS", icon_path: "linux_logo" },
            SkillItem { name: "Docker / CI/CD", icon_path: "docker_logo" },
        ],
    },
    SkillCategory {
        category: "Audiovisual & Design",
        items: &[
            SkillItem { name: "Blender (VSE/3D)", icon_path: "blender" },
            SkillItem { name: "CapCut Pro", icon_path: "capcut" },
            SkillItem { name: "UI/UX Design", icon_path: "figma" },
            SkillItem { name: "DaVinci Resolve", icon_path: "davinci" },
        ],
    },
    SkillCategory {
        category: "Estratégia",
        items: &[
            SkillItem { name: "Gestão de Projetos", icon_path: "jira" },
            SkillItem { name: "Comunicação", icon_path: "communication" },
            SkillItem { name: "Inglês Técnico", icon_path: "english" },
        ],
    },
];

// --- Components ---

#[component]
fn GearIcon(#[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.09a2 2 0 0 1-1-1.74v-.51a2 2 0 0 1 1-1.72l.15-.1a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
            <circle cx="12" cy="12" r="3"/>
        </svg>
    }
}

/// A self-contained card that tracks mouse movement relative to itself.
/// Solves the "uniform spotlight" issue by calculating coordinates locally.
#[component]
fn SpotlightCard(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    let div_ref = NodeRef::<Div>::new();
    let (mouse_pos, set_mouse_pos) = signal((0.0, 0.0));
    let (opacity, set_opacity) = signal(0.0);

    let handle_mousemove = move |ev: MouseEvent| {
        if let Some(div) = div_ref.get() {
            let rect = div.get_bounding_client_rect();
            let x = ev.client_x() as f64 - rect.left();
            let y = ev.client_y() as f64 - rect.top();
            set_mouse_pos.set((x, y));
            set_opacity.set(1.0);
        }
    };

    let handle_mouseleave = move |_| {
        set_opacity.set(0.0);
    };

    view! {
        <div
            node_ref=div_ref
            class=format!("relative overflow-hidden rounded-3xl bg-[#1a1a1a]/80 border border-white/10 p-6 transition-all duration-300 hover:border-white/20 group {}", class)
            on:mousemove=handle_mousemove
            on:mouseleave=handle_mouseleave
        >
            // Local Spotlight Overlay
            <div 
                class="pointer-events-none absolute inset-0 transition-opacity duration-300"
                style:opacity=move || opacity.get().to_string()
                style:background=move || format!(
                    "radial-gradient(600px circle at {}px {}px, rgba(255, 255, 255, 0.06), transparent 40%)", 
                    mouse_pos.get().0, 
                    mouse_pos.get().1
                )
            ></div>
            
            <div class="relative z-10 h-full">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn Hero() -> impl IntoView {
    // Hero-specific mouse tracking for the Grid Spotlight
    // Changed Div to Section to match the <section> element below
    let container_ref = NodeRef::<Section>::new();
    let (mouse_pos, set_mouse_pos) = signal((0.0, 0.0));
    
    // We update this signal to move the spotlight on the grid
    let handle_mousemove = move |ev: MouseEvent| {
        if let Some(el) = container_ref.get() {
            // We use page/client coordinates relative to the viewport for the perspective effect,
            // but for the spotlight we want coordinates relative to the Hero section
            let rect = el.get_bounding_client_rect();
            set_mouse_pos.set((
                ev.client_x() as f64 - rect.left(),
                ev.client_y() as f64 - rect.top()
            ));
        }
    };

    view! {
        <section 
            node_ref=container_ref
            on:mousemove=handle_mousemove
            class="relative min-h-[90vh] flex items-center justify-center overflow-hidden bg-[#050505]"
        >
            // --- Interactive Grid Background ---
            <div class="absolute inset-0 z-0">
                // The Grid Pattern
                <div class="absolute inset-0 bg-grid-pattern opacity-20"
                     style:transform="perspective(1000px) rotateX(60deg) translateY(0px) translateZ(-100px)">
                </div>
                
                // The Spotlight Mask: Reveals the grid nicely where mouse hovers
                // Uses mix-blend-mode or simple masking logic
                <div 
                    class="absolute inset-0 pointer-events-none"
                    style:background=move || format!(
                        "radial-gradient(800px circle at {}px {}px, rgba(59, 130, 246, 0.15), transparent 50%)", 
                        mouse_pos.get().0, 
                        mouse_pos.get().1
                    )
                ></div>

                // Base vignette to keep edges dark
                <div class="absolute inset-0 bg-radial-fade"></div>
            </div>

            <VStack
                class="relative z-10 text-center px-4"
                align=VAlign::Center
                spacing="2rem".to_string()
            >
                <div class="relative">
                    <h1 class="text-7xl md:text-9xl font-black tracking-tighter text-transparent bg-clip-text bg-gradient-to-b from-white to-gray-600 select-none font-science-gothic">
                        "NICOLAS"
                    </h1>
                    <h1 class="text-7xl md:text-9xl font-black tracking-tighter text-transparent bg-clip-text bg-gradient-to-b from-gray-400 to-gray-800 select-none font-science-gothic -mt-4 md:-mt-8">
                        "ALMINO"
                    </h1>
                </div>

                <p class="text-xl md:text-2xl text-blue-300 font-light tracking-widest uppercase select-none mix-blend-plus-lighter">
                    "Web Developer • Audiovisual • High Perf"
                </p>

                <div class="flex gap-4 mt-8">
                    // Updated with .hover-underline logic via custom CSS or utility
                    <a href="#projects" class="group relative px-8 py-4 rounded-full bg-white text-black font-bold text-lg overflow-hidden transition-all hover:scale-105">
                        <span class="relative z-10">"VER PROJETOS"</span>
                        <div class="absolute inset-0 bg-blue-500 transform scale-x-0 group-hover:scale-x-100 transition-transform origin-left duration-300 -z-0"></div>
                        <span class="absolute inset-0 z-10 text-white opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity duration-300">"VER PROJETOS"</span>
                    </a>
                </div>
            </VStack>
        </section>
    }
}

#[component]
fn Skills() -> impl IntoView {
    view! {
        <section id="skills" class="relative py-32 px-4 overflow-hidden bg-[#0a0a0a]">
            // Engrenagem Gigante Giratória (Background)
            <div class="absolute -bottom-64 -right-64 opacity-5 pointer-events-none select-none">
                <GearIcon class="w-[800px] h-[800px] animate-spin-slow text-white"/>
            </div>

            <div class="max-w-6xl mx-auto relative z-10">
                <h2 class="text-5xl font-bold mb-16 text-white select-none font-science-gothic">"ARSENAL TÉCNICO"</h2>
                
                // MouseSpotlight container removed; SpotlightCard now handles itself
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {SKILLS.iter().map(|cat| {
                        view! {
                            <SpotlightCard class="h-full">
                                <h3 class="text-2xl font-bold mb-6 text-blue-400 select-none">{cat.category}</h3>
                                <div class="flex flex-wrap gap-3">
                                    {cat.items.iter().map(|item| {
                                        view! {
                                            <div class="flex items-center gap-2 px-4 py-2 bg-white/5 rounded-full border border-white/5 hover:bg-white/10 hover:border-blue-500/50 transition-colors group/badge cursor-default">
                                                // SVG Placeholder Image
                                                <div class="w-5 h-5 flex items-center justify-center text-gray-400 group-hover/badge:text-blue-400 transition-colors">
                                                    // Fallback to text if image fails or for dev
                                                    <img 
                                                        src=format!("/assets/icons/{}.png", item.icon_path) 
                                                        alt=item.name
                                                        class="w-full h-full object-contain filter grayscale group-hover/badge:grayscale-0 transition-all rounded-full"
                                                        onerror="this.style.display='none'"
                                                    />
                                                </div>
                                                <span class="text-sm font-medium text-gray-200 hover-underline">{item.name}</span>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </SpotlightCard>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}

#[component]
fn ProjectShowcase() -> impl IntoView {
    let video_ref = NodeRef::<Video>::new();
    let (is_visible, set_visible) = signal(false);

    // Observer para autoplay inteligente do vídeo
    Effect::new(move |_| {
        let UseIntersectionObserverReturn { stop, .. } = use_intersection_observer_with_options(
            video_ref,
            move |entries, _| {
                if let Some(entry) = entries.first() {
                    set_visible.set(entry.is_intersecting());
                }
            },
            UseIntersectionObserverOptions::default().thresholds(vec![0.2]),
        );
        on_cleanup(move || stop());
    });

    Effect::new(move |_| {
        if let Some(video) = video_ref.get() {
            if is_visible.get() {
                let _ = video.play();
            } else {
                let _ = video.pause();
            }
        }
    });

    view! {
        <section id="projects" class="py-32 px-4 bg-[#0c0c0c]">
            <div class="max-w-7xl mx-auto">
                <h2 class="text-5xl font-bold mb-20 text-white text-right select-none font-science-gothic">"PROJETOS SELECIONADOS"</h2>

                // Projeto 1: Camará
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center mb-32">
                    <div class="order-2 lg:order-1">
                        <div class="relative rounded-3xl overflow-hidden border border-white/10 shadow-2xl group cursor-pointer">
                            <video
                                node_ref=video_ref
                                class="w-full h-auto object-cover transform group-hover:scale-105 transition-transform duration-700"
                                loop=true
                                muted=true
                                playsinline=true
                                poster="/assets/camara_poster.jpg" 
                            >
                                <source src="/assets/camaracapoeira_720p.mp4" type="video/mp4"/>
                            </video>
                            <div class="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent opacity-60"></div>
                            
                            <div class="absolute bottom-6 left-6 flex gap-2">
                                <span class="px-3 py-1 bg-orange-600/90 text-white text-xs font-bold rounded-full backdrop-blur-md">"RUST"</span>
                                <span class="px-3 py-1 bg-blue-600/90 text-white text-xs font-bold rounded-full backdrop-blur-md">"LEPTOS"</span>
                            </div>
                        </div>
                    </div>
                    
                    <div class="order-1 lg:order-2 text-left lg:pl-10">
                        <h3 class="text-4xl font-bold text-white mb-4 select-none font-science-gothic">"ASSOCIAÇÃO CAMARÁ"</h3>
                        <p class="text-xl text-gray-400 mb-8 leading-relaxed select-none">
                            "Transformação digital completa para uma ONG de 25 anos. Desenvolvi uma plataforma de alta performance capaz de rodar em hardware de baixo custo, garantindo inclusão digital na fronteira."
                        </p>
                        
                        <div class="flex flex-col gap-4 border-l-2 border-blue-500 pl-6 mb-8">
                            <div class="text-sm text-gray-300">
                                <strong class="text-white block text-lg mb-1">"Desafio"</strong>
                                "Infraestrutura limitada e necessidade de custo zero de manutenção."
                            </div>
                            <div class="text-sm text-gray-300">
                                <strong class="text-white block text-lg mb-1">"Solução"</strong>
                                "Arquitetura Server-Side em Rust com binário de 6MB."
                            </div>
                        </div>

                        // // Link with new animation
                        // <a href="#" class="inline-flex items-center gap-2 text-blue-400 font-bold tracking-wider hover-underline select-none">
                        //     "VER CASE COMPLETO" 
                        //     <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"></path></svg>
                        // </a>
                    </div>
                </div>

                // Projeto 2: Esportes
                <div class="bg-[#151515] rounded-3xl p-8 md:p-12 border border-white/5">
                    <HStack align=HAlign::Center justify=crate::components::stacks::hstack::JustifyContent::SpaceBetween wrap=crate::components::stacks::hstack::FlexWrap::Wrap spacing="2rem".to_string()>
                        <div class="max-w-xl">
                            <h3 class="text-3xl font-bold text-white mb-4 select-none">"ESPORTES NA TV"</h3>
                            <p class="text-gray-400 mb-6 select-none">
                                "Portal de guias esportivos com alto volume de tráfego. Foco em SEO e retenção de usuário."
                            </p>
                        </div>
                        <a href="https://esportesnatv.com.br" target="_blank" class="px-6 py-3 bg-white/5 hover:bg-white/10 rounded-xl text-white font-medium transition-colors select-none border border-white/10 hover-underline">
                            "Acessar Portal"
                        </a>
                    </HStack>
                </div>
            </div>
        </section>
    }
}

// --- Main Page ---

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Style>
            {
                r#"
                /* Font Settings */
                .font-science-gothic {
                    font-family: 'Science Gothic', sans-serif;
                    font-variation-settings: 'wght' 700, 'wdth' 100;
                }

                /* Disable Selection Global */
                * {
                    user-select: none;
                    -webkit-user-select: none;
                }

                /* Custom Scrollbar */
                ::-webkit-scrollbar {
                    width: 8px;
                }
                ::-webkit-scrollbar-track {
                    background: #0a0a0a;
                }
                ::-webkit-scrollbar-thumb {
                    background: #333;
                    border-radius: 4px;
                }
                ::-webkit-scrollbar-thumb:hover {
                    background: #555;
                }
                
                .bg-radial-fade {
                    background: radial-gradient(circle at center, transparent 0%, #0a0a0a 100%);
                }

                @keyframes spin-slow {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }
                .animate-spin-slow {
                    animation: spin-slow 60s linear infinite;
                }
                "#
            }
        </Style>

        <header class="fixed top-0 left-0 right-0 z-50 px-6 py-4 transition-all duration-300">
            <div class="max-w-7xl mx-auto bg-black/50 backdrop-blur-xl border border-white/10 rounded-full px-6 py-3 flex justify-between items-center shadow-lg">
                <span class="font-science-gothic font-bold text-xl tracking-wider text-white">"NA."</span>
                
                <nav class="hidden md:flex gap-8">
                    <a href="#projects" class="text-sm font-medium text-gray-300 hover:text-white transition-colors hover-underline">"PROJETOS"</a>
                    <a href="#skills" class="text-sm font-medium text-gray-300 hover:text-white transition-colors hover-underline">"SKILLS"</a>
                    <a href="#about" class="text-sm font-medium text-gray-300 hover:text-white transition-colors hover-underline">"SOBRE"</a>
                </nav>

                <a href="mailto:nicolas.almino@hotmail.com" class="bg-white text-black px-4 py-1.5 rounded-full text-sm font-bold hover:bg-gray-200 transition-colors">
                    "CONTATO"
                </a>
            </div>
        </header>

        <main class="bg-[#0a0a0a]">
            <Hero/>
            <ProjectShowcase/>
            <Skills/>
            
            <footer class="py-12 text-center text-gray-600 text-sm font-mono border-t border-white/5">
                <p>"DESIGNED & CODED BY NICOLAS ALMINO"</p>
                <p class="mt-2">"POWERED BY RUST & LEPTOS"</p>
            </footer>
        </main>
    }
}