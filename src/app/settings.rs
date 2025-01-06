use leptos::prelude::*;

#[component]
pub fn Settings() -> impl IntoView {
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
