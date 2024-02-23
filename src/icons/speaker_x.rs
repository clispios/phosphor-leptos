//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media", feature = "system"))]
#[component]
pub fn SpeakerX(
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
                <path d="M157.27,21.22a12,12,0,0,0-12.64,1.31L75.88,76H32A20,20,0,0,0,12,96v64a20,20,0,0,0,20,20H75.88l68.75,53.47A12,12,0,0,0,164,224V32A12,12,0,0,0,157.27,21.22ZM36,100H68v56H36Zm104,99.47L92,162.12V93.88l48-37.34Zm108.49-55.95a12,12,0,0,1-17,17L216,145l-15.51,15.52a12,12,0,0,1-17-17L199,128l-15.52-15.51a12,12,0,0,1,17-17L216,111l15.51-15.51a12,12,0,0,1,17,17L233,128Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M80,88v80H32a8,8,0,0,1-8-8V96a8,8,0,0,1,8-8Z" opacity="0.2"></path>
    <path d="M155.51,24.81a8,8,0,0,0-8.42.88L77.25,80H32A16,16,0,0,0,16,96v64a16,16,0,0,0,16,16H77.25l69.84,54.31A8,8,0,0,0,160,224V32A8,8,0,0,0,155.51,24.81ZM32,96H72v64H32ZM144,207.64,88,164.07V91.94l56-43.58Zm101.66-61.3a8,8,0,0,1-11.32,11.32L216,139.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L204.69,128l-18.35-18.34a8,8,0,0,1,11.32-11.32L216,116.69l18.34-18.35a8,8,0,0,1,11.32,11.32L227.31,128Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M155.51,24.81a8,8,0,0,0-8.42.88L77.25,80H32A16,16,0,0,0,16,96v64a16,16,0,0,0,16,16H77.25l69.84,54.31A8,8,0,0,0,160,224V32A8,8,0,0,0,155.51,24.81ZM32,96H72v64H32Z"></path>
    <path d="M227.31,128l18.35-18.34a8,8,0,0,0-11.32-11.32L216,116.69,197.66,98.34a8,8,0,0,0-11.32,11.32L204.69,128l-18.35,18.34a8,8,0,0,0,11.32,11.32L216,139.31l18.34,18.35a8,8,0,0,0,11.32-11.32Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M154.64,26.61a6,6,0,0,0-6.32.65L77.94,82H32A14,14,0,0,0,18,96v64a14,14,0,0,0,14,14H77.94l70.38,54.74A6,6,0,0,0,158,224V32A6,6,0,0,0,154.64,26.61ZM30,160V96a2,2,0,0,1,2-2H74v68H32A2,2,0,0,1,30,160Zm116,51.73L86,165.06V90.94l60-46.67Zm98.24-64a6,6,0,1,1-8.48,8.48L216,136.48l-19.76,19.76a6,6,0,0,1-8.48-8.48L207.52,128l-19.76-19.76a6,6,0,0,1,8.48-8.48L216,119.52l19.76-19.76a6,6,0,0,1,8.48,8.48L224.48,128Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M155.51,24.81a8,8,0,0,0-8.42.88L77.25,80H32A16,16,0,0,0,16,96v64a16,16,0,0,0,16,16H77.25l69.84,54.31A8,8,0,0,0,160,224V32A8,8,0,0,0,155.51,24.81ZM32,96H72v64H32ZM144,207.64,88,164.09V91.91l56-43.55Zm101.66-61.3a8,8,0,0,1-11.32,11.32L216,139.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L204.69,128l-18.35-18.34a8,8,0,0,1,11.32-11.32L216,116.69l18.34-18.35a8,8,0,0,1,11.32,11.32L227.31,128Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M153.76,28.41a4,4,0,0,0-4.22.43L78.63,84H32A12,12,0,0,0,20,96v64a12,12,0,0,0,12,12H78.63l70.91,55.16A4.07,4.07,0,0,0,152,228a3.92,3.92,0,0,0,1.76-.41A4,4,0,0,0,156,224V32A4,4,0,0,0,153.76,28.41ZM28,160V96a4,4,0,0,1,4-4H76v72H32A4,4,0,0,1,28,160Zm120,55.82L84,166V90l64-49.78Zm94.83-66.65a4,4,0,0,1-5.66,5.66L216,133.66l-21.17,21.17a4,4,0,0,1-5.66-5.66L210.34,128l-21.17-21.17a4,4,0,0,1,5.66-5.66L216,122.34l21.17-21.17a4,4,0,1,1,5.66,5.66L221.66,128Z"></path>
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
