//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "arrows"))]
#[component]
pub fn ArrowsIn(
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
                <path d="M140,104V64a12,12,0,0,1,24,0V75l35.51-35.52a12,12,0,0,1,17,17L181,92h11a12,12,0,0,1,0,24H152A12,12,0,0,1,140,104Zm-36,36H64a12,12,0,0,0,0,24H75L39.51,199.51a12,12,0,0,0,17,17L92,181v11a12,12,0,0,0,24,0V152A12,12,0,0,0,104,140Zm77,24h11a12,12,0,0,0,0-24H152a12,12,0,0,0-12,12v40a12,12,0,0,0,24,0V181l35.51,35.52a12,12,0,0,0,17-17ZM104,52A12,12,0,0,0,92,64V75L56.49,39.51a12,12,0,0,0-17,17L75,92H64a12,12,0,0,0,0,24h40a12,12,0,0,0,12-12V64A12,12,0,0,0,104,52Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M192,104H152V64Zm-88,88V152H64Zm48,0,40-40H152Zm-48-88V64L64,104Z"
        opacity="0.2"
    ></path>
    <path d="M152,112h40a8,8,0,0,0,5.66-13.66L183.31,84l30.35-30.34a8,8,0,0,0-11.32-11.32L172,72.69,157.66,58.34A8,8,0,0,0,144,64v40A8,8,0,0,0,152,112Zm8-28.69L172.69,96H160ZM104,144H64a8,8,0,0,0-5.66,13.66L72.69,172,42.34,202.34a8,8,0,0,0,11.32,11.32L84,183.31l14.34,14.35A8,8,0,0,0,112,192V152A8,8,0,0,0,104,144Zm-8,28.69L83.31,160H96Zm87.31-.69,14.35-14.34A8,8,0,0,0,192,144H152a8,8,0,0,0-8,8v40a8,8,0,0,0,13.66,5.66L172,183.31l30.34,30.35a8,8,0,0,0,11.32-11.32Zm-23.31.69V160h12.69ZM107.06,56.61a8,8,0,0,0-8.72,1.73L84,72.69,53.66,42.34A8,8,0,0,0,42.34,53.66L72.69,84,58.34,98.34A8,8,0,0,0,64,112h40a8,8,0,0,0,8-8V64A8,8,0,0,0,107.06,56.61ZM96,96H83.31L96,83.31Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M144,104V64a8,8,0,0,1,13.66-5.66L172,72.69l30.34-30.35a8,8,0,0,1,11.32,11.32L183.31,84l14.35,14.34A8,8,0,0,1,192,112H152A8,8,0,0,1,144,104Zm-40,40H64a8,8,0,0,0-5.66,13.66L72.69,172,42.34,202.34a8,8,0,0,0,11.32,11.32L84,183.31l14.34,14.35A8,8,0,0,0,112,192V152A8,8,0,0,0,104,144Zm3.06-87.39a8,8,0,0,0-8.72,1.73L84,72.69,53.66,42.34A8,8,0,0,0,42.34,53.66L72.69,84,58.34,98.34A8,8,0,0,0,64,112h40a8,8,0,0,0,8-8V64A8,8,0,0,0,107.06,56.61ZM183.31,172l14.35-14.34A8,8,0,0,0,192,144H152a8,8,0,0,0-8,8v40a8,8,0,0,0,13.66,5.66L172,183.31l30.34,30.35a8,8,0,0,0,11.32-11.32Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M146,104V64a6,6,0,0,1,12,0V89.51l45.76-45.75a6,6,0,0,1,8.48,8.48L166.48,98H192a6,6,0,0,1,0,12H152A6,6,0,0,1,146,104Zm-42,42H64a6,6,0,0,0,0,12H89.52L43.76,203.76a6,6,0,1,0,8.48,8.48L98,166.48V192a6,6,0,0,0,12,0V152A6,6,0,0,0,104,146Zm62.48,12H192a6,6,0,0,0,0-12H152a6,6,0,0,0-6,6v40a6,6,0,0,0,12,0V166.48l45.76,45.76a6,6,0,0,0,8.48-8.48ZM104,58a6,6,0,0,0-6,6V89.51L52.24,43.76a6,6,0,0,0-8.48,8.48L89.52,98H64a6,6,0,0,0,0,12h40a6,6,0,0,0,6-6V64A6,6,0,0,0,104,58Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M144,104V64a8,8,0,0,1,16,0V84.69l42.34-42.35a8,8,0,0,1,11.32,11.32L171.31,96H192a8,8,0,0,1,0,16H152A8,8,0,0,1,144,104Zm-40,40H64a8,8,0,0,0,0,16H84.69L42.34,202.34a8,8,0,0,0,11.32,11.32L96,171.31V192a8,8,0,0,0,16,0V152A8,8,0,0,0,104,144Zm67.31,16H192a8,8,0,0,0,0-16H152a8,8,0,0,0-8,8v40a8,8,0,0,0,16,0V171.31l42.34,42.35a8,8,0,0,0,11.32-11.32ZM104,56a8,8,0,0,0-8,8V84.69L53.66,42.34A8,8,0,0,0,42.34,53.66L84.69,96H64a8,8,0,0,0,0,16h40a8,8,0,0,0,8-8V64A8,8,0,0,0,104,56Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M148,104V64a4,4,0,0,1,8,0V94.34l49.17-49.17a4,4,0,1,1,5.66,5.66L161.66,100H192a4,4,0,0,1,0,8H152A4,4,0,0,1,148,104Zm-44,44H64a4,4,0,0,0,0,8H94.34L45.17,205.17a4,4,0,0,0,5.66,5.66L100,161.66V192a4,4,0,0,0,8,0V152A4,4,0,0,0,104,148Zm57.66,8H192a4,4,0,0,0,0-8H152a4,4,0,0,0-4,4v40a4,4,0,0,0,8,0V161.66l49.17,49.17a4,4,0,0,0,5.66-5.66ZM104,60a4,4,0,0,0-4,4V94.34L50.83,45.17a4,4,0,0,0-5.66,5.66L94.34,100H64a4,4,0,0,0,0,8h40a4,4,0,0,0,4-4V64A4,4,0,0,0,104,60Z"></path>
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
