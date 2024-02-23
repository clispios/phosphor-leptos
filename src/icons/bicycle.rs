//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "health", feature = "map", feature = "objects"))]
#[component]
pub fn Bicycle(
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
                <path d="M204,108a51.82,51.82,0,0,0-15.13,2.25L168.89,76H192a4,4,0,0,1,4,4,12,12,0,0,0,24,0,28,28,0,0,0-28-28H148a12,12,0,0,0-10.37,18l8.14,14H109.56L94.37,58A12,12,0,0,0,84,52H52a12,12,0,0,0,0,24H77.11L88.18,95,74,112.89a52.17,52.17,0,1,0,18.8,14.92l8.37-10.57L118,146.05A12,12,0,1,0,138.7,134L123.56,108h36.21l8.39,14.38A52,52,0,1,0,204,108ZM80,160a28,28,0,1,1-21.71-27.28l-15.7,19.83a12,12,0,0,0,18.82,14.9l15.7-19.83A27.84,27.84,0,0,1,80,160Zm124,28a28,28,0,0,1-23.11-43.79l12.74,21.84A12,12,0,0,0,214.37,154l-12.75-21.84c.79-.07,1.58-.11,2.38-.11a28,28,0,0,1,0,56Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M248,160a40,40,0,1,1-40-40A40,40,0,0,1,248,160ZM48,120a40,40,0,1,0,40,40A40,40,0,0,0,48,120Z"
        opacity="0.2"
    ></path>
    <path d="M208,112a47.81,47.81,0,0,0-16.93,3.09L165.93,72H192a8,8,0,0,1,8,8,8,8,0,0,0,16,0,24,24,0,0,0-24-24H152a8,8,0,0,0-6.91,12l11.65,20H99.26L82.91,60A8,8,0,0,0,76,56H48a8,8,0,0,0,0,16H71.41L85.12,95.51,69.41,117.06a48.13,48.13,0,1,0,12.92,9.44l11.59-15.9L125.09,164A8,8,0,1,0,138.91,156l-30.32-52h57.48l11.19,19.17A48,48,0,1,0,208,112ZM80,160a32,32,0,1,1-20.21-29.74l-18.25,25a8,8,0,1,0,12.92,9.42l18.25-25A31.88,31.88,0,0,1,80,160Zm128,32a32,32,0,0,1-22.51-54.72L201.09,164A8,8,0,1,0,214.91,156L199.3,129.21A32,32,0,1,1,208,192Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M54.46,164.71,82.33,126.5a48,48,0,1,1-12.92-9.44L41.54,155.29a8,8,0,1,0,12.92,9.42ZM208,112a47.81,47.81,0,0,0-16.93,3.09L214.91,156A8,8,0,1,1,201.09,164l-23.83-40.86A48,48,0,1,0,208,112ZM165.93,72H192a8,8,0,0,1,8,8,8,8,0,0,0,16,0,24,24,0,0,0-24-24H152a8,8,0,0,0-6.91,12l11.65,20H99.26L82.91,60A8,8,0,0,0,76,56H48a8,8,0,0,0,0,16H71.41L85.12,95.51,69.41,117.06a47.87,47.87,0,0,1,12.92,9.44l11.59-15.9L125.09,164A8,8,0,1,0,138.91,156l-30.32-52h57.48l11.19,19.17a48.11,48.11,0,0,1,13.81-8.08Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M208,114a45.88,45.88,0,0,0-17.8,3.58L162.45,70H192a10,10,0,0,1,10,10,6,6,0,0,0,12,0,22,22,0,0,0-22-22H152a6,6,0,0,0-5.18,9l13.4,23H98.11L81.18,61A6,6,0,0,0,76,58H48a6,6,0,0,0,0,12H72.55l15,25.64L70,119.62a46.22,46.22,0,1,0,9.68,7.09L94.11,107,126.82,163a6,6,0,0,0,5.19,3,5.91,5.91,0,0,0,3-.82,6,6,0,0,0,2.16-8.2l-32.07-55h62.11l12.63,21.66A46,46,0,1,0,208,114ZM82,160a34,34,0,1,1-19.13-30.57l-19.72,27a6,6,0,0,0,9.7,7.08l19.7-27A33.88,33.88,0,0,1,82,160Zm126,34a34,34,0,0,1-22-59.86L202.82,163a6,6,0,0,0,5.19,3,5.91,5.91,0,0,0,3-.82,6,6,0,0,0,2.16-8.2l-16.86-28.91A34,34,0,1,1,208,194Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,112a47.81,47.81,0,0,0-16.93,3.09L165.93,72H192a8,8,0,0,1,8,8,8,8,0,0,0,16,0,24,24,0,0,0-24-24H152a8,8,0,0,0-6.91,12l11.65,20H99.26L82.91,60A8,8,0,0,0,76,56H48a8,8,0,0,0,0,16H71.41L85.12,95.51,69.41,117.06a48.13,48.13,0,1,0,12.92,9.44l11.59-15.9L125.09,164A8,8,0,1,0,138.91,156l-30.32-52h57.48l11.19,19.17A48,48,0,1,0,208,112ZM80,160a32,32,0,1,1-20.21-29.74l-18.25,25a8,8,0,1,0,12.92,9.42l18.25-25A31.88,31.88,0,0,1,80,160Zm128,32a32,32,0,0,1-22.51-54.72L201.09,164A8,8,0,1,0,214.91,156L199.3,129.21A32,32,0,1,1,208,192Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M208,116a43.66,43.66,0,0,0-18.62,4.15L159,68h33a12,12,0,0,1,12,12,4,4,0,0,0,8,0,20,20,0,0,0-20-20H152a4,4,0,0,0-3.46,6L163.7,92H97L79.46,62A4,4,0,0,0,76,60H48a4,4,0,0,0,0,8H73.7L89.89,95.76,70.57,122.25A44.21,44.21,0,1,0,77,127L94.29,103.3,128.54,162a4,4,0,0,0,3.46,2,4.11,4.11,0,0,0,2-.54,4,4,0,0,0,1.44-5.48l-33.83-58h66.74l14.11,24.19A44,44,0,1,0,208,116ZM84,160a36,36,0,1,1-18.16-31.25L44.77,157.64a4,4,0,0,0,6.46,4.72l21.07-28.9A35.92,35.92,0,0,1,84,160Zm124,36a36,36,0,0,1-21.47-64.88l18,30.9a4,4,0,0,0,3.46,2,4.11,4.11,0,0,0,2-.54,4,4,0,0,0,1.44-5.48l-18-30.89A36,36,0,1,1,208,196Z"></path>
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
