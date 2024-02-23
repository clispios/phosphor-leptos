//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn Keyboard(
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
                <path d="M223.51,44h-191A20.51,20.51,0,0,0,12,64.49v127A20.51,20.51,0,0,0,32.49,212h191A20.51,20.51,0,0,0,244,191.51v-127A20.51,20.51,0,0,0,223.51,44ZM220,188H36V68H220ZM52,128a12,12,0,0,1,12-12H192a12,12,0,0,1,0,24H64A12,12,0,0,1,52,128Zm0-36A12,12,0,0,1,64,80H192a12,12,0,0,1,0,24H64A12,12,0,0,1,52,92Zm0,72a12,12,0,0,1,12-12h8a12,12,0,0,1,0,24H64A12,12,0,0,1,52,164Zm108,0a12,12,0,0,1-12,12H108a12,12,0,0,1,0-24h40A12,12,0,0,1,160,164Zm44,0a12,12,0,0,1-12,12h-8a12,12,0,0,1,0-24h8A12,12,0,0,1,204,164Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,64.49v127a8.49,8.49,0,0,1-8.49,8.49h-191A8.49,8.49,0,0,1,24,191.51v-127A8.49,8.49,0,0,1,32.49,56h191A8.49,8.49,0,0,1,232,64.49Z"
        opacity="0.2"
    ></path>
    <path d="M223.51,48h-191A16.51,16.51,0,0,0,16,64.49v127A16.51,16.51,0,0,0,32.49,208h191A16.51,16.51,0,0,0,240,191.51v-127A16.51,16.51,0,0,0,223.51,48ZM224,191.51a.49.49,0,0,1-.49.49h-191a.49.49,0,0,1-.49-.49v-127a.49.49,0,0,1,.49-.49h191a.49.49,0,0,1,.49.49ZM208,128a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H200A8,8,0,0,1,208,128Zm0-32a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H200A8,8,0,0,1,208,96ZM72,160a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16h8A8,8,0,0,1,72,160Zm96,0a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16h64A8,8,0,0,1,168,160Zm40,0a8,8,0,0,1-8,8h-8a8,8,0,0,1,0-16h8A8,8,0,0,1,208,160Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M223.51,48h-191A16.51,16.51,0,0,0,16,64.49v127A16.51,16.51,0,0,0,32.49,208h191A16.51,16.51,0,0,0,240,191.51v-127A16.51,16.51,0,0,0,223.51,48ZM64,168H48a8,8,0,0,1,0-16H64a8,8,0,0,1,0,16Zm96,0H96a8,8,0,0,1,0-16h64a8,8,0,0,1,0,16Zm48,0H192a8,8,0,0,1,0-16h16a8,8,0,0,1,0,16Zm0-32H48a8,8,0,0,1,0-16H208a8,8,0,0,1,0,16Zm0-32H48a8,8,0,0,1,0-16H208a8,8,0,0,1,0,16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M223.51,50h-191A14.51,14.51,0,0,0,18,64.49v127A14.51,14.51,0,0,0,32.49,206h191A14.51,14.51,0,0,0,238,191.51v-127A14.51,14.51,0,0,0,223.51,50ZM226,191.51a2.49,2.49,0,0,1-2.49,2.49h-191A2.49,2.49,0,0,1,30,191.51v-127A2.49,2.49,0,0,1,32.49,62h191A2.49,2.49,0,0,1,226,64.49ZM206,128a6,6,0,0,1-6,6H56a6,6,0,0,1,0-12H200A6,6,0,0,1,206,128Zm0-32a6,6,0,0,1-6,6H56a6,6,0,0,1,0-12H200A6,6,0,0,1,206,96ZM70,160a6,6,0,0,1-6,6H56a6,6,0,0,1,0-12h8A6,6,0,0,1,70,160Zm96,0a6,6,0,0,1-6,6H96a6,6,0,0,1,0-12h64A6,6,0,0,1,166,160Zm40,0a6,6,0,0,1-6,6h-8a6,6,0,0,1,0-12h8A6,6,0,0,1,206,160Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M223.51,48h-191A16.51,16.51,0,0,0,16,64.49v127A16.51,16.51,0,0,0,32.49,208h191A16.51,16.51,0,0,0,240,191.51v-127A16.51,16.51,0,0,0,223.51,48ZM224,191.51a.49.49,0,0,1-.49.49h-191a.49.49,0,0,1-.49-.49v-127a.49.49,0,0,1,.49-.49h191a.49.49,0,0,1,.49.49ZM208,128a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H200A8,8,0,0,1,208,128Zm0-32a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H200A8,8,0,0,1,208,96ZM72,160a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16h8A8,8,0,0,1,72,160Zm96,0a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16h64A8,8,0,0,1,168,160Zm40,0a8,8,0,0,1-8,8h-8a8,8,0,0,1,0-16h8A8,8,0,0,1,208,160Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M223.51,52h-191A12.5,12.5,0,0,0,20,64.49v127A12.5,12.5,0,0,0,32.49,204h191A12.5,12.5,0,0,0,236,191.51v-127A12.5,12.5,0,0,0,223.51,52ZM228,191.51a4.49,4.49,0,0,1-4.49,4.49h-191A4.49,4.49,0,0,1,28,191.51v-127A4.49,4.49,0,0,1,32.49,60h191A4.49,4.49,0,0,1,228,64.49ZM204,128a4,4,0,0,1-4,4H56a4,4,0,0,1,0-8H200A4,4,0,0,1,204,128Zm0-32a4,4,0,0,1-4,4H56a4,4,0,0,1,0-8H200A4,4,0,0,1,204,96ZM68,160a4,4,0,0,1-4,4H56a4,4,0,0,1,0-8h8A4,4,0,0,1,68,160Zm96,0a4,4,0,0,1-4,4H96a4,4,0,0,1,0-8h64A4,4,0,0,1,164,160Zm40,0a4,4,0,0,1-4,4h-8a4,4,0,0,1,0-8h8A4,4,0,0,1,204,160Z"></path>
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
