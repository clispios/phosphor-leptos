//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "development", feature = "editor"))]
#[component]
pub fn BracketsRound(
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
                <path d="M82.33,222.19a12,12,0,0,1-16.5,4.09C64,225.16,20,198,20,128S64,30.84,65.83,29.72A12,12,0,0,1,78.24,50.25C76.71,51.21,44,72.31,44,128s32.85,76.88,34.25,77.75A12,12,0,0,1,82.33,222.19ZM190.17,29.72a12,12,0,1,0-12.42,20.53C179.15,51.12,212,72.19,212,128s-32.85,76.88-34.17,77.7a12,12,0,1,0,12.34,20.58C192,225.16,236,198,236,128S192,30.84,190.17,29.72Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,128c0,64-40,88-40,88H72s-40-24-40-88S72,40,72,40H184S224,64,224,128Z"
        opacity="0.2"
    ></path>
    <path d="M40,128c0,58.29,34.67,80.25,36.15,81.16a8,8,0,0,1-8.27,13.7C66.09,221.78,24,195.75,24,128S66.09,34.22,67.88,33.14a8,8,0,0,1,8.26,13.7C74.54,47.83,40,69.82,40,128ZM188.12,33.14a8,8,0,0,0-8.27,13.7C181.33,47.75,216,69.71,216,128s-34.67,80.25-36.12,81.14a8,8,0,0,0,8.24,13.72C189.91,221.78,232,195.75,232,128S189.91,34.22,188.12,33.14Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM99.61,176.86a8,8,0,0,1-7.19,14.3A71.23,71.23,0,0,1,56,128,71.23,71.23,0,0,1,92.42,64.84a8,8,0,0,1,7.18,14.3C98.37,79.78,72,93.76,72,128S98.48,176.28,99.61,176.86Zm64,14.3a8,8,0,1,1-7.16-14.32c1.1-.56,27.58-14.52,27.58-48.84s-26.48-48.28-27.61-48.86a8,8,0,0,1,7.19-14.3A71.23,71.23,0,0,1,200,128,71.23,71.23,0,0,1,163.58,191.16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M56.52,64C44.23,81.46,38,103,38,128s6.23,46.54,18.52,64c9.17,13,18.49,18.81,18.59,18.87a6,6,0,0,1-6.2,10.27C67.16,220.09,26,194.63,26,128S67.16,35.91,68.91,34.86a6,6,0,0,1,6.2,10.27C75,45.19,65.69,51,56.52,64ZM187.09,34.86a6,6,0,0,0-6.2,10.27c.1.06,9.42,5.84,18.59,18.87C211.77,81.46,218,103,218,128s-6.23,46.54-18.52,64c-9.17,13-18.49,18.81-18.57,18.85a6,6,0,1,0,6.18,10.29c1.75-1,42.91-26.51,42.91-93.14S188.84,35.91,187.09,34.86Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M40,128c0,58.29,34.67,80.25,36.15,81.16a8,8,0,0,1-8.27,13.7C66.09,221.78,24,195.75,24,128S66.09,34.22,67.88,33.14a8,8,0,0,1,8.26,13.7C74.54,47.83,40,69.82,40,128ZM188.12,33.14a8,8,0,0,0-8.27,13.7C181.33,47.75,216,69.71,216,128s-34.67,80.25-36.12,81.14a8,8,0,0,0,8.24,13.72C189.91,221.78,232,195.75,232,128S189.91,34.22,188.12,33.14Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M55,62.67C46.34,74.93,36,96.22,36,128s10.34,53.07,19,65.33c9.41,13.3,19,19.19,19.06,19.24A4,4,0,0,1,72,220a4,4,0,0,1-2-.57C68.23,218.4,28,193.51,28,128S68.23,37.6,69.94,36.57a4,4,0,0,1,4.13,6.86C74,43.48,64.42,49.37,55,62.67Zm131.05-26.1a4,4,0,0,0-4.13,6.86c.1,0,9.65,5.94,19.06,19.24,8.67,12.26,19,33.55,19,65.33s-10.34,53.07-19,65.33c-9.41,13.3-19,19.19-19.05,19.24a4,4,0,0,0,4.12,6.86c1.71-1,41.94-25.92,41.94-91.43S187.77,37.6,186.06,36.57Z"></path>
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
