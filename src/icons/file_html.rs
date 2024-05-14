//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "office", feature = "editor", feature = "development"))]
#[component]
pub fn FileHtml(
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
                <path d="M216,112V88a8,8,0,0,0-2.34-5.66l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v72a8,8,0,0,0,8,8H208A8,8,0,0,0,216,112ZM152,88V44l44,44Zm-24,64a8,8,0,0,1-8,8h-8v48a8,8,0,0,1-16,0V160H88a8,8,0,0,1,0-16h32A8,8,0,0,1,128,152Zm-56,0v56a8,8,0,0,1-16,0V188H40v20a8,8,0,0,1-16,0V152a8,8,0,0,1,16,0v20H56V152a8,8,0,0,1,16,0Zm176,56a8,8,0,0,1-8,8H220a8,8,0,0,1-8-8V152a8,8,0,0,1,16,0v48h12A8,8,0,0,1,248,208Zm-48-56v56a8,8,0,0,1-16,0V180.88l-9.14,15.24a8,8,0,0,1-13.72,0L152,180.88V208a8,8,0,0,1-16,0V152a8,8,0,0,1,14.86-4.12L168,176.45l17.14-28.57A8,8,0,0,1,200,152Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,88H152V32Z" opacity="0.2"></path>
    <path d="M128,152a8,8,0,0,1-8,8h-8v48a8,8,0,0,1-16,0V160H88a8,8,0,0,1,0-16h32A8,8,0,0,1,128,152Zm-64-8a8,8,0,0,0-8,8v20H40V152a8,8,0,0,0-16,0v56a8,8,0,0,0,16,0V188H56v20a8,8,0,0,0,16,0V152A8,8,0,0,0,64,144Zm176,56H228V152a8,8,0,0,0-16,0v56a8,8,0,0,0,8,8h20a8,8,0,0,0,0-16Zm-45.86-55.71a8,8,0,0,0-9,3.59L168,176.45l-17.14-28.57A8,8,0,0,0,136,152v56a8,8,0,0,0,16,0V180.88l9.14,15.24a8,8,0,0,0,13.72,0L184,180.88V208a8,8,0,0,0,16,0V152A8,8,0,0,0,194.14,144.29ZM208,120a8,8,0,0,1-8-8V96H152a8,8,0,0,1-8-8V40H56v72a8,8,0,0,1-16,0V40A16,16,0,0,1,56,24h96a8,8,0,0,1,5.66,2.34l56,56A8,8,0,0,1,216,88v24A8,8,0,0,1,208,120ZM188.69,80,160,51.31V80Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M124,152a4,4,0,0,1-4,4H108v52a4,4,0,0,1-8,0V156H88a4,4,0,0,1,0-8h32A4,4,0,0,1,124,152Zm-60-4a4,4,0,0,0-4,4v24H36V152a4,4,0,0,0-8,0v56a4,4,0,0,0,8,0V184H60v24a4,4,0,0,0,8,0V152A4,4,0,0,0,64,148Zm176,56H224V152a4,4,0,0,0-8,0v56a4,4,0,0,0,4,4h20a4,4,0,0,0,0-8Zm-46.93-55.85a4,4,0,0,0-4.5,1.79L168,184.23l-20.57-34.29A4,4,0,0,0,140,152v56a4,4,0,0,0,8,0V166.44l16.57,27.62a4,4,0,0,0,6.86,0L188,166.44V208a4,4,0,0,0,8,0V152A4,4,0,0,0,193.07,148.15Zm17.76-63A4,4,0,0,1,212,88v24a4,4,0,0,1-8,0V92H152a4,4,0,0,1-4-4V36H56a4,4,0,0,0-4,4v72a4,4,0,0,1-8,0V40A12,12,0,0,1,56,28h96a4,4,0,0,1,2.83,1.17ZM198.34,84,156,41.65V84Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M128,152a12,12,0,0,1-12,12h-2v44a12,12,0,0,1-24,0V164H88a12,12,0,0,1,0-24h28A12,12,0,0,1,128,152ZM56,140a12,12,0,0,0-12,12v16H36V152a12,12,0,0,0-24,0v56a12,12,0,0,0,24,0V192h8v16a12,12,0,0,0,24,0V152A12,12,0,0,0,56,140Zm188,56h-4V152a12,12,0,0,0-24,0v56a12,12,0,0,0,12,12h16a12,12,0,0,0,0-24Zm-44.8-55.56a12,12,0,0,0-13.49,5.39L172,168.68l-13.71-22.85A12,12,0,0,0,136,152v56a12,12,0,0,0,24,0V195.32l1.71,2.85a12,12,0,0,0,20.58,0l1.71-2.85V208a12,12,0,0,0,24,0V152A12,12,0,0,0,199.2,140.44ZM208,120a12,12,0,0,1-12-12v-4H148a12,12,0,0,1-12-12V44H60v64a12,12,0,0,1-24,0V40A20,20,0,0,1,56,20h96a12,12,0,0,1,8.49,3.52l56,56A12,12,0,0,1,220,88v20A12,12,0,0,1,208,120ZM183,80,160,57V80Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M126,152a6,6,0,0,1-6,6H110v50a6,6,0,0,1-12,0V158H88a6,6,0,0,1,0-12h32A6,6,0,0,1,126,152Zm-62-6a6,6,0,0,0-6,6v22H38V152a6,6,0,0,0-12,0v56a6,6,0,0,0,12,0V186H58v22a6,6,0,0,0,12,0V152A6,6,0,0,0,64,146Zm176,56H226V152a6,6,0,0,0-12,0v56a6,6,0,0,0,6,6h20a6,6,0,0,0,0-12Zm-46.4-55.78a6,6,0,0,0-6.75,2.69L168,180.34l-18.85-31.43A6,6,0,0,0,138,152v56a6,6,0,0,0,12,0V173.66l12.85,21.43a6,6,0,0,0,10.3,0L186,173.66V208a6,6,0,0,0,12,0V152A6,6,0,0,0,193.6,146.22ZM214,112a6,6,0,0,1-12,0V94H152a6,6,0,0,1-6-6V38H56a2,2,0,0,0-2,2v72a6,6,0,0,1-12,0V40A14,14,0,0,1,56,26h96a6,6,0,0,1,4.25,1.76l56,56A6,6,0,0,1,214,88ZM193.52,82,158,46.48V82Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,152a8,8,0,0,1-8,8h-8v48a8,8,0,0,1-16,0V160H88a8,8,0,0,1,0-16h32A8,8,0,0,1,128,152Zm-64-8a8,8,0,0,0-8,8v20H40V152a8,8,0,0,0-16,0v56a8,8,0,0,0,16,0V188H56v20a8,8,0,0,0,16,0V152A8,8,0,0,0,64,144Zm176,56H228V152a8,8,0,0,0-16,0v56a8,8,0,0,0,8,8h20a8,8,0,0,0,0-16Zm-45.86-55.71a8,8,0,0,0-9,3.59L168,176.45l-17.14-28.57A8,8,0,0,0,136,152v56a8,8,0,0,0,16,0V180.88l9.14,15.24a8,8,0,0,0,13.72,0L184,180.88V208a8,8,0,0,0,16,0V152A8,8,0,0,0,194.14,144.29ZM208,120a8,8,0,0,1-8-8V96H152a8,8,0,0,1-8-8V40H56v72a8,8,0,0,1-16,0V40A16,16,0,0,1,56,24h96a8,8,0,0,1,5.66,2.34l56,56A8,8,0,0,1,216,88v24A8,8,0,0,1,208,120ZM188.69,80,160,51.31V80Z"></path>
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
