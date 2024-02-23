//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "weather"))]
#[component]
pub fn Snowflake(
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
                <path d="M227.65,149.14a12,12,0,0,1-8.79,14.51l-20.67,5.08,5.4,20.16a12,12,0,0,1-23.18,6.22l-7.29-27.2L140,148.78V187l20.48,20.48a12,12,0,0,1-17,17L128,209l-15.51,15.52a12,12,0,0,1-17-17L116,187V148.78L82.88,167.91l-7.29,27.2a12,12,0,0,1-23.18-6.22l5.4-20.16-20.67-5.08a12,12,0,1,1,5.72-23.3l27.89,6.85L104,128,70.75,108.8l-27.89,6.85A11.8,11.8,0,0,1,40,116a12,12,0,0,1-2.85-23.65l20.67-5.08-5.4-20.16a12,12,0,0,1,23.18-6.22l7.29,27.2L116,107.21V69L95.52,48.48a12,12,0,0,1,17-17L128,47l15.51-15.52a12,12,0,1,1,17,17L140,69v38.24l33.12-19.12,7.29-27.2a12,12,0,0,1,23.18,6.22l-5.4,20.16,20.67,5.08A12,12,0,0,1,216,116a11.8,11.8,0,0,1-2.87-.35l-27.89-6.85L152,128l33.25,19.2,27.89-6.85A12,12,0,0,1,227.65,149.14Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M220,128a92,92,0,1,1-92-92A92.1,92.1,0,0,1,220,128Z" opacity="0.2"></path>
    <path d="M223.77,150.09a8,8,0,0,1-5.86,9.68l-24.64,6,6.46,24.11a8,8,0,0,1-5.66,9.8A8.25,8.25,0,0,1,192,200a8,8,0,0,1-7.72-5.93l-7.72-28.8L136,141.86v46.83l21.66,21.65a8,8,0,0,1-11.32,11.32L128,203.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L120,188.69V141.86L79.45,165.27l-7.72,28.8A8,8,0,0,1,64,200a8.25,8.25,0,0,1-2.08-.27,8,8,0,0,1-5.66-9.8l6.46-24.11-24.64-6a8,8,0,0,1,3.82-15.54l29.45,7.23L112,128,71.36,104.54l-29.45,7.23A7.85,7.85,0,0,1,40,112a8,8,0,0,1-1.91-15.77l24.64-6L56.27,66.07a8,8,0,0,1,15.46-4.14l7.72,28.8L120,114.14V67.31L98.34,45.66a8,8,0,0,1,11.32-11.32L128,52.69l18.34-18.35a8,8,0,0,1,11.32,11.32L136,67.31v46.83l40.55-23.41,7.72-28.8a8,8,0,0,1,15.46,4.14l-6.46,24.11,24.64,6A8,8,0,0,1,216,112a7.85,7.85,0,0,1-1.91-.23l-29.45-7.23L144,128l40.64,23.46,29.45-7.23A8,8,0,0,1,223.77,150.09Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm42.37,119.22,18.94-6.76a8,8,0,1,1,5.38,15.08l-15.48,5.52,4.52,16.87a8,8,0,0,1-5.66,9.8A8.23,8.23,0,0,1,176,184a8,8,0,0,1-7.73-5.93l-5.57-20.8L136,141.86v30.83l13.66,13.65a8,8,0,0,1-11.32,11.32L128,187.31l-10.34,10.35a8,8,0,0,1-11.32-11.32L120,172.69V141.86L93.3,157.27l-5.57,20.8A8,8,0,0,1,80,184a8.23,8.23,0,0,1-2.07-.27,8,8,0,0,1-5.66-9.8l4.52-16.87-15.48-5.52a8,8,0,0,1,5.38-15.08l18.94,6.76L112,128,85.63,112.78l-18.94,6.76A8.18,8.18,0,0,1,64,120a8,8,0,0,1-2.69-15.54l15.48-5.52L72.27,82.07a8,8,0,0,1,15.46-4.14l5.57,20.8L120,114.14V83.31L106.34,69.66a8,8,0,0,1,11.32-11.32L128,68.69l10.34-10.35a8,8,0,0,1,11.32,11.32L136,83.31v30.83l26.7-15.41,5.57-20.8a8,8,0,0,1,15.46,4.14l-4.52,16.87,15.48,5.52A8,8,0,0,1,192,120a8.18,8.18,0,0,1-2.69-.46l-18.94-6.76L144,128Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M221.83,150.57a6,6,0,0,1-4.4,7.26l-26.62,6.54,7,26.08a6,6,0,0,1-4.24,7.35,6.4,6.4,0,0,1-1.55.2,6,6,0,0,1-5.8-4.45L178.27,164,134,138.39v51.13l22.24,22.24a6,6,0,1,1-8.48,8.48L128,200.49l-19.76,19.75a6,6,0,0,1-8.48-8.48L122,189.52V138.39L77.73,164l-7.93,29.6A6,6,0,0,1,64,198a6.4,6.4,0,0,1-1.55-.2,6,6,0,0,1-4.24-7.35l7-26.08-26.62-6.54a6,6,0,0,1,2.86-11.66l30.23,7.43L116,128,71.66,102.4l-30.23,7.43A5.88,5.88,0,0,1,40,110a6,6,0,0,1-1.43-11.83l26.62-6.54-7-26.08a6,6,0,1,1,11.59-3.1l7.93,29.6L122,117.61V66.48L99.76,44.24a6,6,0,0,1,8.48-8.48L128,55.51l19.76-19.75a6,6,0,0,1,8.48,8.48L134,66.48v51.13l44.27-25.56,7.93-29.6a6,6,0,1,1,11.59,3.1l-7,26.08,26.62,6.54A6,6,0,0,1,216,110a5.88,5.88,0,0,1-1.43-.17l-30.23-7.43L140,128l44.34,25.6,30.23-7.43A6,6,0,0,1,221.83,150.57Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M223.77,150.09a8,8,0,0,1-5.86,9.68l-24.64,6,6.46,24.11a8,8,0,0,1-5.66,9.8A8.25,8.25,0,0,1,192,200a8,8,0,0,1-7.72-5.93l-7.72-28.8L136,141.86v46.83l21.66,21.65a8,8,0,0,1-11.32,11.32L128,203.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L120,188.69V141.86L79.45,165.27l-7.72,28.8A8,8,0,0,1,64,200a8.25,8.25,0,0,1-2.08-.27,8,8,0,0,1-5.66-9.8l6.46-24.11-24.64-6a8,8,0,0,1,3.82-15.54l29.45,7.23L112,128,71.36,104.54l-29.45,7.23A7.85,7.85,0,0,1,40,112a8,8,0,0,1-1.91-15.77l24.64-6L56.27,66.07a8,8,0,0,1,15.46-4.14l7.72,28.8L120,114.14V67.31L98.34,45.66a8,8,0,0,1,11.32-11.32L128,52.69l18.34-18.35a8,8,0,0,1,11.32,11.32L136,67.31v46.83l40.55-23.41,7.72-28.8a8,8,0,0,1,15.46,4.14l-6.46,24.11,24.64,6A8,8,0,0,1,216,112a7.85,7.85,0,0,1-1.91-.23l-29.45-7.23L144,128l40.64,23.46,29.45-7.23A8,8,0,0,1,223.77,150.09Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M219.88,151.05a4,4,0,0,1-2.93,4.83l-28.6,7L195.86,191a4,4,0,0,1-2.83,4.9,3.65,3.65,0,0,1-1,.14,4,4,0,0,1-3.86-3L180,162.64l-48-27.71v55.41l22.83,22.83a4,4,0,0,1-5.66,5.66L128,197.66l-21.17,21.17a4,4,0,0,1-5.66-5.66L124,190.34V134.93L76,162.64,67.86,193A4,4,0,0,1,64,196a3.65,3.65,0,0,1-1-.14,4,4,0,0,1-2.83-4.9l7.51-28.05-28.6-7A4,4,0,1,1,41,148.12l31,7.61L120,128,72,100.27l-31,7.61a4.07,4.07,0,0,1-1,.12,4,4,0,0,1-1-7.88l28.6-7L60.14,65A4,4,0,0,1,67.86,63L76,93.36l48,27.71V65.66L101.17,42.83a4,4,0,0,1,5.66-5.66L128,58.34l21.17-21.17a4,4,0,1,1,5.66,5.66L132,65.66v55.41l48-27.71L188.14,63A4,4,0,0,1,195.86,65l-7.51,28.05,28.6,7A4,4,0,0,1,216,108a4.07,4.07,0,0,1-1-.12l-31-7.61L136,128l48,27.73,31-7.61A4,4,0,0,1,219.88,151.05Z"></path>
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
