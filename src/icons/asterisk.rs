//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any(feature ="communication"))]
#[component]
pub fn Asterisk(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm59.43,129.07a8,8,0,0,1-4,14.93,7.92,7.92,0,0,1-4-1.07L136,141.86V192a8,8,0,0,1-16,0V141.86L76.57,166.93A8,8,0,0,1,65.65,164a8,8,0,0,1,2.92-10.93L112,128,68.57,102.93a8,8,0,0,1,8-13.86L120,114.14V64a8,8,0,0,1,16,0v50.14l43.43-25.07a8,8,0,0,1,8,13.86L144,128Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M200,128a72,72,0,1,1-72-72A72,72,0,0,1,200,128Z" opacity="0.2"></path>
    <path d="M214.86,180.12a8,8,0,0,1-11,2.74L136,142.13V216a8,8,0,0,1-16,0V142.13L52.12,182.86a8,8,0,1,1-8.23-13.72L112.45,128,43.89,86.86a8,8,0,1,1,8.23-13.72L120,113.87V40a8,8,0,0,1,16,0v73.87l67.88-40.73a8,8,0,1,1,8.23,13.72L143.55,128l68.56,41.14A8,8,0,0,1,214.86,180.12Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M211.43,178.06a4,4,0,0,1-5.49,1.37L132,135.06V216a4,4,0,0,1-8,0V135.06L50.06,179.43a4,4,0,0,1-4.12-6.86L120.22,128,45.94,83.43a4,4,0,0,1,4.12-6.86L124,120.94V40a4,4,0,0,1,8,0v80.94l73.94-44.37a4,4,0,1,1,4.12,6.86L135.78,128l74.28,44.57A4,4,0,0,1,211.43,178.06Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M218.29,182.17a12,12,0,0,1-16.47,4.12L140,149.19V216a12,12,0,0,1-24,0V149.19l-61.82,37.1a12,12,0,1,1-12.35-20.58L104.68,128,41.83,90.29A12,12,0,1,1,54.18,69.71L116,106.81V40a12,12,0,0,1,24,0v66.81l61.82-37.1a12,12,0,1,1,12.35,20.58L151.32,128l62.85,37.71A12,12,0,0,1,218.29,182.17Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M213.14,179.09a6,6,0,0,1-8.23,2.06L134,138.6V216a6,6,0,0,1-12,0V138.6L51.09,181.15A6.07,6.07,0,0,1,48,182a6,6,0,0,1-3.1-11.15L116.34,128,44.91,85.15a6,6,0,0,1,6.18-10.3L122,117.4V40a6,6,0,0,1,12,0v77.4l70.91-42.55a6,6,0,0,1,6.18,10.3L139.66,128l71.43,42.85A6,6,0,0,1,213.14,179.09Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M214.86,180.12a8,8,0,0,1-11,2.74L136,142.13V216a8,8,0,0,1-16,0V142.13L52.12,182.86a8,8,0,1,1-8.23-13.72L112.45,128,43.89,86.86a8,8,0,1,1,8.23-13.72L120,113.87V40a8,8,0,0,1,16,0v73.87l67.88-40.73a8,8,0,1,1,8.23,13.72L143.55,128l68.56,41.14A8,8,0,0,1,214.86,180.12Z"></path>
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