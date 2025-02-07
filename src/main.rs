mod components;
mod utils;

use components::{button::Button, flex_box::FlexBox, header::Header, text_box::TextBox};
use leptos::prelude::*;
use utils::style;

/// Displays a `render_prop` and some children within markup.
#[component]
pub fn TakesChildren<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` can take one of several different types, each of which
    /// is a function that returns some view type
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h1><code>"<TakesChildren/>"</code></h1>
        <h2>"Render Prop"</h2>
        {render_prop()}
        <hr/>
        <h2>"Children"</h2>
        {children()}
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (name, set_name) = signal("".to_string());
    let (email, set_email) = signal("".to_string());
    let (company_name, set_company_name) = signal("".to_string());

    let input_style = "
        height: 3.5rem;
        margin: 0;
        padding: 0 20px;
        font-size: 1rem;
        leading-trim: both;
        text-edge: cap;
        font-size: 14px;
        font-style: normal;
        font-weight: 600;
        line-height: 140%;
        letter-spacing: 0.2px;
        border-radius: 12px;
        border: none;
        background: #F2F2F2;
        width: 100%;
    ";

    view! {
        <FlexBox style="min-height: 100dvh; padding-top: 5.75rem; padding-bottom: 1.5rem; padding-inline: 1.5rem;">
            <Header />

            <FlexBox cross_alignment="center">

            // <TextBox
            //     value=name
            //     on_change=move |ev| {
            //         set_name.set(ev.target().value());
            //     }
            // />
            // <TextBox setter=set_name />
                <FlexBox gap=style::size(1.0) cross_alignment="stretch" style="max-width: 800px; width: 100%;">
                <p style="
                    color: var(--Neutrals-Black, #000);
                    leading-trim: both;
                    text-edge: cap;
                    font-size: 20px;
                    font-style: normal;
                    font-weight: 600;
                    line-height: 140%; /* 28px */
                    letter-spacing: 0.195px;
                ">Complete your setup</p>
                    <input
                        style=input_style
                        placeholder="Name"
                        type="text"
                        bind:value=(name, set_name)
                    />
                    <input
                        style=input_style
                        placeholder="Email"
                        type="email"
                        bind:value=(email, set_email)
                    />
                    <input
                        style=input_style
                        placeholder="Company name"
                        type="company_name"
                        bind:value=(company_name, set_company_name)
                    />
                    <Button
                        on:click=move |_| {
                            *set_count.write() += 1;
                        }
                    >
                        "Go to the payment"
                    </Button>
                </FlexBox>
                // <p>"Name is: " {name}</p>
                // <p>"Email is: " {email}</p>
                // <Button
                //     on:click=move |_| {
                //         *set_count.write() += 1;
                //     }
                // >
                //     "Click me: "
                //     {count}
                // </Button>
            </FlexBox>
            // <FlexBox background_color="magenta" flex=1 direction="horizontal" cross_alignment="end">
            //     <p>Heyy</p>
            //     <FlexBox background_color="yellow" flex=1>
            //         <p>Heyy</p>
            //     </FlexBox>
            //     <FlexBox background_color="pink" flex=2>
            //         <p>Heyy</p>
            //     </FlexBox>
            // </FlexBox>
            // <TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
            //     // these get passed to `children`
            //     "Some text"
            //     <span>"A span"</span>
            // </TakesChildren>
        </FlexBox>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
