//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "finance", feature = "office"))]
#[component]
pub fn ChartDonut(
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
                <path d="M24.75,115.54A102.31,102.31,0,0,1,48.06,61.78a16,16,0,0,1,23.63-1.1L99.36,88.94a15.87,15.87,0,0,1,1.69,20.48h0a25.59,25.59,0,0,0-3.7,7.75A4,4,0,0,1,93.53,120H28.73A4,4,0,0,1,24.75,115.54ZM137.39,24.06A16,16,0,0,0,120,40V80.67a15.86,15.86,0,0,0,13.25,15.76,32,32,0,0,1,5.41,61.76A4.06,4.06,0,0,0,136,162v65.23a4,4,0,0,0,4.46,4A104.34,104.34,0,0,0,232,129.48C232.75,75.19,191.19,28.88,137.39,24.06Zm-20.14,134.1a32,32,0,0,1-19.4-19.42,4.06,4.06,0,0,0-3.8-2.74H28.72a4,4,0,0,0-4,4.45,104.1,104.1,0,0,0,90.82,90.82,4,4,0,0,0,4.45-4V162A4.05,4.05,0,0,0,117.25,158.16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,129.37c-.72,51.7-42.92,93.9-94.62,94.62H128V168a40,40,0,0,0,6.55-79.47A7.92,7.92,0,0,1,128,80.67V40a8,8,0,0,1,8.67-8C186.07,36.46,224.7,78.65,224,129.37Z"
        opacity="0.2"
    ></path>
    <path d="M137.39,24.06A16,16,0,0,0,120,40V80.67a15.86,15.86,0,0,0,13.25,15.76A32,32,0,1,1,96,129.68c-.41-8.22,1.27-15,5-20.26h0a15.86,15.86,0,0,0-1.69-20.47L71.69,60.68a16,16,0,0,0-23.63,1.1A103.6,103.6,0,0,0,55,202.05,103.24,103.24,0,0,0,128,232h1.49A104.3,104.3,0,0,0,232,129.48C232.75,75.18,191.19,28.88,137.39,24.06ZM60.32,71.94l27.61,28.19,0,.06A43.29,43.29,0,0,0,80.44,120H40.36A87.13,87.13,0,0,1,60.32,71.94ZM40.37,136h40.3A48,48,0,0,0,120,175.34v40.3A88,88,0,0,1,40.37,136Zm149.77,54.14A87.45,87.45,0,0,1,136,215.61V175.34a47.59,47.59,0,0,0,24.73-12.23A48,48,0,0,0,136,80.66L136,40c45.52,4.08,80.67,43.28,80,89.25A87.45,87.45,0,0,1,190.14,190.14Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M137,28.05a11.94,11.94,0,0,0-9.12,3.08A12.09,12.09,0,0,0,124,40V80.67a11.88,11.88,0,0,0,9.9,11.81,36,36,0,1,1-41.85,37.4c-.47-9.14,1.47-16.8,5.75-22.79h0a11.89,11.89,0,0,0-1.3-15.35L68.87,63.51a12,12,0,0,0-17.74.83A99.6,99.6,0,0,0,57.79,199.2,99.24,99.24,0,0,0,128,228h1.43A100.29,100.29,0,0,0,228,129.42C228.72,77.21,188.76,32.68,137,28.05ZM57.28,69.46A4,4,0,0,1,60.2,68h.19a3.91,3.91,0,0,1,2.79,1.14L90.81,97.36a3.93,3.93,0,0,1,.48,5.08A40.47,40.47,0,0,0,84.08,124h-48A90.36,90.36,0,0,1,57.28,69.46ZM36.09,132H84.18A44,44,0,0,0,124,171.81V219.9A92,92,0,0,1,36.09,132ZM193,193a91.43,91.43,0,0,1-61,26.92V171.82a43.51,43.51,0,0,0,26-11.63,44,44,0,0,0-22.79-75.6A4,4,0,0,1,132,80.67V40a4,4,0,0,1,1.31-3,3.89,3.89,0,0,1,3-1c47.59,4.26,84.34,45.24,83.67,93.29A91.42,91.42,0,0,1,193,193Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M137.75,20.08A20,20,0,0,0,116,40V80.67a19.83,19.83,0,0,0,16.6,19.7A28,28,0,0,1,126,155.93,28.3,28.3,0,0,1,100,129.47c-.37-7.28,1.06-13.25,4.27-17.73h0a19.88,19.88,0,0,0-2.13-25.64L74.61,57.94l-.09-.09A20,20,0,0,0,45,59.22,107.62,107.62,0,0,0,52.17,204.9,107.21,107.21,0,0,0,128,236h1.55A108.32,108.32,0,0,0,236,129.53C236.77,73.16,193.62,25.08,137.75,20.08ZM60.69,78l22.19,22.66A47.76,47.76,0,0,0,77.08,116H44.87A82.31,82.31,0,0,1,60.69,78ZM44.86,140H77.39A52,52,0,0,0,116,178.6v32.55A84,84,0,0,1,44.86,140ZM140,211.11V178.62A51.53,51.53,0,0,0,163.46,166,52,52,0,0,0,140,77.39V44.5c41.29,5.93,72.58,42.3,72,84.7C211.42,170.56,180,205.25,140,211.11Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M137.21,26.05A14,14,0,0,0,122,40V80.67a13.86,13.86,0,0,0,11.58,13.78,34,34,0,1,1-39.53,35.33c-.44-8.68,1.37-15.92,5.38-21.53h0a13.87,13.87,0,0,0-1.5-17.91L70.28,62.1a14,14,0,0,0-20.69,1,101.62,101.62,0,0,0,6.79,137.57A101.29,101.29,0,0,0,128,230h1.46A102.31,102.31,0,0,0,230,129.45C230.73,76.2,190,30.78,137.21,26.05ZM58.81,70.74A2.06,2.06,0,0,1,60.29,70h.09a1.89,1.89,0,0,1,1.37.54L89.4,98.78a2,2,0,0,1,.27,2.49A41.83,41.83,0,0,0,82.23,122h-44A88.42,88.42,0,0,1,58.81,70.74ZM38.22,134H82.4A46,46,0,0,0,122,173.61v44.17A89.93,89.93,0,0,1,38.22,134Zm153.34,57.56A89.48,89.48,0,0,1,134,217.79V173.62a45.53,45.53,0,0,0,25.36-12,46,46,0,0,0-23.83-79A1.93,1.93,0,0,1,134,80.67V40a2,2,0,0,1,.66-1.5,1.91,1.91,0,0,1,1.48-.5c46.55,4.17,82.51,44.26,81.85,91.27A89.44,89.44,0,0,1,191.56,191.56Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M137.39,24.06A16,16,0,0,0,120,40V80.67a15.86,15.86,0,0,0,13.25,15.76A32,32,0,1,1,96,129.68c-.41-8.22,1.27-15,5-20.26h0a15.86,15.86,0,0,0-1.69-20.47L71.69,60.68a16,16,0,0,0-23.63,1.1A103.6,103.6,0,0,0,55,202.05,103.24,103.24,0,0,0,128,232h1.49A104.3,104.3,0,0,0,232,129.48C232.75,75.18,191.19,28.88,137.39,24.06ZM60.32,71.94l27.61,28.19,0,.06A43.29,43.29,0,0,0,80.44,120H40.36A87.13,87.13,0,0,1,60.32,71.94ZM40.37,136h40.3A48,48,0,0,0,120,175.34v40.3A88,88,0,0,1,40.37,136Zm149.77,54.14A87.45,87.45,0,0,1,136,215.61V175.34a47.55,47.55,0,0,0,24.73-12.23A48,48,0,0,0,136,80.66L136,40c45.52,4.08,80.67,43.28,80,89.25A87.45,87.45,0,0,1,190.14,190.14Z"></path>
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
