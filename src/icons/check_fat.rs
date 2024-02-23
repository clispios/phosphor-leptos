//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn CheckFat(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Bold => view! {
                <path d="M246.15,65.46l-.07-.07L222.15,41.85a20,20,0,0,0-28.22-.06l-90,88.62L70,97.76a20,20,0,0,0-28.19.1l-24,24a20,20,0,0,0,0,28.26l71.62,72a20,20,0,0,0,28.29,0L246.15,93.74A20,20,0,0,0,246.15,65.46ZM103.61,202.33,37.64,136,56.05,117.6,90,150.24a20,20,0,0,0,28.12,0L208,61.61l18.32,18Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M237.66,85.26l-128.4,128.4a8,8,0,0,1-11.32,0l-71.6-72a8,8,0,0,1,0-11.31l24-24a8,8,0,0,1,11.32,0l36.68,35.32a8,8,0,0,0,11.32,0l92.68-91.32a8,8,0,0,1,11.32,0l24,23.6A8,8,0,0,1,237.66,85.26Z"
        opacity="0.2"
    ></path>
    <path d="M243.28,68.24l-24-23.56a16,16,0,0,0-22.58,0L104,136h0l-.11-.11L67.25,100.62a16,16,0,0,0-22.57.06l-24,24a16,16,0,0,0,0,22.61l71.62,72a16,16,0,0,0,22.63,0L243.33,90.91A16,16,0,0,0,243.28,68.24ZM103.62,208,32,136l24-24,.11.11,36.64,35.27a16,16,0,0,0,22.52,0L208.06,56,232,79.6Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M243.33,90.91,114.92,219.31a16,16,0,0,1-22.63,0l-71.62-72a16,16,0,0,1,0-22.61l24-24a16,16,0,0,1,22.57-.06l36.64,35.27.11.11h0l92.73-91.37a16,16,0,0,1,22.58,0l24,23.56A16,16,0,0,1,243.33,90.91Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M241.87,69.66l-24-23.56a14,14,0,0,0-19.77,0l-92.72,91.34a2,2,0,0,1-2.83,0l-.09-.08L65.85,102.05a14,14,0,0,0-19.75,0l-24,24a14,14,0,0,0,0,19.79l71.62,72a14,14,0,0,0,19.8,0L241.91,89.5A14,14,0,0,0,241.87,69.66ZM233.42,81,105,209.41a2,2,0,0,1-2.81,0l-71.62-72a2,2,0,0,1,0-2.82l24-24A2,2,0,0,1,56,110a2,2,0,0,1,1.41.58l.08.08,36.66,35.28a14,14,0,0,0,19.72,0l92.72-91.35a2,2,0,0,1,2.87,0l24,23.56A2,2,0,0,1,233.42,81Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M243.28,68.24l-24-23.56a16,16,0,0,0-22.58,0L104,136h0l-.11-.11L67.25,100.62a16,16,0,0,0-22.57.06l-24,24a16,16,0,0,0,0,22.61l71.62,72a16,16,0,0,0,22.63,0L243.33,90.91A16,16,0,0,0,243.28,68.24ZM103.62,208,32,136l24-24,.11.11,36.64,35.27a16,16,0,0,0,22.52,0L208.06,56,232,79.6Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M240.47,71.09l-24-23.58a12,12,0,0,0-17,0l-92.71,91.34a4,4,0,0,1-5.66,0l0-.06L64.48,103.51a12,12,0,0,0-17,0l-24,24a12,12,0,0,0,0,17l71.61,72a12,12,0,0,0,17,0L240.49,88.08A12,12,0,0,0,240.47,71.09Zm-5.63,11.34L106.43,210.83a4,4,0,0,1-5.65,0l-71.61-72a4,4,0,0,1,0-5.66l24-24A4,4,0,0,1,56,108a4.09,4.09,0,0,1,2.9,1.21l36.66,35.29a12,12,0,0,0,16.93,0l92.71-91.33a4,4,0,0,1,5.68,0l24,23.58A4,4,0,0,1,234.84,82.43Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().unwrap_or(TextProp::from(""))
            class=move || class.get().unwrap_or(TextProp::from(""))
        >
            {body}
        </svg>
    }
}
