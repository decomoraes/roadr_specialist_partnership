use leptos::prelude::*;

use crate::components::flex_box::FlexBox;
use crate::utils::style;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <FlexBox
            direction="horizontal"
            cross_alignment="center"
            gap=style::size(1.0)
            height=style::size(4.25)
            style="
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            background-color: cyan;
            padding-inline: 1rem;
            background: rgba(255, 255, 255, 0.2);
            box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
            backdrop-filter: blur(10px);
            -webkit-backdrop-filter: blur(10px);
            border-bottom: 1px solid rgba(255, 255, 255, 0.3);
            z-index: 1000;">
                <img src="/roadr_specialist_partnership/images/roadr_black.svg" alt="Exemplo" style="height: 1.75rem" />
        </FlexBox>
    }
}
