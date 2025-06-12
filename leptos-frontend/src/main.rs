mod quote;

use std::collections::HashSet;
use leptos::prelude::*;

// Gemini wrote this part initially,
//and I worked on the framework provided by Bart Massey at
//https://github.com/pdx-cs-rust-web/kk2-client-leptos
#[component]
pub fn EnterInput(set_endpoint: WriteSignal<String>) -> impl IntoView {
    // Create a signal to store the current input value
    let (input_text, set_input_text) = signal("".to_string());

    // Define the action to be performed on Enter
    let handle_enter_action = move |_| {
        // This closure needs to capture 'input_text' and 'set_submitted_text'
        // to read the current input and update the submitted text.
        let current_input = input_text.get(); // Get the current value from the signal
        if !current_input.trim().is_empty() {
            set_endpoint.set(format!("quotes/{}", current_input));
        }
    };

    view! {
        <div>
            "Find a quote: " <input
                type="text"
                // Bind the input's value to the signal
                prop:value=input_text
                // Update the signal when the input changes
                on:input=move |ev| {
                    set_input_text.set(event_target_value(&ev));
                }
                // Listen for keydown events
                on:keydown=move |ev: web_sys::KeyboardEvent| {
                    if ev.key() == "Enter" {
                        handle_enter_action(ev);
                    }
                }
                placeholder="Quote ID"
            />
        </div>
    }
}

// fn format_tags(tags: &HashSet<String>) -> String {
//     let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
//     taglist.join(", ")
// }

fn fetch_quote() -> impl IntoView {
    let (endpoint, set_endpoint) = signal::<String>("quotes/random".to_string());
    let quote = LocalResource::new(move || quote::fetch(endpoint.get()));

    let error_fallback = move |errors: ArcRwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect::<Vec<_>>()
            })
        };

        view! {
            <div>
                <h2>"Error"</h2>
                <span class="error">{error_list}</span>
            </div>
        }
    };

    view! {
        <div><Transition fallback=|| view! { <div>"Loading..."</div> }>
            <ErrorBoundary fallback=error_fallback>
                {move || Suspend::new( async move {
                    quote.map(|j| {
                        let j = j.as_ref().unwrap();
                        view! {
                            <div class="quote-display">
                                <blockquote>
                                    {j.qtext.clone()}
                                </blockquote>
                                <p class="author">{j.author.clone()}</p>
                                <p class="creator">Added by: {j.creator.clone()}</p>
                            </div>
                            <span class="annotation">
                                {format!(
                                    "[id: {}; author: {}; creator: {}]",
                                    j.id,
                                    j.author,
                                    j.creator,
                                )}
                            </span>
                        }
                    })
                })}
            </ErrorBoundary>
        </Transition></div>
        <div>
            <button on:click=move |_| {
                let ep = "".to_string();
                set_endpoint.set(ep)
            }>Another Quote!</button>
            <EnterInput set_endpoint=set_endpoint/>
        </div>
    }
}

pub fn main() {
    use tracing_subscriber::fmt;
    use tracing_subscriber_wasm::MakeConsoleWriter;

    fmt()
        .with_writer(
            // To avoid trace events in the browser from showing their
            // JS backtrace, which is very annoying, in my opinion
            MakeConsoleWriter::default()
                .map_trace_level_to(tracing::Level::DEBUG),
        )
        // For some reason, if we don't do this in the browser, we get
        // a runtime error.
        .without_time()
        .init();
    console_error_panic_hook::set_once();
    mount_to_body(fetch_quote)
}
