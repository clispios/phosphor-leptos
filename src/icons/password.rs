//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn Password(
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
                <path d="M48,56V200a12,12,0,0,1-24,0V56a12,12,0,0,1,24,0Zm82.73,50.7L116,111.48V96a12,12,0,0,0-24,0v15.48L77.27,106.7a12,12,0,1,0-7.41,22.82l14.72,4.79-9.1,12.52A12,12,0,1,0,94.9,160.94l9.1-12.52,9.1,12.52a12,12,0,1,0,19.42-14.11l-9.1-12.52,14.72-4.79a12,12,0,1,0-7.41-22.82Zm111.12,7.7a12,12,0,0,0-15.12-7.7L212,111.48V96a12,12,0,0,0-24,0v15.48l-14.73-4.78a12,12,0,1,0-7.41,22.82l14.72,4.79-9.1,12.52a12,12,0,1,0,19.42,14.11l9.1-12.52,9.1,12.52a12,12,0,1,0,19.42-14.11l-9.1-12.52,14.72-4.79A12,12,0,0,0,241.85,114.4Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M248,64V192a8,8,0,0,1-8,8H40V56H240A8,8,0,0,1,248,64Z" opacity="0.2"></path>
    <path d="M48,56V200a8,8,0,0,1-16,0V56a8,8,0,0,1,16,0Zm84,54.5L112,117V96a8,8,0,0,0-16,0v21L76,110.5a8,8,0,0,0-5,15.22l20,6.49-12.34,17a8,8,0,1,0,12.94,9.4l12.34-17,12.34,17a8,8,0,1,0,12.94-9.4l-12.34-17,20-6.49A8,8,0,0,0,132,110.5ZM238,115.64A8,8,0,0,0,228,110.5L208,117V96a8,8,0,0,0-16,0v21l-20-6.49a8,8,0,0,0-4.95,15.22l20,6.49-12.34,17a8,8,0,1,0,12.94,9.4l12.34-17,12.34,17a8,8,0,1,0,12.94-9.4l-12.34-17,20-6.49A8,8,0,0,0,238,115.64Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M216,48H40A16,16,0,0,0,24,64V192a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V64A16,16,0,0,0,216,48ZM64,168a8,8,0,0,1-16,0V88a8,8,0,0,1,16,0Zm73.3-39.81-12.36,4,7.64,10.5a8,8,0,1,1-13,9.41L112,141.61l-7.63,10.51a8,8,0,1,1-13-9.41l7.64-10.5-12.36-4a8,8,0,1,1,5-15.21L104,117V104a8,8,0,0,1,16,0v13l12.35-4a8,8,0,1,1,5,15.21Zm72,0-12.36,4,7.64,10.5a8,8,0,1,1-13,9.41L184,141.61l-7.63,10.51a8,8,0,1,1-13-9.41l7.64-10.5-12.36-4a8,8,0,1,1,5-15.21L176,117V104a8,8,0,0,1,16,0v13l12.35-4a8,8,0,0,1,5,15.21Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M46,56V200a6,6,0,0,1-12,0V56a6,6,0,0,1,12,0Zm86.58,56.41L110,119.74V96a6,6,0,0,0-12,0v23.74l-22.58-7.33a6,6,0,1,0-3.71,11.41l22.58,7.33-14,19.21a6,6,0,1,0,9.7,7.06l14-19.21,14,19.21a6,6,0,0,0,9.7-7.06l-14-19.21,22.58-7.33a6,6,0,1,0-3.71-11.41Zm103.56,3.85a6,6,0,0,0-7.56-3.85L206,119.74V96a6,6,0,0,0-12,0v23.74l-22.58-7.33a6,6,0,1,0-3.71,11.41l22.58,7.33-13.95,19.21a6,6,0,1,0,9.7,7.06l14-19.21,14,19.21a6,6,0,0,0,9.7-7.06l-13.95-19.21,22.58-7.33A6,6,0,0,0,236.14,116.26Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M48,56V200a8,8,0,0,1-16,0V56a8,8,0,0,1,16,0Zm84,54.5L112,117V96a8,8,0,0,0-16,0v21L76,110.5a8,8,0,0,0-5,15.22l20,6.49-12.34,17a8,8,0,1,0,12.94,9.4l12.34-17,12.34,17a8,8,0,1,0,12.94-9.4l-12.34-17,20-6.49A8,8,0,0,0,132,110.5ZM238,115.64A8,8,0,0,0,228,110.5L208,117V96a8,8,0,0,0-16,0v21l-20-6.49a8,8,0,0,0-4.95,15.22l20,6.49-12.34,17a8,8,0,1,0,12.94,9.4l12.34-17,12.34,17a8,8,0,1,0,12.94-9.4l-12.34-17,20-6.49A8,8,0,0,0,238,115.64Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M44,56V200a4,4,0,0,1-8,0V56a4,4,0,0,1,8,0Zm89.2,58.31L108,122.49V96a4,4,0,0,0-8,0v26.49l-25.2-8.18a4,4,0,1,0-2.47,7.61l25.2,8.18L82,151.54a4,4,0,1,0,6.47,4.7L104,134.81l15.57,21.43a4,4,0,0,0,3.24,1.65,4,4,0,0,0,3.23-6.35L110.47,130.1l25.2-8.18a4,4,0,0,0-2.47-7.61Zm101,2.57a4,4,0,0,0-5-2.57L204,122.49V96a4,4,0,0,0-8,0v26.49l-25.2-8.18a4,4,0,0,0-2.47,7.61l25.2,8.18L178,151.54a4,4,0,1,0,6.47,4.7L200,134.81l15.57,21.43a4,4,0,0,0,3.24,1.65,4,4,0,0,0,3.23-6.35L206.47,130.1l25.2-8.18A4,4,0,0,0,234.24,116.88Z"></path>
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
