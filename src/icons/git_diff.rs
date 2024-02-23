//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "development"))]
#[component]
pub fn GitDiff(
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
                <path d="M112,148a12,12,0,0,0-12,12v19L78.54,157.57A35.76,35.76,0,0,1,68,132.12V97.94a36,36,0,1,0-24,0v34.18a59.61,59.61,0,0,0,17.57,42.42L83,196H64a12,12,0,0,0,0,24h48a12,12,0,0,0,12-12V160A12,12,0,0,0,112,148ZM56,52A12,12,0,1,1,44,64,12,12,0,0,1,56,52ZM212,158.06V123.88a59.61,59.61,0,0,0-17.57-42.42L173,60h19a12,12,0,0,0,0-24H144a12,12,0,0,0-12,12V96a12,12,0,0,0,24,0V77l21.46,21.46A35.76,35.76,0,0,1,188,123.88v34.18a36,36,0,1,0,24,0ZM200,204a12,12,0,1,1,12-12A12,12,0,0,1,200,204Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M80,64A24,24,0,1,1,56,40,24,24,0,0,1,80,64ZM200,168a24,24,0,1,0,24,24A24,24,0,0,0,200,168Z"
        opacity="0.2"
    ></path>
    <path d="M112,152a8,8,0,0,0-8,8v28.69L75.72,160.4A39.71,39.71,0,0,1,64,132.12V95a32,32,0,1,0-16,0v37.13a55.67,55.67,0,0,0,16.4,39.6L92.69,200H64a8,8,0,0,0,0,16h48a8,8,0,0,0,8-8V160A8,8,0,0,0,112,152ZM40,64A16,16,0,1,1,56,80,16,16,0,0,1,40,64Zm168,97V123.88a55.67,55.67,0,0,0-16.4-39.6L163.31,56H192a8,8,0,0,0,0-16H144a8,8,0,0,0-8,8V96a8,8,0,0,0,16,0V67.31L180.28,95.6A39.71,39.71,0,0,1,192,123.88V161a32,32,0,1,0,16,0Zm-8,47a16,16,0,1,1,16-16A16,16,0,0,1,200,208Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M120,160v48a8,8,0,0,1-8,8H64a8,8,0,0,1,0-16H92.69L64.4,171.72A55.67,55.67,0,0,1,48,132.12V95a32,32,0,1,1,16,0v37.13A39.71,39.71,0,0,0,75.72,160.4L104,188.69V160a8,8,0,0,1,16,0Zm88,1V123.88a55.67,55.67,0,0,0-16.4-39.6L163.31,56H192a8,8,0,0,0,0-16H144a8,8,0,0,0-8,8V96a8,8,0,0,0,16,0V67.31L180.28,95.6A39.71,39.71,0,0,1,192,123.88V161a32,32,0,1,0,16,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M112,154a6,6,0,0,0-6,6v33.52l-31.7-31.7A41.75,41.75,0,0,1,62,132.12V93.4a30,30,0,1,0-12,0v38.72A53.65,53.65,0,0,0,65.82,170.3L97.52,202H64a6,6,0,0,0,0,12h48a6,6,0,0,0,6-6V160A6,6,0,0,0,112,154ZM38,64A18,18,0,1,1,56,82,18,18,0,0,1,38,64Zm168,98.6V123.88A53.65,53.65,0,0,0,190.18,85.7L158.48,54H192a6,6,0,0,0,0-12H144a6,6,0,0,0-6,6V96a6,6,0,0,0,12,0V62.48l31.7,31.7a41.75,41.75,0,0,1,12.3,29.7V162.6a30,30,0,1,0,12,0ZM200,210a18,18,0,1,1,18-18A18,18,0,0,1,200,210Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M112,152a8,8,0,0,0-8,8v28.69L75.72,160.4A39.71,39.71,0,0,1,64,132.12V95a32,32,0,1,0-16,0v37.13a55.67,55.67,0,0,0,16.4,39.6L92.69,200H64a8,8,0,0,0,0,16h48a8,8,0,0,0,8-8V160A8,8,0,0,0,112,152ZM40,64A16,16,0,1,1,56,80,16,16,0,0,1,40,64Zm168,97V123.88a55.67,55.67,0,0,0-16.4-39.6L163.31,56H192a8,8,0,0,0,0-16H144a8,8,0,0,0-8,8V96a8,8,0,0,0,16,0V67.31L180.28,95.6A39.71,39.71,0,0,1,192,123.88V161a32,32,0,1,0,16,0Zm-8,47a16,16,0,1,1,16-16A16,16,0,0,1,200,208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M112,156a4,4,0,0,0-4,4v38.34L72.89,163.23A43.71,43.71,0,0,1,60,132.12V91.71a28,28,0,1,0-8,0v40.41a51.66,51.66,0,0,0,15.23,36.77L102.34,204H64a4,4,0,0,0,0,8h48a4,4,0,0,0,4-4V160A4,4,0,0,0,112,156ZM36,64A20,20,0,1,1,56,84,20,20,0,0,1,36,64ZM204,164.29V123.88a51.66,51.66,0,0,0-15.23-36.77L153.66,52H192a4,4,0,0,0,0-8H144a4,4,0,0,0-4,4V96a4,4,0,0,0,8,0V57.66l35.11,35.11A43.71,43.71,0,0,1,196,123.88v40.41a28,28,0,1,0,8,0ZM200,212a20,20,0,1,1,20-20A20,20,0,0,1,200,212Z"></path>
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
