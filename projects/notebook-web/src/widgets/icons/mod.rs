use yew::prelude::*;

pub fn add_icon(u: usize) -> Html {
    html! {
    <svg viewBox="0 0 15 15" width="15" height="15" stroke="currentColor"
        style="stroke-width: 1; stroke-linecap: round;">
        <line x1="7.5" x2="7.5" y1="2.5" y2="12.5"></line>
        <line y1="7.5" y2="7.5" x1="2.5" x2="12.5"></line>
    </svg>
    }
}

pub fn run_icon(u: usize) -> Html {
    html! {
    <svg viewBox="-1 0 16 16" width="16" height="16" stroke="currentColor" stroke-linejoin="round" stroke-width="1.6" fill="none" role="button">
        <path d=" M11.7206 6.94335 C12.2406 7.34365 12.2406 8.12786 11.7206 8.52816L5.60999 13.2321 C4.95242 13.7383 4 13.2696 4 12.4397L4 3.03178 C4 2.20194 4.95243 1.73318 5.60999 2.23937L11.7206 6.94335Z"></path>
    </svg>
    }
}
