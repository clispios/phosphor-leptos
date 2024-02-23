//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "map", feature = "nature"))]
#[component]
pub fn Star(
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
                <path d="M243,96.05a20,20,0,0,0-17.26-13.72l-57-4.93-22.3-53.14h0a20,20,0,0,0-36.82,0L87.29,77.4l-57,4.93A20,20,0,0,0,18.87,117.4l43.32,37.8-13,56.24A20,20,0,0,0,79,233.1l49-29.76,49,29.76a20,20,0,0,0,29.8-21.66l-13-56.24,43.32-37.8A20,20,0,0,0,243,96.05Zm-66.75,42.62a20,20,0,0,0-6.35,19.63l11.39,49.32-42.94-26.08a19.9,19.9,0,0,0-20.7,0L74.71,207.62,86.1,158.3a20,20,0,0,0-6.35-19.63L41.66,105.44,91.8,101.1a19.92,19.92,0,0,0,16.69-12.19L128,42.42l19.51,46.49A19.92,19.92,0,0,0,164.2,101.1l50.14,4.34Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M184.13,147.7a8.08,8.08,0,0,0-2.54,7.89l13.52,58.54a8,8,0,0,1-11.89,8.69l-51.1-31a7.93,7.93,0,0,0-8.24,0l-51.1,31a8,8,0,0,1-11.89-8.69l13.52-58.54a8.08,8.08,0,0,0-2.54-7.89L26.76,108.35A8,8,0,0,1,31.3,94.28l59.46-5.14a8,8,0,0,0,6.67-4.88L120.66,28.9a8,8,0,0,1,14.68,0l23.23,55.36a8,8,0,0,0,6.67,4.88l59.46,5.14a8,8,0,0,1,4.54,14.07Z"
        opacity="0.2"
    ></path>
    <path d="M239.2,97.29a16,16,0,0,0-13.81-11L166,81.17,142.72,25.81h0a15.95,15.95,0,0,0-29.44,0L90.07,81.17,30.61,86.32a16,16,0,0,0-9.11,28.06L66.61,153.8,53.09,212.34a16,16,0,0,0,23.84,17.34l51-31,51.11,31a16,16,0,0,0,23.84-17.34l-13.51-58.6,45.1-39.36A16,16,0,0,0,239.2,97.29Zm-15.22,5-45.1,39.36a16,16,0,0,0-5.08,15.71L187.35,216v0l-51.07-31a15.9,15.9,0,0,0-16.54,0l-51,31h0L82.2,157.4a16,16,0,0,0-5.08-15.71L32,102.35a.37.37,0,0,1,0-.09l59.44-5.14a16,16,0,0,0,13.35-9.75L128,32.08l23.2,55.29a16,16,0,0,0,13.35,9.75L224,102.26S224,102.32,224,102.33Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M234.5,114.38l-45.1,39.36,13.51,58.6a16,16,0,0,1-23.84,17.34l-51.11-31-51,31a16,16,0,0,1-23.84-17.34L66.61,153.8,21.5,114.38a16,16,0,0,1,9.11-28.06l59.46-5.15,23.21-55.36a15.95,15.95,0,0,1,29.44,0h0L166,81.17l59.44,5.15a16,16,0,0,1,9.11,28.06Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M237.3,97.9a13.78,13.78,0,0,0-12.08-9.6l-59.46-5.14a2,2,0,0,1-1.65-1.22L140.88,26.58h0a14,14,0,0,0-25.76,0L91.89,81.94a2,2,0,0,1-1.65,1.22L30.78,88.3A13.78,13.78,0,0,0,18.7,97.9a14,14,0,0,0,4.11,15l45.11,39.35a2.06,2.06,0,0,1,.64,2L55,212.76a14,14,0,0,0,5.45,14.56,13.74,13.74,0,0,0,15.4.62l51.11-31a1.9,1.9,0,0,1,2,0l51.11,31A14,14,0,0,0,201,212.76l-13.52-58.53a2.06,2.06,0,0,1,.64-2l45.11-39.35A14,14,0,0,0,237.3,97.9Zm-12,5.92-45.11,39.35a14,14,0,0,0-4.44,13.76l13.52,58.53a2,2,0,0,1-.79,2.13,1.81,1.81,0,0,1-2.14.09l-51.11-31a13.92,13.92,0,0,0-14.46,0l-51.11,31a1.81,1.81,0,0,1-2.14-.09,2,2,0,0,1-.79-2.13l13.52-58.53a14,14,0,0,0-4.44-13.76L30.7,103.82a2,2,0,0,1-.59-2.19,1.86,1.86,0,0,1,1.7-1.38l59.47-5.14A14,14,0,0,0,103,86.58l23.23-55.36a2,2,0,0,1,3.62,0L153,86.58a14,14,0,0,0,11.68,8.53l59.47,5.14a1.86,1.86,0,0,1,1.7,1.38A2,2,0,0,1,225.3,103.82Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M239.2,97.29a16,16,0,0,0-13.81-11L166,81.17,142.72,25.81h0a15.95,15.95,0,0,0-29.44,0L90.07,81.17,30.61,86.32a16,16,0,0,0-9.11,28.06L66.61,153.8,53.09,212.34a16,16,0,0,0,23.84,17.34l51-31,51.11,31a16,16,0,0,0,23.84-17.34l-13.51-58.6,45.1-39.36A16,16,0,0,0,239.2,97.29Zm-15.22,5-45.1,39.36a16,16,0,0,0-5.08,15.71L187.35,216v0l-51.07-31a15.9,15.9,0,0,0-16.54,0l-51,31h0L82.2,157.4a16,16,0,0,0-5.08-15.71L32,102.35a.37.37,0,0,1,0-.09l59.44-5.14a16,16,0,0,0,13.35-9.75L128,32.08l23.2,55.29a16,16,0,0,0,13.35,9.75L224,102.26S224,102.32,224,102.33Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M235.38,98.53A11.82,11.82,0,0,0,225,90.29l-59.45-5.14a4,4,0,0,1-3.32-2.44L139,27.36h0a11.95,11.95,0,0,0-22.06,0L93.74,82.71a4,4,0,0,1-3.32,2.44L31,90.29a12,12,0,0,0-6.83,21.07l45.1,39.35a4,4,0,0,1,1.28,4L57,213.22a11.88,11.88,0,0,0,4.67,12.48,11.76,11.76,0,0,0,13.19.53l51.1-31a3.89,3.89,0,0,1,4.08,0l51.1,31a12,12,0,0,0,17.86-13l-13.52-58.54a4,4,0,0,1,1.28-4l45.1-39.35A11.89,11.89,0,0,0,235.38,98.53Zm-8.78,6.8-45.1,39.35a12,12,0,0,0-3.82,11.8L191.2,215a4,4,0,0,1-1.56,4.2,3.86,3.86,0,0,1-4.35.17l-51.1-31a11.88,11.88,0,0,0-12.38,0l-51.1,31a3.86,3.86,0,0,1-4.35-.17A4,4,0,0,1,64.8,215l13.52-58.54a12,12,0,0,0-3.82-11.8L29.4,105.33A4,4,0,0,1,28.22,101a4,4,0,0,1,3.44-2.75l59.45-5.14a12,12,0,0,0,10-7.31l23.22-55.36a4,4,0,0,1,7.32,0l23.22,55.36a12,12,0,0,0,10,7.31l59.45,5.14a4,4,0,0,1,3.44,2.75A4,4,0,0,1,226.6,105.33Z"></path>
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
