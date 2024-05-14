//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media"))]
#[component]
pub fn MusicNotesPlus(
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
                <path d="M232,56a8,8,0,0,1-8,8H208V80a8,8,0,0,1-16,0V64H176a8,8,0,0,1,0-16h16V32a8,8,0,0,1,16,0V48h16A8,8,0,0,1,232,56Zm-24,56a8,8,0,0,0-8,8v22.08A36,36,0,1,0,216,172V120A8,8,0,0,0,208,112Zm-54.42-10.67a8,8,0,0,0,2.76-9.88,8.11,8.11,0,0,0-1.1-1.79,55.78,55.78,0,0,1-11-39A8,8,0,0,0,137,42a7.9,7.9,0,0,0-2.61.21L78.06,56.24A8,8,0,0,0,72,64V174.08A36,36,0,1,0,88,204V118.25l62.82-15.71A8.06,8.06,0,0,0,153.58,101.33Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,172a28,28,0,1,1-28-28A28,28,0,0,1,208,172ZM52,176a28,28,0,1,0,28,28A28,28,0,0,0,52,176Z"
        opacity="0.2"
    ></path>
    <path d="M232,56a8,8,0,0,1-8,8H208V80a8,8,0,0,1-16,0V64H176a8,8,0,0,1,0-16h16V32a8,8,0,0,1,16,0V48h16A8,8,0,0,1,232,56ZM88,118.25V204a36,36,0,1,1-16-29.92V64a8,8,0,0,1,6.06-7.76l56-14a8,8,0,0,1,3.88,15.52L88,70.25v31.5l70.06-17.51a8,8,0,0,1,3.88,15.52ZM72,204a20,20,0,1,0-20,20A20,20,0,0,0,72,204Zm144-84v52a36,36,0,1,1-16-29.92V120a8,8,0,0,1,16,0Zm-16,52a20,20,0,1,0-20,20A20,20,0,0,0,200,172Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M228,56a4,4,0,0,1-4,4H204V80a4,4,0,0,1-8,0V60H176a4,4,0,0,1,0-8h20V32a4,4,0,0,1,8,0V52h20A4,4,0,0,1,228,56ZM84,115.12V204a32.06,32.06,0,1,1-8-21.13V64a4,4,0,0,1,3-3.88l56-14A4,4,0,0,1,137,53.88L84,67.12v39.76l75-18.76A4,4,0,0,1,161,95.88ZM76,204a24,24,0,1,0-24,24A24,24,0,0,0,76,204Zm136-84v52a32.06,32.06,0,1,1-8-21.13V120a4,4,0,0,1,8,0Zm-8,52a24,24,0,1,0-24,24A24,24,0,0,0,204,172Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M236,56a12,12,0,0,1-12,12H212V80a12,12,0,0,1-24,0V68H176a12,12,0,0,1,0-24h12V32a12,12,0,0,1,24,0V44h12A12,12,0,0,1,236,56Zm-16,68v48a40,40,0,1,1-24-36.65V124a12,12,0,0,1,24,0Zm-24,48a16,16,0,1,0-16,16A16,16,0,0,0,196,172ZM92,121.37V204a40,40,0,1,1-24-36.65V64a12,12,0,0,1,9.09-11.64l52-13a12,12,0,0,1,5.82,23.28L92,73.37V96.63l53.09-13.27a12,12,0,0,1,5.82,23.28ZM68,204a16,16,0,1,0-16,16A16,16,0,0,0,68,204Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M230,56a6,6,0,0,1-6,6H206V80a6,6,0,0,1-12,0V62H176a6,6,0,0,1,0-12h18V32a6,6,0,0,1,12,0V50h18A6,6,0,0,1,230,56ZM86,116.68V204a34.06,34.06,0,1,1-12-25.89V64a6,6,0,0,1,4.54-5.82l56-14a6,6,0,1,1,2.92,11.64L86,68.68v35.64l72.54-18.14a6,6,0,1,1,2.92,11.64ZM74,204a22,22,0,1,0-22,22A22,22,0,0,0,74,204Zm140-84v52a34.06,34.06,0,1,1-12-25.89V120a6,6,0,0,1,12,0Zm-12,52a22,22,0,1,0-22,22A22,22,0,0,0,202,172Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,56a8,8,0,0,1-8,8H208V80a8,8,0,0,1-16,0V64H176a8,8,0,0,1,0-16h16V32a8,8,0,0,1,16,0V48h16A8,8,0,0,1,232,56ZM88,118.25V204a36,36,0,1,1-16-29.92V64a8,8,0,0,1,6.06-7.76l56-14a8,8,0,0,1,3.88,15.52L88,70.25v31.5l70.06-17.51a8,8,0,0,1,3.88,15.52ZM72,204a20,20,0,1,0-20,20A20,20,0,0,0,72,204Zm144-84v52a36,36,0,1,1-16-29.92V120a8,8,0,0,1,16,0Zm-16,52a20,20,0,1,0-20,20A20,20,0,0,0,200,172Z"></path>
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
