//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce"))]
#[component]
pub fn Trademark(
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
                <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm0,192a84,84,0,1,1,84-84A84.09,84.09,0,0,1,128,212Zm64-104v40a12,12,0,0,1-24,0V134.09l-8.19,7a12,12,0,0,1-15.62,0l-8.19-7V148a12,12,0,0,1-24,0V120H100v28a12,12,0,0,1-24,0V120H72a12,12,0,0,1,0-24h52a12,12,0,0,1,7.81,2.89L152,116.2l20.19-17.31A12,12,0,0,1,192,108Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,128a96,96,0,1,1-96-96A96,96,0,0,1,224,128Z" opacity="0.2"></path>
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216ZM112,104a8,8,0,0,1-8,8H96v40a8,8,0,0,1-16,0V112H72a8,8,0,0,1,0-16h32A8,8,0,0,1,112,104Zm80,0v48a8,8,0,0,1-16,0V125.29l-14,16a8,8,0,0,1-12,0l-14-16V152a8,8,0,0,1-16,0V104a8,8,0,0,1,14-5.27l22,25.12,22-25.12A8,8,0,0,1,192,104Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm-24,88H96v40a8,8,0,0,1-16,0V112H72a8,8,0,0,1,0-16h32a8,8,0,0,1,0,16Zm88,40a8,8,0,0,1-16,0V125.29l-14,16a8,8,0,0,1-12,0l-14-16V152a8,8,0,0,1-16,0V104a8,8,0,0,1,14-5.27l22,25.12,22-25.12A8,8,0,0,1,192,104Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218ZM110,104a6,6,0,0,1-6,6H94v42a6,6,0,0,1-12,0V110H72a6,6,0,0,1,0-12h32A6,6,0,0,1,110,104Zm80,0v48a6,6,0,0,1-12,0V120l-17.48,20a6,6,0,0,1-9,0L134,120v32a6,6,0,0,1-12,0V104a6,6,0,0,1,10.52-4L156,126.89l23.48-26.84A6,6,0,0,1,190,104Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216ZM112,104a8,8,0,0,1-8,8H96v40a8,8,0,0,1-16,0V112H72a8,8,0,0,1,0-16h32A8,8,0,0,1,112,104Zm80,0v48a8,8,0,0,1-16,0V125.29l-14,16a8,8,0,0,1-12,0l-14-16V152a8,8,0,0,1-16,0V104a8,8,0,0,1,14-5.27l22,25.12,22-25.12A8,8,0,0,1,192,104Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm0,192a92,92,0,1,1,92-92A92.1,92.1,0,0,1,128,220ZM108,104a4,4,0,0,1-4,4H92v44a4,4,0,0,1-8,0V108H72a4,4,0,0,1,0-8h32A4,4,0,0,1,108,104Zm80,0v48a4,4,0,0,1-8,0V114.65l-21,24a4,4,0,0,1-6,0l-21-24V152a4,4,0,0,1-8,0V104a4,4,0,0,1,7-2.63l25,28.56,25-28.56a4,4,0,0,1,7,2.63Z"></path>
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
