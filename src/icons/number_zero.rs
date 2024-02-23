//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "finance"))]
#[component]
pub fn NumberZero(
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
                <path d="M186.62,61.05c-13.76-21.62-34-33-58.62-33S83.14,39.43,69.38,61.05C58.17,78.66,52,102.44,52,128s6.17,49.33,17.38,66.94C83.14,216.57,103.41,228,128,228s44.86-11.43,58.62-33.06C197.83,177.33,204,153.56,204,128S197.83,78.66,186.62,61.05Zm-20.25,121C157.11,196.62,144.2,204,128,204s-29.11-7.38-38.37-21.94C80.84,168.25,76,149.05,76,128s4.84-40.25,13.63-54.06C98.89,59.38,111.8,52,128,52s29.11,7.38,38.37,21.94C175.16,87.75,180,107,180,128S175.16,168.25,166.37,182.06Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,40V216a16,16,0,0,1-16,16H56a16,16,0,0,1-16-16V40A16,16,0,0,1,56,24H200A16,16,0,0,1,216,40Z"
        opacity="0.2"
    ></path>
    <path d="M183.25,63.2C170.25,42.79,151.15,32,128,32S85.75,42.79,72.75,63.2C62,80.18,56,103.19,56,128s6,47.82,16.75,64.8c13,20.41,32.1,31.2,55.25,31.2s42.25-10.79,55.25-31.2c10.8-17,16.75-40,16.75-64.8S194.05,80.18,183.25,63.2ZM128,208c-38.68,0-56-40.18-56-80s17.32-80,56-80,56,40.18,56,80S166.68,208,128,208Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M157.68,88.39C164.34,98.52,168,112.59,168,128s-3.66,29.47-10.32,39.61C150.55,178.49,140.56,184,128,184s-22.55-5.51-29.68-16.39C91.66,157.47,88,143.41,88,128s3.66-29.48,10.32-39.61C105.45,77.51,115.44,72,128,72S150.55,77.51,157.68,88.39ZM216,40V216a16,16,0,0,1-16,16H56a16,16,0,0,1-16-16V40A16,16,0,0,1,56,24H200A16,16,0,0,1,216,40Zm-32,88c0-18.49-4.6-35.68-12.94-48.39C160.92,64.16,146,56,128,56S95.08,64.16,84.94,79.61C76.6,92.32,72,109.51,72,128s4.6,35.68,12.94,48.39C95.08,191.84,110,200,128,200s32.92-8.16,43.06-23.61C179.4,163.68,184,146.49,184,128Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M181.56,64.28C169,44.47,150.43,34,128,34S87.05,44.47,74.44,64.28C63.84,80.94,58,103.57,58,128s5.84,47.06,16.44,63.72C87.05,211.53,105.57,222,128,222s40.95-10.47,53.56-30.28C192.16,175.06,198,152.43,198,128S192.16,80.94,181.56,64.28ZM128,210c-40.07,0-58-41.18-58-82s17.93-82,58-82,58,41.18,58,82S168.07,210,128,210Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M183.25,63.2C170.25,42.79,151.15,32,128,32S85.75,42.79,72.75,63.2C62,80.18,56,103.19,56,128s6,47.82,16.75,64.8c13,20.41,32.1,31.2,55.25,31.2s42.25-10.79,55.25-31.2c10.8-17,16.75-40,16.75-64.8S194.05,80.18,183.25,63.2ZM128,208c-38.68,0-56-40.18-56-80s17.32-80,56-80,56,40.18,56,80S166.68,208,128,208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M179.87,65.35C167.65,46.15,149.71,36,128,36S88.35,46.15,76.13,65.35C65.73,81.69,60,103.94,60,128s5.73,46.31,16.13,62.65C88.35,209.85,106.29,220,128,220s39.65-10.15,51.87-29.35C190.27,174.31,196,152.06,196,128S190.27,81.69,179.87,65.35ZM128,212c-41.45,0-60-42.19-60-84s18.55-84,60-84,60,42.19,60,84S169.45,212,128,212Z"></path>
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
