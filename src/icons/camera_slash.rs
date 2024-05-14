//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media", feature = "system"))]
#[component]
pub fn CameraSlash(
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
                <path d="M53.92,34.62A8,8,0,1,0,42.08,45.38L51.73,56H48A24,24,0,0,0,24,80V192a24,24,0,0,0,24,24H197.19l4.89,5.38a8,8,0,1,0,11.84-10.76ZM128,168a36,36,0,0,1-27.88-58.77L148,161.92A35.72,35.72,0,0,1,128,168ZM232,80V186a8,8,0,0,1-13.92,5.38l-130-143a8,8,0,0,1-.74-9.81l2-3A8,8,0,0,1,96,32h64a8,8,0,0,1,6.66,3.56L180.28,56H208A24,24,0,0,1,232,80Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,64H176L160,40H96L80,64H48A16,16,0,0,0,32,80V192a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V80A16,16,0,0,0,208,64ZM128,168a36,36,0,1,1,36-36A36,36,0,0,1,128,168Z"
        opacity="0.2"
    ></path>
    <path d="M53.92,34.62A8,8,0,1,0,42.08,45.38L51.73,56H48A24,24,0,0,0,24,80V192a24,24,0,0,0,24,24H197.19l4.89,5.38a8,8,0,1,0,11.84-10.76Zm51.66,80.61,37,40.69A27.71,27.71,0,0,1,128,160a28,28,0,0,1-22.42-44.77ZM48,200a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H66.28l28.41,31.26A44,44,0,0,0,128,176a44.21,44.21,0,0,0,25.44-8.12L182.64,200ZM232,80V186a8,8,0,0,1-16,0V80a8,8,0,0,0-8-8H176a8,8,0,0,1-6.65-3.56L155.71,48H100.24a8,8,0,0,1-12.91-9.42l2-3A8,8,0,0,1,96,32h64a8,8,0,0,1,6.66,3.56L180.28,56H208A24,24,0,0,1,232,80Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M51,37.31A4,4,0,0,0,45,42.69L60.78,60H48A20,20,0,0,0,28,80V192a20,20,0,0,0,20,20H199l6.08,6.69a4,4,0,1,0,5.92-5.38Zm97.58,119.23A31.69,31.69,0,0,1,128,164a32,32,0,0,1-22.48-54.78ZM48,204a12,12,0,0,1-12-12V80A12,12,0,0,1,48,68h20l32.09,35.3a40,40,0,0,0,53.79,59.16L191.69,204ZM228,80V186a4,4,0,0,1-8,0V80a12,12,0,0,0-12-12H176a4,4,0,0,1-3.32-1.78L157.85,44H98.13l-.82,1.23a4,4,0,1,1-6.65-4.44l2-3A4,4,0,0,1,96,36h64a4,4,0,0,1,3.33,1.78L178.13,60H208A20,20,0,0,1,228,80Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M56.88,31.93A12,12,0,1,0,39.12,48.07l4,4.37A28,28,0,0,0,20,80V192a28,28,0,0,0,28,28H195.42l3.7,4.07a12,12,0,0,0,17.76-16.14Zm49.35,90L136,154.64a24,24,0,0,1-29.77-32.75ZM48,196a4,4,0,0,1-4-4V80a4,4,0,0,1,4-4H64.51l24.92,27.41a48,48,0,0,0,63.34,69.67L173.6,196ZM236,80v94.14a12,12,0,0,1-24,0V80a4,4,0,0,0-4-4H176a12,12,0,0,1-10-5.34L153.57,52H102.06a12,12,0,0,1,0-24H160a12,12,0,0,1,10,5.34L182.42,52H208A28,28,0,0,1,236,80Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M52.44,36A6,6,0,0,0,43.56,44L56.25,58H48A22,22,0,0,0,26,80V192a22,22,0,0,0,22,22H198.07l5.49,6a6,6,0,0,0,8.88-8.08Zm53.06,76.2,40.11,44.13A29.67,29.67,0,0,1,128,162a30,30,0,0,1-22.5-49.84ZM48,202a10,10,0,0,1-10-10V80A10,10,0,0,1,48,70H67.16l30.23,33.25a42,42,0,0,0,56.33,62L187.16,202ZM230,80V186a6,6,0,0,1-12,0V80a10,10,0,0,0-10-10H176a6,6,0,0,1-5-2.67L156.78,46H99.21l-.23.34a6,6,0,0,1-10-6.65l2-3A6,6,0,0,1,96,34h64a6,6,0,0,1,5,2.67L179.21,58H208A22,22,0,0,1,230,80Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M53.92,34.62A8,8,0,1,0,42.08,45.38L51.73,56H48A24,24,0,0,0,24,80V192a24,24,0,0,0,24,24H197.19l4.89,5.38a8,8,0,1,0,11.84-10.76Zm51.66,80.61,37,40.69A27.71,27.71,0,0,1,128,160a28,28,0,0,1-22.42-44.77ZM48,200a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H66.28l28.41,31.26A44,44,0,0,0,128,176a44.21,44.21,0,0,0,25.44-8.12L182.64,200ZM232,80V186a8,8,0,0,1-16,0V80a8,8,0,0,0-8-8H176a8,8,0,0,1-6.65-3.56L155.71,48H100.24a8,8,0,0,1-12.91-9.42l2-3A8,8,0,0,1,96,32h64a8,8,0,0,1,6.66,3.56L180.28,56H208A24,24,0,0,1,232,80Z"></path>
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
