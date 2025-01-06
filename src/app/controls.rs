use leptos::prelude::*;

#[component]
pub fn Controls() -> impl IntoView {
    view! {
        <div class="controls">
            <button class="start">"Start"</button>
            <button class="pause">"Pause"</button>
            <button class="reset">"Reset"</button>
        </div>
    }
}
