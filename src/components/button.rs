use leptos::prelude::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button
            style="
                background-color: #000000;
                height: 3.5rem;
                border-radius: 1.75rem;
                border: none;
                color: white;
                padding: 0 20px;
                color: var(--Neutrals-White, #FFF);
                text-align: center;
                leading-trim: both;
                text-edge: cap;
                text-shadow: 3.895px 7.789px 23.368px rgba(16, 16, 16, 0.25);
                font-size: 15px;
                font-style: normal;
                font-weight: 600;
                line-height: 100%; /* 15px */
                letter-spacing: 0.195px;
            "
        >{children()}</button>
    }
}
