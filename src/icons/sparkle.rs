//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "nature"))]
#[component]
pub fn Sparkle(
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
            IconWeight::Fill => view! {
                <path d="M208,144a15.78,15.78,0,0,1-10.42,14.94l-51.65,19-19,51.61a15.92,15.92,0,0,1-29.88,0L78,178l-51.62-19a15.92,15.92,0,0,1,0-29.88l51.65-19,19-51.61a15.92,15.92,0,0,1,29.88,0l19,51.65,51.61,19A15.78,15.78,0,0,1,208,144ZM152,48h16V64a8,8,0,0,0,16,0V48h16a8,8,0,0,0,0-16H184V16a8,8,0,0,0-16,0V32H152a8,8,0,0,0,0,16Zm88,32h-8V72a8,8,0,0,0-16,0v8h-8a8,8,0,0,0,0,16h8v8a8,8,0,0,0,16,0V96h8a8,8,0,0,0,0-16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M194.82,151.43l-51.66,19a7.88,7.88,0,0,0-4.69,4.69l-19,51.66a7.92,7.92,0,0,1-14.86,0l-19-51.66a7.88,7.88,0,0,0-4.69-4.69l-51.66-19a7.92,7.92,0,0,1,0-14.86l51.66-19a7.88,7.88,0,0,0,4.69-4.69l19-51.66a7.92,7.92,0,0,1,14.86,0l19,51.66a7.88,7.88,0,0,0,4.69,4.69l51.66,19A7.92,7.92,0,0,1,194.82,151.43Z"
        opacity="0.2"
    ></path>
    <path d="M197.58,129.06l-51.61-19-19-51.65a15.92,15.92,0,0,0-29.88,0L78.07,110l-51.65,19a15.92,15.92,0,0,0,0,29.88L78,178l19,51.62a15.92,15.92,0,0,0,29.88,0l19-51.61,51.65-19a15.92,15.92,0,0,0,0-29.88ZM140.39,163a15.87,15.87,0,0,0-9.43,9.43l-19,51.46L93,172.39A15.87,15.87,0,0,0,83.61,163h0L32.15,144l51.46-19A15.87,15.87,0,0,0,93,115.61l19-51.46,19,51.46a15.87,15.87,0,0,0,9.43,9.43l51.46,19ZM144,40a8,8,0,0,1,8-8h16V16a8,8,0,0,1,16,0V32h16a8,8,0,0,1,0,16H184V64a8,8,0,0,1-16,0V48H152A8,8,0,0,1,144,40ZM248,88a8,8,0,0,1-8,8h-8v8a8,8,0,0,1-16,0V96h-8a8,8,0,0,1,0-16h8V72a8,8,0,0,1,16,0v8h8A8,8,0,0,1,248,88Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M196.2,132.81l-51.66-19a3.91,3.91,0,0,1-2.32-2.32l-19-51.66a11.93,11.93,0,0,0-22.38,0l-19,51.66a3.91,3.91,0,0,1-2.32,2.32l-51.66,19a11.93,11.93,0,0,0,0,22.38l51.66,19a3.91,3.91,0,0,1,2.32,2.32l19,51.66a11.93,11.93,0,0,0,22.38,0l19-51.66a3.91,3.91,0,0,1,2.32-2.32l51.66-19a11.93,11.93,0,0,0,0-22.38Zm-2.77,14.87-51.65,19a11.93,11.93,0,0,0-7.07,7.07l-19,51.65a3.92,3.92,0,0,1-7.36,0l-19-51.65a11.93,11.93,0,0,0-7.07-7.07h0l-51.65-19a3.92,3.92,0,0,1,0-7.36l51.65-19a11.93,11.93,0,0,0,7.07-7.07l19-51.65a3.92,3.92,0,0,1,7.36,0l19,51.65a11.93,11.93,0,0,0,7.07,7.07l51.65,19a3.92,3.92,0,0,1,0,7.36ZM148,40a4,4,0,0,1,4-4h20V16a4,4,0,0,1,8,0V36h20a4,4,0,0,1,0,8H180V64a4,4,0,0,1-8,0V44H152A4,4,0,0,1,148,40Zm96,48a4,4,0,0,1-4,4H228v12a4,4,0,0,1-8,0V92H208a4,4,0,0,1,0-8h12V72a4,4,0,0,1,8,0V84h12A4,4,0,0,1,244,88Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M199,125.31l-49.89-18.38L130.69,57a19.92,19.92,0,0,0-37.38,0L74.93,106.93,25,125.31a19.92,19.92,0,0,0,0,37.38l49.89,18.38L93.31,231a19.92,19.92,0,0,0,37.38,0l18.38-49.89L199,162.69a19.92,19.92,0,0,0,0-37.38Zm-60,33.9a19.89,19.89,0,0,0-11.8,11.8L112,212.28,96.79,171A19.89,19.89,0,0,0,85,159.21h0L43.72,144,85,128.79A19.89,19.89,0,0,0,96.79,117L112,75.72,127.21,117a19.89,19.89,0,0,0,11.8,11.8L180.28,144ZM140,40a12,12,0,0,1,12-12h12V16a12,12,0,0,1,24,0V28h12a12,12,0,0,1,0,24H188V64a12,12,0,0,1-24,0V52H152A12,12,0,0,1,140,40ZM252,88a12,12,0,0,1-12,12h-4v4a12,12,0,0,1-24,0v-4h-4a12,12,0,0,1,0-24h4V72a12,12,0,0,1,24,0v4h4A12,12,0,0,1,252,88Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M196.89,130.94l-51.65-19a2,2,0,0,1-1.15-1.14l-19-51.66a13.92,13.92,0,0,0-26.12,0l-19,51.65a2,2,0,0,1-1.14,1.15l-51.66,19a13.92,13.92,0,0,0,0,26.12l51.65,19a2,2,0,0,1,1.15,1.14l19,51.66a13.92,13.92,0,0,0,26.12,0l19-51.65a2,2,0,0,1,1.14-1.15l51.66-19a13.92,13.92,0,0,0,0-26.12Zm-4.15,14.86-51.66,19a13.94,13.94,0,0,0-8.25,8.26l-19,51.65a1.92,1.92,0,0,1-3.6,0l-19-51.66a14,14,0,0,0-8.25-8.25h0l-51.65-19a1.92,1.92,0,0,1,0-3.6l51.66-19a13.94,13.94,0,0,0,8.25-8.26l19-51.65a1.92,1.92,0,0,1,3.6,0l19,51.66a13.94,13.94,0,0,0,8.26,8.25l51.65,19a1.92,1.92,0,0,1,0,3.6ZM146,40a6,6,0,0,1,6-6h18V16a6,6,0,0,1,12,0V34h18a6,6,0,0,1,0,12H182V64a6,6,0,0,1-12,0V46H152A6,6,0,0,1,146,40ZM246,88a6,6,0,0,1-6,6H230v10a6,6,0,0,1-12,0V94H208a6,6,0,0,1,0-12h10V72a6,6,0,0,1,12,0V82h10A6,6,0,0,1,246,88Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M197.58,129.06l-51.61-19-19-51.65a15.92,15.92,0,0,0-29.88,0L78.07,110l-51.65,19a15.92,15.92,0,0,0,0,29.88L78,178l19,51.62a15.92,15.92,0,0,0,29.88,0l19-51.61,51.65-19a15.92,15.92,0,0,0,0-29.88ZM140.39,163a15.87,15.87,0,0,0-9.43,9.43l-19,51.46L93,172.39A15.87,15.87,0,0,0,83.61,163h0L32.15,144l51.46-19A15.87,15.87,0,0,0,93,115.61l19-51.46,19,51.46a15.87,15.87,0,0,0,9.43,9.43l51.46,19ZM144,40a8,8,0,0,1,8-8h16V16a8,8,0,0,1,16,0V32h16a8,8,0,0,1,0,16H184V64a8,8,0,0,1-16,0V48H152A8,8,0,0,1,144,40ZM248,88a8,8,0,0,1-8,8h-8v8a8,8,0,0,1-16,0V96h-8a8,8,0,0,1,0-16h8V72a8,8,0,0,1,16,0v8h8A8,8,0,0,1,248,88Z"></path>
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
