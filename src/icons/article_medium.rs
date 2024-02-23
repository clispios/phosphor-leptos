//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media", feature = "objects"))]
#[component]
pub fn ArticleMedium(
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
                <path d="M60,132a12,12,0,0,1-12,12H24a12,12,0,0,1,0-24h4V64H24a12,12,0,0,1,0-24H40a12,12,0,0,1,10.18,5.63L80,93.36l29.82-47.72A12,12,0,0,1,120,40h16a12,12,0,0,1,0,24h-4v56h4a12,12,0,0,1,0,24H112a12,12,0,0,1-4-23.3V93.84L90.18,122.36a12,12,0,0,1-20.36,0L52,93.84V120.7A12,12,0,0,1,60,132Zm116-28h64a12,12,0,0,0,0-24H176a12,12,0,0,0,0,24Zm64,16H176a12,12,0,0,0,0,24h64a12,12,0,0,0,0-24Zm0,40H72a12,12,0,0,0,0,24H240a12,12,0,0,0,0-24Zm0,40H72a12,12,0,0,0,0,24H240a12,12,0,0,0,0-24Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M240,104v96H72V168h96V104Z" opacity="0.2"></path>
    <path d="M56,136a8,8,0,0,1-8,8H24a8,8,0,0,1,0-16h8V64H24a8,8,0,0,1,0-16H40v0a8,8,0,0,1,6.78,3.74L80,104.91l33.22-53.15A8,8,0,0,1,120,48v0h16a8,8,0,0,1,0,16h-8v64h8a8,8,0,0,1,0,16H112a8,8,0,0,1,0-16V83.89L86.78,124.24a8,8,0,0,1-13.56,0L48,83.89V128A8,8,0,0,1,56,136Zm112-24h72a8,8,0,0,0,0-16H168a8,8,0,0,0,0,16Zm72,16H168a8,8,0,0,0,0,16h72a8,8,0,0,0,0-16Zm0,32H72a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16Zm0,32H72a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M56,144a8,8,0,0,1-8,8H24a8,8,0,0,1,0-16h8V72H24a8,8,0,0,1,0-16H40v0a8,8,0,0,1,6.78,3.74L80,112.91l33.22-53.15A8,8,0,0,1,120,56v0h16a8,8,0,0,1,0,16h-8v64h8a8,8,0,0,1,0,16H112a8,8,0,0,1,0-16V91.89L86.78,132.24a8,8,0,0,1-13.56,0L48,91.89V136A8,8,0,0,1,56,144Zm184-24H168a8,8,0,0,0-8,8v16a8,8,0,0,0,8,8h72a8,8,0,0,0,8-8V128A8,8,0,0,0,240,120Zm0,48H72a8,8,0,0,0-8,8v16a8,8,0,0,0,8,8H240a8,8,0,0,0,8-8V176A8,8,0,0,0,240,168Zm0-96H168a8,8,0,0,0-8,8V96a8,8,0,0,0,8,8h72a8,8,0,0,0,8-8V80A8,8,0,0,0,240,72Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M54,136a6,6,0,0,1-6,6H24a6,6,0,0,1,0-12H34V62H24a6,6,0,0,1,0-12H40v0a6,6,0,0,1,5.09,2.8L80,108.68l34.91-55.86A6,6,0,0,1,120,50v0h16a6,6,0,0,1,0,12H126v68h10a6,6,0,0,1,0,12H112a6,6,0,0,1,0-12h2V76.92L85.09,123.18a6,6,0,0,1-10.18,0L46,76.92V130h2A6,6,0,0,1,54,136Zm114-26h72a6,6,0,0,0,0-12H168a6,6,0,0,0,0,12Zm72,20H168a6,6,0,0,0,0,12h72a6,6,0,0,0,0-12Zm0,32H72a6,6,0,0,0,0,12H240a6,6,0,0,0,0-12Zm0,32H72a6,6,0,0,0,0,12H240a6,6,0,0,0,0-12Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M56,136a8,8,0,0,1-8,8H24a8,8,0,0,1,0-16h8V64H24a8,8,0,0,1,0-16H40v0a8,8,0,0,1,6.78,3.74L80,104.91l33.22-53.15A8,8,0,0,1,120,48v0h16a8,8,0,0,1,0,16h-8v64h8a8,8,0,0,1,0,16H112a8,8,0,0,1,0-16V83.89L86.78,124.24a8,8,0,0,1-13.56,0L48,83.89V128A8,8,0,0,1,56,136Zm112-24h72a8,8,0,0,0,0-16H168a8,8,0,0,0,0,16Zm72,16H168a8,8,0,0,0,0,16h72a8,8,0,0,0,0-16Zm0,32H72a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16Zm0,32H72a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M52,136a4,4,0,0,1-4,4H24a4,4,0,0,1,0-8H36V60H24a4,4,0,0,1,0-8H40a4,4,0,0,1,3.39,1.87L80,112.45l36.61-58.57A4,4,0,0,1,120,52h16a4,4,0,0,1,0,8H124v72h12a4,4,0,0,1,0,8H112a4,4,0,0,1,0-8h4V70L83.39,122.12a4,4,0,0,1-6.78,0L44,70V132h4A4,4,0,0,1,52,136Zm116-28h72a4,4,0,0,0,0-8H168a4,4,0,0,0,0,8Zm72,24H168a4,4,0,0,0,0,8h72a4,4,0,0,0,0-8Zm0,32H72a4,4,0,0,0,0,8H240a4,4,0,0,0,0-8Zm0,32H72a4,4,0,0,0,0,8H240a4,4,0,0,0,0-8Z"></path>
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
