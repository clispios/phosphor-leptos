//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "office", feature = "media", feature = "objects"))]
#[component]
pub fn BookOpen(
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
                <path d="M240,64V192a16,16,0,0,1-16,16H160a24,24,0,0,0-24,24,8,8,0,0,1-16,0,24,24,0,0,0-24-24H32a16,16,0,0,1-16-16V64A16,16,0,0,1,32,48H88a32,32,0,0,1,32,32v88a8,8,0,0,0,16,0V80a32,32,0,0,1,32-32h56A16,16,0,0,1,240,64Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,64V192a8,8,0,0,1-8,8H160a32,32,0,0,0-32,32,32,32,0,0,0-32-32H32a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H96a32,32,0,0,1,32,32,32,32,0,0,1,32-32h64A8,8,0,0,1,232,64Z"
        opacity="0.2"
    ></path>
    <path d="M224,48H160a40,40,0,0,0-32,16A40,40,0,0,0,96,48H32A16,16,0,0,0,16,64V192a16,16,0,0,0,16,16H96a24,24,0,0,1,24,24,8,8,0,0,0,16,0,24,24,0,0,1,24-24h64a16,16,0,0,0,16-16V64A16,16,0,0,0,224,48ZM96,192H32V64H96a24,24,0,0,1,24,24V200A39.81,39.81,0,0,0,96,192Zm128,0H160a39.81,39.81,0,0,0-24,8V88a24,24,0,0,1,24-24h64Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M224,52H160a36,36,0,0,0-32,19.54A36,36,0,0,0,96,52H32A12,12,0,0,0,20,64V192a12,12,0,0,0,12,12H96a28,28,0,0,1,28,28,4,4,0,0,0,8,0,28,28,0,0,1,28-28h64a12,12,0,0,0,12-12V64A12,12,0,0,0,224,52ZM96,196H32a4,4,0,0,1-4-4V64a4,4,0,0,1,4-4H96a28,28,0,0,1,28,28V209.4A35.93,35.93,0,0,0,96,196Zm132-4a4,4,0,0,1-4,4H160a35.94,35.94,0,0,0-28,13.41V88a28,28,0,0,1,28-28h64a4,4,0,0,1,4,4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M224,44H160a43.86,43.86,0,0,0-32,13.85A43.86,43.86,0,0,0,96,44H32A20,20,0,0,0,12,64V192a20,20,0,0,0,20,20H96a20,20,0,0,1,20,20,12,12,0,0,0,24,0,20,20,0,0,1,20-20h64a20,20,0,0,0,20-20V64A20,20,0,0,0,224,44ZM96,188H36V68H96a20,20,0,0,1,20,20V192.81A43.79,43.79,0,0,0,96,188Zm124,0H160a43.71,43.71,0,0,0-20,4.83V88a20,20,0,0,1,20-20h60Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M224,50H160a38,38,0,0,0-32,17.55A38,38,0,0,0,96,50H32A14,14,0,0,0,18,64V192a14,14,0,0,0,14,14H96a26,26,0,0,1,26,26,6,6,0,0,0,12,0,26,26,0,0,1,26-26h64a14,14,0,0,0,14-14V64A14,14,0,0,0,224,50ZM96,194H32a2,2,0,0,1-2-2V64a2,2,0,0,1,2-2H96a26,26,0,0,1,26,26V204.31A37.86,37.86,0,0,0,96,194Zm130-2a2,2,0,0,1-2,2H160a37.87,37.87,0,0,0-26,10.32V88a26,26,0,0,1,26-26h64a2,2,0,0,1,2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,48H160a40,40,0,0,0-32,16A40,40,0,0,0,96,48H32A16,16,0,0,0,16,64V192a16,16,0,0,0,16,16H96a24,24,0,0,1,24,24,8,8,0,0,0,16,0,24,24,0,0,1,24-24h64a16,16,0,0,0,16-16V64A16,16,0,0,0,224,48ZM96,192H32V64H96a24,24,0,0,1,24,24V200A39.81,39.81,0,0,0,96,192Zm128,0H160a39.81,39.81,0,0,0-24,8V88a24,24,0,0,1,24-24h64Z"></path>
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
