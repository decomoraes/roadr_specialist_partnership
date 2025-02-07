use leptos::{ev::Targeted, prelude::*};
use web_sys::{Event, HtmlInputElement};

// impl FnMut(MouseEvent) + 'static
#[component]
pub fn TextBox(
    on_change: impl FnMut(Targeted<Event, HtmlInputElement>) + 'static,
    value: ReadSignal<String>,
) -> impl IntoView {
    view! {
        <input type="text"
            // value=value
            value=move || value.get()
            // on:click=move |_| setter.update(|value| *value = !*value)
            // on:input:target=move |ev| {
            //     setter.update(|value| *value = ev.target().value())
            //     // on_click.set(ev.target().value());
            // }
            on:input:target=on_change
                // .value() returns the current value of an HTML input element
                // setter.set(ev.target().value());
            // }
        />
        // <p>"Name is: " {name}</p>

        <button>"Toggle"</button>
    }
}

// #[component]
// pub fn TextBox(setter: WriteSignal<String> /*, value: ReadSignal<String>*/) -> impl IntoView {
//     view! {
//         <input type="text"
//             // prop:value=value
//             // on:click=move |_| setter.update(|value| *value = !*value)
//             // on:input:target=move |ev| {
//             //     setter.update(|value| *value = ev.target().value())
//             //     // on_click.set(ev.target().value());
//             // }
//             on:input:target=move |ev| {
//                 // .value() returns the current value of an HTML input element
//                 setter.set(ev.target().value());
//             }

//         />
//         // <p>"Name is: " {name}</p>

//         <button>"Toggle"</button>
//     }
// }
