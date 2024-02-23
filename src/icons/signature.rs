//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "office"))]
#[component]
pub fn Signature(
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
                <path d="M232,164H70.35c2.2-4.42,4.41-8.94,6.59-13.53.24-.49.47-1,.7-1.46,13.56-1.6,28.78-11.26,45.52-28.9,5.33,12.06,14.07,24.87,27.65,27.44,5.25,1,13.49.79,22.9-5.81a57.35,57.35,0,0,0,9.08-8.06C191.12,141.3,205.87,148,232,148a12,12,0,0,0,0-24c-32.66,0-35.81-11.61-36-12.6a12,12,0,0,0-23.06-4c-9.37,13.93-15.6,17-17.66,16.6-4.91-.93-12.45-15.7-15.52-30.39a12,12,0,0,0-22.09-3.68c-10,12.38-18.56,20.93-25.68,26.5,21.31-53.19,17.15-72.52,11.47-82.57-5-8.87-14-13.78-25.36-13.83h-.32c-18,.13-32.43,16.66-38.73,44.27-3.45,15.13-4,31.88-1.46,45.94,2.64,14.69,8.33,25.67,16.61,32.22C50.63,149.84,47,157.09,43.45,164H24a12,12,0,0,0,0,24h6.68c-9.68,17.67-16.84,29.6-17,29.81a12,12,0,0,0,4.1,16.47A11.85,11.85,0,0,0,24,236a12,12,0,0,0,10.29-5.81C34.91,229.14,45.22,212,58,188H232a12,12,0,0,0,0-24Zm-36-52c0-.18,0-.36,0-.54A3.19,3.19,0,0,1,196,112ZM62.44,69.61C65.77,55,72.44,44,78,44c3.67,0,4.25,1,4.6,1.64,1.43,2.54,6.55,17.09-17.42,72.8A44.79,44.79,0,0,1,61.2,106,94.89,94.89,0,0,1,62.44,69.61Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M67.41,142.56C35.71,129.52,45.41,32.07,78,32,111.82,32.05,92,90.26,67.41,142.56Z"
        opacity="0.2"
    ></path>
    <path d="M232,168H63.86c2.66-5.24,5.33-10.63,8-16.11,14.94,1.65,32.62-8.8,52.74-31.22.6,1.66,1.27,3.37,2,5.1,6.51,15.25,14.91,23.94,25,25.85,10.34,2,20.58-3.23,31.08-15.82C189.5,143.87,203.5,152,232,152a8,8,0,0,0,0-16c-30.63,0-39.55-10.59-40-16.22a8,8,0,0,0-15.51-2.54c-12.17,18.25-19.38,19.14-22,18.66-8.33-1.57-16.08-20.93-18.69-33.51A8,8,0,0,0,121,100.16c-19.8,24.62-33.08,33-41.41,35.14,8.49-18.88,14.83-35.45,18.89-49.4,6.82-23.44,7.32-39.83,1.51-50.1-3-5.36-9.29-11.75-21.91-11.8h-.25c-16,.11-28.6,15.3-34.62,41.7-3.59,15.71-4.18,33.19-1.63,48s7.86,25.51,15.55,31.89c-3.72,7.73-7.53,15.28-11.23,22.43H24a8,8,0,0,0,0,16H37.41c-11.32,21-20.12,35.64-20.26,35.88a8,8,0,1,0,13.71,8.24c.15-.26,11.27-18.79,24.7-44.12H232a8,8,0,0,0,0-16Zm-40-48v-.21A1.11,1.11,0,0,1,192,120ZM58.79,69.26C62.78,51.78,70.48,40,78,40,83.25,40,85,41.86,86,43.67c3,5.33,6.52,24.19-21.65,86.37C56.16,118.75,53.37,93,58.79,69.26Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M232,168H63.86c2.66-5.24,5.33-10.63,8-16.11,14.94,1.65,32.62-8.8,52.74-31.22.6,1.66,1.27,3.37,2,5.1,6.51,15.25,14.91,23.94,25,25.85,10.34,2,20.58-3.23,31.08-15.82C189.5,143.87,203.5,152,232,152a8,8,0,0,0,0-16c-30.63,0-39.55-10.59-40-16.22a8,8,0,0,0-15.51-2.54c-12.17,18.25-19.38,19.14-22,18.66-8.33-1.57-16.08-20.93-18.69-33.51A8,8,0,0,0,121,100.16c-19.8,24.62-33.08,33-41.41,35.14,8.49-18.88,14.83-35.45,18.89-49.4,6.82-23.44,7.32-39.83,1.51-50.1-3-5.36-9.29-11.75-21.91-11.8h-.25c-16,.11-28.6,15.3-34.62,41.7-3.59,15.71-4.18,33.19-1.63,48s7.86,25.51,15.55,31.89c-3.72,7.73-7.53,15.28-11.23,22.43H24a8,8,0,0,0,0,16H37.41c-11.32,21-20.12,35.64-20.26,35.88a8,8,0,1,0,13.71,8.24c.15-.26,11.27-18.79,24.7-44.12H232a8,8,0,0,0,0-16Zm-40-48v-.21A1.11,1.11,0,0,1,192,120Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M232,170H60.6c3.34-6.54,6.73-13.35,10.06-20.29,15.1,2.53,33.41-8.49,54.63-32.88,4.1,12.34,12.37,30.12,26.63,32.82,10.08,1.91,20.18-3.68,30.73-17C188.53,140.82,201.84,150,232,150a6,6,0,0,0,0-12c-40.32,0-41.94-17.14-42-18.12a6,6,0,0,0-11.7-1.75c-9.26,13.93-17.81,20.93-24.14,19.73-5-.94-10.21-7.13-14.63-17.42a105.06,105.06,0,0,1-5.65-17.65,6,6,0,0,0-11.22-1.52c-18.41,22.9-34.79,35.72-46.48,36.65C100,85.84,107.23,52.7,98.23,36.78,95.46,31.88,89.71,26,78,26h-.21c-15,.1-26.89,14.72-32.69,40.14C38.33,96,42.16,132,59.62,145c-4.15,8.65-8.4,17.09-12.52,25H24a6,6,0,0,0,0,12H40.76C28.65,204.58,19,220.66,18.86,220.91a6,6,0,1,0,10.28,6.18c.16-.26,11.56-19.27,25.21-45.09H232a6,6,0,0,0,0-12ZM56.84,68.82C61.1,50.12,69.39,38,78,38c6.36,0,8.59,2.51,9.82,4.69,4,7,6,28.15-22.87,91C54.63,123.37,50.79,95.32,56.84,68.82Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,168H63.86c2.66-5.24,5.33-10.63,8-16.11,14.93,1.65,32.62-8.8,52.74-31.22.6,1.66,1.27,3.37,2,5.1,6.51,15.25,14.91,23.94,25,25.85,10.34,2,20.58-3.23,31.08-15.82C189.5,143.87,203.5,152,232,152a8,8,0,0,0,0-16c-30.63,0-39.55-10.59-40-16.22a8,8,0,0,0-15.51-2.54c-12.17,18.25-19.38,19.14-22,18.66-8.33-1.57-16.08-20.93-18.69-33.51A8,8,0,0,0,121,100.16c-19.8,24.62-33.08,33-41.41,35.14,8.49-18.88,14.83-35.45,18.89-49.4,6.82-23.44,7.32-39.83,1.51-50.1-3-5.36-9.29-11.75-21.91-11.8h-.25c-16,.11-28.6,15.3-34.62,41.7-3.59,15.71-4.18,33.19-1.63,48s7.86,25.51,15.55,31.89c-3.72,7.73-7.53,15.28-11.23,22.43H24a8,8,0,0,0,0,16H37.41c-11.32,21-20.12,35.64-20.26,35.88a8,8,0,1,0,13.71,8.24c.15-.26,11.27-18.79,24.7-44.12H232a8,8,0,0,0,0-16Zm-40-48v-.21A1.11,1.11,0,0,1,192,120ZM58.79,69.26C62.78,51.78,70.48,40,78,40,83.25,40,85,41.86,86,43.67c3,5.33,6.52,24.19-21.65,86.37C56.16,118.75,53.37,93,58.79,69.26Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M232,172H57.32c4-7.83,8.18-16.11,12.21-24.56,15.17,3.56,34.17-8.08,56.62-34.68a109.73,109.73,0,0,0,4.11,11.44c6,13.94,13.37,21.85,22,23.49,9.8,1.85,19.83-4.22,30.49-18.54C187.38,137.33,199.59,148,232,148a4,4,0,0,0,0-8c-42.88,0-44-19.24-44-20a4,4,0,0,0-7.87-1c-9.93,15-19,22.23-26.34,20.84-12.21-2.31-19.93-27.23-21.87-36.64a4,4,0,0,0-7.56-.85c-20.74,25.85-38.67,38.9-51.29,37.56C97.31,87.5,105.19,53.16,96.49,37.77,94,33.33,88.74,28,78,28h-.18c-13.78.1-25.27,14.51-30.76,38.59C43.62,81.8,43,98.72,45.5,113c2.64,15.26,8.37,26,16.65,31.32-4.57,9.59-9.29,19-13.84,27.68H24a4,4,0,0,0,0,8H44.1c-12.87,24.17-23.37,41.68-23.53,41.94a4,4,0,0,0,1.37,5.49A3.93,3.93,0,0,0,24,228a4,4,0,0,0,3.43-1.94c.16-.27,11.85-19.75,25.72-46.06H232a4,4,0,0,0,0-8ZM53.39,111.64a114.51,114.51,0,0,1,1.5-43.27C58.45,52.74,66.39,36,78,36c7.48,0,10.18,3.26,11.56,5.7C99,58.4,79.92,106,65.6,137,59.78,132.68,55.49,123.83,53.39,111.64Z"></path>
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
