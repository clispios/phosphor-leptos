//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "brand", feature = "communication", feature = "office"))]
#[component]
pub fn MicrosoftOutlookLogo(
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
                <path d="M88,144a16,16,0,1,1,16-16A16,16,0,0,1,88,144Zm144-24v88a16,16,0,0,1-5,11.61,8.8,8.8,0,0,1-.72.65A15.93,15.93,0,0,1,216,224H88a16,16,0,0,1-16-16V192H40a16,16,0,0,1-16-16V80A16,16,0,0,1,40,64H96V48a16,16,0,0,1,16-16h80a16,16,0,0,1,16,16v56h8a15.94,15.94,0,0,1,10,3.54l.08.07.14.11a6.78,6.78,0,0,1,.74.68A15.93,15.93,0,0,1,232,120ZM112,64h24a16,16,0,0,1,16,16v74.13l40-28.89V48H112ZM88,160a32,32,0,1,0-32-32A32,32,0,0,0,88,160Zm111.26,48L152,173.87V176a16,16,0,0,1-16,16H88v16ZM216,127.65,165.66,164,216,200.35Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M144,80v96a8,8,0,0,1-8,8H40a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8h96A8,8,0,0,1,144,80Z"
        opacity="0.2"
    ></path>
    <path d="M88,96a32,32,0,1,0,32,32A32,32,0,0,0,88,96Zm0,48a16,16,0,1,1,16-16A16,16,0,0,1,88,144Zm128-40h-8V48a16,16,0,0,0-16-16H112A16,16,0,0,0,96,48V64H40A16,16,0,0,0,24,80v96a16,16,0,0,0,16,16H72v16a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V120A16,16,0,0,0,216,104ZM112,48h80v77.24l-40,28.89V80a16,16,0,0,0-16-16H112ZM40,176V80h96v96H40Zm48,32V192h48a16,16,0,0,0,16-16v-2.13L199.26,208Zm128-7.65L165.66,164,216,127.65Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M88,100a28,28,0,1,0,28,28A28,28,0,0,0,88,100Zm0,48a20,20,0,1,1,20-20A20,20,0,0,1,88,148Zm128-40H204V48a12,12,0,0,0-12-12H112a12,12,0,0,0-12,12V68H40A12,12,0,0,0,28,80v96a12,12,0,0,0,12,12H76v20a12,12,0,0,0,12,12H216a12,12,0,0,0,12-12V120A12,12,0,0,0,216,108Zm-57.17,56L220,119.84a.78.78,0,0,1,0,.16v88a1,1,0,0,1,0,.17Zm52.8-48L204,121.51V116ZM108,48a4,4,0,0,1,4-4h80a4,4,0,0,1,4,4v79.29l-44,31.78-4-2.89V80a12,12,0,0,0-12-12H108ZM36,176V80a4,4,0,0,1,4-4h96a4,4,0,0,1,4,4v96a4,4,0,0,1-4,4H40A4,4,0,0,1,36,176Zm48,32V188h52a12,12,0,0,0,12-12v-9.95L211.63,212H88A4,4,0,0,1,84,208Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M88,92a36,36,0,1,0,36,36A36,36,0,0,0,88,92Zm0,48a12,12,0,1,1,12-12A12,12,0,0,1,88,140Zm128-40h-4V40a20,20,0,0,0-20-20H112A20,20,0,0,0,92,40V56H36A20,20,0,0,0,16,76V180a20,20,0,0,0,20,20H68v16a20,20,0,0,0,20,20H216a20,20,0,0,0,20-20V120A20,20,0,0,0,216,100Zm-44.45,68L212,136.54v62.92ZM116,44h72v80.8l-28,21.78V76a20,20,0,0,0-20-20H116ZM40,80h96v96H40ZM92,200h48a20,20,0,0,0,18.28-11.92L189,212H92Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M118,128a30,30,0,1,0-30,30A30,30,0,0,0,118,128Zm-48,0a18,18,0,1,1,18,18A18,18,0,0,1,70,128Zm146-22H206V48a14,14,0,0,0-14-14H112A14,14,0,0,0,98,48V66H40A14,14,0,0,0,26,80v96a14,14,0,0,0,14,14H74v18a14,14,0,0,0,14,14H216a14,14,0,0,0,14-14V120A14,14,0,0,0,216,106Zm-53.75,58L218,123.73v80.54ZM110,48a2,2,0,0,1,2-2h80a2,2,0,0,1,2,2v78.27L152,156.6l-2-1.45V80a14,14,0,0,0-14-14H110ZM38,176V80a2,2,0,0,1,2-2h96a2,2,0,0,1,2,2v96a2,2,0,0,1-2,2H40A2,2,0,0,1,38,176Zm48,32V190h50a14,14,0,0,0,14-14v-6l55.44,40H88A2,2,0,0,1,86,208Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M88,96a32,32,0,1,0,32,32A32,32,0,0,0,88,96Zm0,48a16,16,0,1,1,16-16A16,16,0,0,1,88,144Zm128-40h-8V48a16,16,0,0,0-16-16H112A16,16,0,0,0,96,48V64H40A16,16,0,0,0,24,80v96a16,16,0,0,0,16,16H72v16a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V120A16,16,0,0,0,216,104ZM112,48h80v77.24l-40,28.89V80a16,16,0,0,0-16-16H112ZM40,176V80h96v96H40Zm48,32V192h48a16,16,0,0,0,16-16v-2.13L199.26,208Zm128-7.65L165.66,164,216,127.65Z"></path>
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
