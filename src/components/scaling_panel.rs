use leptos::*;
use leptos::prelude::*;

#[component]
pub fn ScalingPanel(
    children: Children,
) -> impl IntoView {
    let scale = RwSignal::new(1.0_f32);
    
    // Removed unused import for leptos_use
    let scroll_y = use_context::<RwSignal<f32>>().expect("Should be inside StackingContainer");
    
    // Create effect that updates scale based on scroll position
    Effect::new(move |_| {
        let current_scroll = scroll_y.get();
        // Adjust these values to control the shrinking behavior
        let new_scale = 1.0_f32 - (current_scroll as f32 / 1000.0).min(0.5).max(0.0);
        scale.set(new_scale);
    });

    view! {
        <div
            style:transform=move || format!("scale({})", scale.get())
            style:transition="transform 0.1s linear"
            style="position: sticky; top: 0; height: 100vh; display: flex; justify-content: center; align-items: center; transform-origin: center top;"
        >
            {children()}
        </div>
    }
}