use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/timer-app.css"/>

        // sets the document title
        <Title text="Tabata Timer!"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Tabata Timer!"</h1>
        <div class="timer-container">
            <Clock/>
            <Rounds />
            <Controls />
        </div>
        <div class="settings">
            <Settings />
        </div>
    }
}

#[component]
fn Clock() -> impl IntoView {
    view! {
        <div class="timer">"00:00"</div>
    }
}

#[component]
fn Rounds() -> impl IntoView {
    view! {
        <div class="rounds-counter">"1/8"</div>
    }
}

#[component]
fn Controls() -> impl IntoView {
    view! {
        <div class="controls">
            <button class="start">"Start"</button>
            <button class="pause">"Pause"</button>
            <button class="reset">"Reset"</button>
        </div>
    }
}

#[component]
fn Settings() -> impl IntoView {
    view! {
        <div class="settings">
            <form>
                <div class="settings-row">
                    <label for="work-time">"Work Time"</label>
                    <input type="number" id="work-time" name="work-time" min="1" max="60" value="20"/>
                </div>
                <div class="settings-row">
                    <label for="rest-time">"Rest Time"</label>
                    <input type="number" id="rest-time" name="rest-time" min="1" max="60" value="10"/>
                </div>
                <div class="settings-row">
                    <label for="rounds">"Rounds"</label>
                    <input type="number" id="rounds" name="rounds" min="1" max="60" value="8"/>
                </div>
                <div class="settings-row">
                    <label for="prep-time">"Prep Time"</label>
                    <input type="number" id="prep-time" name="prep-time" min="1" max="60" value="10"/>
                </div>
                <div class="total-time">"Total Time: 00:00"</div>
            </form>
        </div>
    }
}


