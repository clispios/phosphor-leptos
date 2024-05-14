//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "finance"))]
#[component]
pub fn CurrencyKrw(
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
                <path d="M128,110.78,134.46,128H121.54ZM92,161.22,98.46,144H85.54ZM232,128A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128Zm-16,8a8,8,0,0,0-8-8H193.54l14-37.19a8,8,0,0,0-15-5.62l-16,42.81H151.54l-16-42.81a8,8,0,0,0-15,0L104.46,128H79.54L63.49,85.19a8,8,0,1,0-15,5.62l14,37.19H48a8,8,0,0,0,0,16H68.46l16.05,42.81a8,8,0,0,0,15,0L115.54,144h24.92l16,42.81a8,8,0,0,0,15,0l16-42.81H208A8,8,0,0,0,216,136Zm-52,25.22L170.46,144H157.54Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M50,136h52L76,200Zm52,0h52L128,72Zm52,0,26,64,26-64Z" opacity="0.2"></path>
    <path d="M240,128H217.89l21.52-53a8,8,0,1,0-14.82-6l-24,59H159.38l-24-59a8,8,0,0,0-14.82,0l-24,59H55.38l-24-59a8,8,0,0,0-14.82,6l21.52,53H16a8,8,0,0,0,0,16H44.61l24,59a8,8,0,0,0,14.82,0l24-59h41.24l24,59a8,8,0,0,0,14.82,0l24-59H240a8,8,0,0,0,0-16ZM76,178.75,61.88,144H90.12ZM113.88,128,128,93.26,142.12,128ZM180,178.75,165.88,144h28.24Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M240,132H211.94l23.77-58.49a4,4,0,1,0-7.42-3l-25,61.51H156.69l-25-61.51a4,4,0,0,0-7.42,0L99.31,132H52.69l-25-61.51a4,4,0,0,0-7.42,3L44.06,132H16a4,4,0,0,0,0,8H47.31l25,61.51a4,4,0,0,0,7.42,0l25-61.51h46.62l25,61.51a4,4,0,0,0,7.42,0l25-61.51H240a4,4,0,0,0,0-8ZM76,189.37,55.94,140H96.06ZM107.94,132,128,82.63,148.06,132ZM180,189.37,159.94,140h40.12Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M240,124H223.83l19.29-47.48a12,12,0,0,0-22.24-9l-23,56.51H162.08l-23-56.51a12,12,0,0,0-22.24,0L93.92,124H58.08l-23-56.51a12,12,0,0,0-22.24,9L32.17,124H16a12,12,0,0,0,0,24H41.92l23,56.52a12,12,0,0,0,22.24,0l23-56.52h35.84l23,56.52a12,12,0,0,0,22.24,0l23-56.52H240a12,12,0,0,0,0-24ZM76,168.12,67.83,148H84.17ZM119.83,124,128,103.89,136.17,124ZM180,168.12,171.83,148h16.34Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M240,130H214.91l22.65-55.74a6,6,0,0,0-11.12-4.52L202,130H158L133.56,69.74a6,6,0,0,0-11.12,0L98,130H54L29.56,69.74a6,6,0,1,0-11.12,4.52L41.09,130H16a6,6,0,0,0,0,12H46l24.48,60.26a6,6,0,0,0,11.12,0L106,142H150l24.48,60.26a6,6,0,0,0,11.12,0L210,142h30a6,6,0,0,0,0-12ZM76,184.06,58.91,142H93.09ZM110.91,130,128,87.94,145.09,130ZM180,184.06,162.91,142h34.18Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,128H217.89l21.52-53a8,8,0,1,0-14.82-6l-24,59H159.38l-24-59a8,8,0,0,0-14.82,0l-24,59H55.38l-24-59a8,8,0,0,0-14.82,6l21.52,53H16a8,8,0,0,0,0,16H44.61l24,59a8,8,0,0,0,14.82,0l24-59h41.24l24,59a8,8,0,0,0,14.82,0l24-59H240a8,8,0,0,0,0-16ZM76,178.75,61.88,144H90.12ZM113.88,128,128,93.26,142.12,128ZM180,178.75,165.88,144h28.24Z"></path>
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
