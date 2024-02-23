//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn WarningDiamond(
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
                <path d="M128,68a12,12,0,0,1,12,12v52a12,12,0,0,1-24,0V80A12,12,0,0,1,128,68Zm0,88a16,16,0,1,0,16,16A16,16,0,0,0,128,156Zm116-28a19.86,19.86,0,0,1-5.84,14.11l-96,96.06a20,20,0,0,1-28.21,0h0l-96-96.06a20,20,0,0,1,0-28.22l96.05-96.06a20,20,0,0,1,28.21,0l96.06,96.06A19.86,19.86,0,0,1,244,128Zm-25.68,0L128,37.67,37.68,128,128,218.33Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M229.67,133.62l-96,96a7.94,7.94,0,0,1-11.24,0l-96-96a7.94,7.94,0,0,1,0-11.24l96.05-96a7.94,7.94,0,0,1,11.24,0l96,96.05A7.94,7.94,0,0,1,229.67,133.62Z"
        opacity="0.2"
    ></path>
    <path d="M128,72a8,8,0,0,1,8,8v56a8,8,0,0,1-16,0V80A8,8,0,0,1,128,72ZM116,172a12,12,0,1,0,12-12A12,12,0,0,0,116,172Zm124-44a15.85,15.85,0,0,1-4.67,11.28l-96.05,96.06a16,16,0,0,1-22.56,0h0l-96-96.06a16,16,0,0,1,0-22.56l96.05-96.06a16,16,0,0,1,22.56,0l96.05,96.06A15.85,15.85,0,0,1,240,128Zm-16,0L128,32,32,128,128,224h0Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M235.33,116.72,139.28,20.66a16,16,0,0,0-22.56,0l-96,96.06a16,16,0,0,0,0,22.56l96.05,96.06h0a16,16,0,0,0,22.56,0l96.05-96.06a16,16,0,0,0,0-22.56ZM120,80a8,8,0,0,1,16,0v56a8,8,0,0,1-16,0Zm8,104a12,12,0,1,1,12-12A12,12,0,0,1,128,184Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M122,136V80a6,6,0,0,1,12,0v56a6,6,0,0,1-12,0Zm6,26a10,10,0,1,0,10,10A10,10,0,0,0,128,162Zm110-34a13.82,13.82,0,0,1-4.09,9.86l-96,96.06a14,14,0,0,1-19.72,0h0l-96-96.06a13.93,13.93,0,0,1,0-19.72l96.05-96.06a14,14,0,0,1,19.72,0l96,96.06A13.82,13.82,0,0,1,238,128Zm-12,0a1.94,1.94,0,0,0-.57-1.38L129.38,30.56a2,2,0,0,0-2.76,0L30.57,126.62a2,2,0,0,0,0,2.76l96.05,96.06h0a2,2,0,0,0,2.76,0l96.05-96.06A1.94,1.94,0,0,0,226,128Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,72a8,8,0,0,1,8,8v56a8,8,0,0,1-16,0V80A8,8,0,0,1,128,72ZM116,172a12,12,0,1,0,12-12A12,12,0,0,0,116,172Zm124-44a15.85,15.85,0,0,1-4.67,11.28l-96.05,96.06a16,16,0,0,1-22.56,0h0l-96-96.06a16,16,0,0,1,0-22.56l96.05-96.06a16,16,0,0,1,22.56,0l96.05,96.06A15.85,15.85,0,0,1,240,128Zm-16,0L128,32,32,128,128,224h0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M124,136V80a4,4,0,0,1,8,0v56a4,4,0,0,1-8,0Zm4,28a8,8,0,1,0,8,8A8,8,0,0,0,128,164Zm108-36a11.87,11.87,0,0,1-3.5,8.45l-96.05,96.06a12,12,0,0,1-16.9,0h0l-96-96.06a12,12,0,0,1,0-16.9l96.05-96.06a12,12,0,0,1,16.9,0l96.05,96.06A11.87,11.87,0,0,1,236,128Zm-8,0a3.9,3.9,0,0,0-1.16-2.79L130.79,29.15a4,4,0,0,0-5.58,0l-96,96.06a3.94,3.94,0,0,0,0,5.58l96.05,96.06a4,4,0,0,0,5.58,0l96.05-96.06A3.9,3.9,0,0,0,228,128Z"></path>
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
