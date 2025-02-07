use leptos::prelude::*;

/// Displays a `render_prop` and some children within markup.
#[component]
pub fn FlexBox<'a>(
    children: Children,
    #[prop(optional)] flex: i32,
    #[prop(optional)] background_color: &'a str,
    #[prop(default = "vertical")] direction: &'a str,
    #[prop(optional)] cross_alignment: &'a str,
    #[prop(optional)] main_alignment: &'a str,
    #[prop(optional)] style: &'a str,
    #[prop(optional)] gap: &'a str,
    #[prop(optional)] height: &'a str,
    #[prop(optional)] width: &'a str,
) -> impl IntoView + 'a {
    view! {
        <div style={
            format!(
                "display: flex; {}{}{}{}{}{}{}{}{}",
                handle_property("background-color", background_color),
                handle_flex_grow(flex),
                handle_direction(direction),
                handle_cross_alignment(cross_alignment),
                handle_main_alignment(main_alignment),
                handle_gap(gap),
                handle_property("height", height),
                handle_property("width", width),
                style,
            )}>
            {children()}
        </div>
    }
}

fn handle_flex_grow(flex: i32) -> String {
    if flex == 0 {
        "".to_string()
    } else {
        format!("flex: {};", flex)
    }
}

fn handle_direction(direction: &str) -> String {
    match direction {
        "horizontal" => "flex-direction: row;",
        "vertical" => "flex-direction: column;",
        _ => "flex-direction: column;",
    }
    .to_string()
}

fn handle_cross_alignment(cross_alignment: &str) -> String {
    match cross_alignment {
        "start" => "align-items: flex-start;",
        "end" => "align-items: flex-end;",
        "center" => "align-items: center;",
        "stretch" => "align-items: stretch;",
        "baseline" => "align-items: baseline;",
        _ => "",
    }
    .to_string()
}

fn handle_main_alignment(main_alignment: &str) -> String {
    match main_alignment {
        "start" => "justify-content: flex-start;",
        "end" => "justify-content: flex-end;",
        "center" => "justify-content: center;",
        "space-between" => "justify-content: space-between;",
        "space-around" => "justify-content: space-around;",
        "space-evenly" => "justify-content: space-evenly;",
        _ => "",
    }
    .to_string()
}

fn handle_gap(gap: &str) -> String {
    if gap.is_empty() {
        "".to_string()
    } else {
        format!("gap: {};", gap)
    }
}

fn handle_property(property: &str, value: &str) -> String {
    if value.is_empty() {
        "".to_string()
    } else {
        format!("{}: {};", property, value)
    }
}
