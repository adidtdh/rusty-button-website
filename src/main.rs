use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
    <button
        on:click=move |_| {
            set_count.update(|n| *n += 1);
        }
        class=("bg-transparent text-blue-700 font-semibold py-2 px-4 border border-blue-500 rounded")
        class=("bg-red-500", move || count() % 2 == 1)
          >
          "Click me: "
        {move || count.get()}
    </button>
    }
}

