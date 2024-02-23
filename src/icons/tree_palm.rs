//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "nature"))]
#[component]
pub fn TreePalm(
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
                <path d="M240.69,50.47a70.87,70.87,0,0,0-103.54,0A76.58,76.58,0,0,0,128,62.19a76.58,76.58,0,0,0-9.15-11.72,70.87,70.87,0,0,0-103.54,0,12,12,0,0,0,3.9,19.28L66,90.12a75.45,75.45,0,0,0-43.43,89,12,12,0,0,0,18.85,6.41L116,128.75V224a12,12,0,0,0,24,0V128.75l74.54,56.79a12,12,0,0,0,18.85-6.41,75.45,75.45,0,0,0-43.43-89l46.83-20.37a12,12,0,0,0,3.9-19.28ZM67.08,52a47.16,47.16,0,0,1,34.38,15A52.41,52.41,0,0,1,112.3,84.08L48,56.1A46.28,46.28,0,0,1,67.08,52ZM44.39,153.15a51.72,51.72,0,0,1,38.14-43.38A52.83,52.83,0,0,1,96.09,108a50.4,50.4,0,0,1,7,.47ZM205,133.81a51.14,51.14,0,0,1,6.57,19.34L153,108.46a52.21,52.21,0,0,1,20.51,1.31A51.61,51.61,0,0,1,205,133.81ZM143.7,84.08A52.41,52.41,0,0,1,154.54,67a47.16,47.16,0,0,1,34.38-15A46.28,46.28,0,0,1,208,56.1Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,58.75,128,104a65.47,65.47,0,0,1,17.84-45.25,58.87,58.87,0,0,1,86.16,0ZM176.56,98.18A64.2,64.2,0,0,0,128,104.52L221.81,176a63,63,0,0,0-6.39-48.22A63.73,63.73,0,0,0,176.56,98.18Zm-66.4-39.43a58.87,58.87,0,0,0-86.16,0L128,104A65.47,65.47,0,0,0,110.16,58.75ZM79.44,98.18a63.73,63.73,0,0,0-38.86,29.6A63,63,0,0,0,34.19,176L128,104.52A64.2,64.2,0,0,0,79.44,98.18Z"
        opacity="0.2"
    ></path>
    <path d="M237.79,53.23a66.86,66.86,0,0,0-97.74,0,72.21,72.21,0,0,0-12.05,17,72.21,72.21,0,0,0-12-17,66.86,66.86,0,0,0-97.74,0,8,8,0,0,0,2.6,12.85L77,90.55a71.42,71.42,0,0,0-43.36,33.21,70.64,70.64,0,0,0-7.2,54.32A8,8,0,0,0,39,182.36l81-61.68V224a8,8,0,0,0,16,0V120.68l81,61.68a8,8,0,0,0,12.57-4.28,70.64,70.64,0,0,0-7.2-54.32A71.42,71.42,0,0,0,179,90.55l56.22-24.47a8,8,0,0,0,2.6-12.85ZM67.08,48a51.13,51.13,0,0,1,37.28,16.26,56.53,56.53,0,0,1,14.26,26.93L39,56.53A50.5,50.5,0,0,1,67.08,48ZM40,161.5a54.82,54.82,0,0,1,7.47-29.7,55.55,55.55,0,0,1,34-25.89A56.52,56.52,0,0,1,96.1,104a55.82,55.82,0,0,1,16.23,2.41ZM208.5,131.8A54.82,54.82,0,0,1,216,161.5l-72.3-55.1a56.3,56.3,0,0,1,64.83,25.4ZM137.38,91.19a56.53,56.53,0,0,1,14.26-26.93A51.13,51.13,0,0,1,188.92,48,50.5,50.5,0,0,1,217,56.53Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M239.84,60.33a8,8,0,0,1-4.65,5.75L179,90.55a71.42,71.42,0,0,1,43.36,33.21,70.64,70.64,0,0,1,7.2,54.32A8,8,0,0,1,217,182.36l-81-61.68V224a8,8,0,0,1-16,0V120.68L39,182.36a8,8,0,0,1-12.57-4.28,70.64,70.64,0,0,1,7.2-54.32A71.42,71.42,0,0,1,77,90.55L20.81,66.08a8,8,0,0,1-2.6-12.85,66.86,66.86,0,0,1,97.74,0,72.21,72.21,0,0,1,12,17,72.21,72.21,0,0,1,12.05-17,66.86,66.86,0,0,1,97.74,0A8,8,0,0,1,239.84,60.33Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M236.35,54.61a64.87,64.87,0,0,0-94.85,0A70,70,0,0,0,128,75a70,70,0,0,0-13.5-20.37,64.87,64.87,0,0,0-94.85,0,6,6,0,0,0,2,9.64l61.83,26.9c-1.85.34-3.7.74-5.54,1.23a69.42,69.42,0,0,0-42.5,32.39,68.65,68.65,0,0,0-7,52.79,6,6,0,0,0,3.86,4.12,6.1,6.1,0,0,0,1.93.32,6,6,0,0,0,3.64-1.23L122,116.64V224a6,6,0,0,0,12,0V116.64l84.17,64.13a6,6,0,0,0,3.64,1.23,6.1,6.1,0,0,0,1.93-.32,6,6,0,0,0,3.86-4.12,68.65,68.65,0,0,0-7-52.79,69.42,69.42,0,0,0-42.5-32.39c-1.84-.49-3.69-.89-5.54-1.23l61.83-26.9a6,6,0,0,0,2-9.64ZM67.08,46a53.16,53.16,0,0,1,38.73,16.88,58.77,58.77,0,0,1,15.47,31.65L34.93,57A52.59,52.59,0,0,1,67.08,46ZM38.29,165.33a56.77,56.77,0,0,1,7.48-34.53A57.58,57.58,0,0,1,81,104a58.79,58.79,0,0,1,15.12-2,57.67,57.67,0,0,1,20.43,3.73ZM210.23,130.8a56.77,56.77,0,0,1,7.48,34.53l-78.24-59.61a58.24,58.24,0,0,1,70.76,25.08ZM134.72,94.53a58.77,58.77,0,0,1,15.47-31.65A53.16,53.16,0,0,1,188.92,46a52.59,52.59,0,0,1,32.15,11Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M237.79,53.23a66.86,66.86,0,0,0-97.74,0,72.21,72.21,0,0,0-12.05,17,72.21,72.21,0,0,0-12-17,66.86,66.86,0,0,0-97.74,0,8,8,0,0,0,2.6,12.85L77,90.55a71.42,71.42,0,0,0-43.36,33.21,70.64,70.64,0,0,0-7.2,54.32A8,8,0,0,0,39,182.36l81-61.68V224a8,8,0,0,0,16,0V120.68l81,61.68a8,8,0,0,0,12.57-4.28,70.64,70.64,0,0,0-7.2-54.32A71.42,71.42,0,0,0,179,90.55l56.22-24.47a8,8,0,0,0,2.6-12.85ZM67.08,48a51.13,51.13,0,0,1,37.28,16.26,56.53,56.53,0,0,1,14.26,26.93L39,56.53A50.5,50.5,0,0,1,67.08,48ZM40,161.5a54.82,54.82,0,0,1,7.47-29.7,55.55,55.55,0,0,1,34-25.89A56.52,56.52,0,0,1,96.1,104a55.82,55.82,0,0,1,16.23,2.41ZM208.5,131.8A54.82,54.82,0,0,1,216,161.5l-72.3-55.1a56.3,56.3,0,0,1,64.83,25.4ZM137.38,91.19a56.53,56.53,0,0,1,14.26-26.93A51.13,51.13,0,0,1,188.92,48,50.5,50.5,0,0,1,217,56.53Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M234.9,56a62.86,62.86,0,0,0-92,0A68.16,68.16,0,0,0,128,80.44a68.16,68.16,0,0,0-15-24.45A62.86,62.86,0,0,0,21.1,56a4,4,0,0,0,1.3,6.42L90.86,92.2a68.66,68.66,0,0,0-12.45,2.11,67.49,67.49,0,0,0-41.28,31.46A66.72,66.72,0,0,0,30.33,177a4,4,0,0,0,2.57,2.75,4.1,4.1,0,0,0,1.29.21,4,4,0,0,0,2.43-.82L124,112.6V224a4,4,0,0,0,8,0V112.6l87.38,66.58a4,4,0,0,0,2.43.82,4.1,4.1,0,0,0,1.29-.21,4,4,0,0,0,2.57-2.75,66.72,66.72,0,0,0-6.8-51.27,67.49,67.49,0,0,0-41.28-31.46,68.66,68.66,0,0,0-12.45-2.11L233.6,62.41A4,4,0,0,0,234.9,56ZM67.08,44a55.13,55.13,0,0,1,40.18,17.5A60.86,60.86,0,0,1,123.7,97.77L31.12,57.48A54.75,54.75,0,0,1,67.08,44ZM36.77,169A58.72,58.72,0,0,1,44,129.79,59.57,59.57,0,0,1,80.47,102,61,61,0,0,1,96.1,100a59.71,59.71,0,0,1,24.41,5.22ZM212,129.79A58.72,58.72,0,0,1,219.23,169L135.49,105.2A60.24,60.24,0,0,1,212,129.79Zm-79.66-32A60.86,60.86,0,0,1,148.74,61.5a54.8,54.8,0,0,1,76.14-4Z"></path>
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
