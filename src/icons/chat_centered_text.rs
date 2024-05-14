//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication"))]
#[component]
pub fn ChatCenteredText(
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
                <path d="M216,40H40A16,16,0,0,0,24,56V184a16,16,0,0,0,16,16l59.5.06,14.78,24.17a16,16,0,0,0,27.41.06L156.53,200H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM160,144H96a8,8,0,0,1,0-16h64a8,8,0,0,1,0,16Zm0-32H96a8,8,0,0,1,0-16h64a8,8,0,0,1,0,16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,56V184a8,8,0,0,1-8,8H156.53a8,8,0,0,0-6.86,3.88l-14.81,24.24a8,8,0,0,1-13.72,0l-14.81-24.24A8,8,0,0,0,99.47,192H40a8,8,0,0,1-8-8V56a8,8,0,0,1,8-8H216A8,8,0,0,1,224,56Z"
        opacity="0.2"
    ></path>
    <path d="M88,104a8,8,0,0,1,8-8h64a8,8,0,0,1,0,16H96A8,8,0,0,1,88,104Zm8,40h64a8,8,0,0,0,0-16H96a8,8,0,0,0,0,16ZM232,56V184a16,16,0,0,1-16,16H156.53l-14.84,24.29a16,16,0,0,1-27.41-.06L99.47,200H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216A16,16,0,0,1,232,56Zm-16,0H40V184H99.47a16.08,16.08,0,0,1,13.7,7.73L128,216l14.82-24.32A16.07,16.07,0,0,1,156.53,184H216Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M216,44H40A12,12,0,0,0,28,56V184a12,12,0,0,0,12,12H99.47a4,4,0,0,1,3.43,1.94l0,0,14.79,24.2a12,12,0,0,0,20.56,0l14.83-24.26a4,4,0,0,1,3.43-1.94H216a12,12,0,0,0,12-12V56A12,12,0,0,0,216,44Zm4,140a4,4,0,0,1-4,4H156.53a12,12,0,0,0-10.27,5.8l-14.83,24.26a4,4,0,0,1-6.88,0l-14.8-24.22A12,12,0,0,0,99.47,188H40a4,4,0,0,1-4-4V56a4,4,0,0,1,4-4H216a4,4,0,0,1,4,4Zm-56-80a4,4,0,0,1-4,4H96a4,4,0,0,1,0-8h64A4,4,0,0,1,164,104Zm0,32a4,4,0,0,1-4,4H96a4,4,0,0,1,0-8h64A4,4,0,0,1,164,136Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M216,36H40A20,20,0,0,0,20,56V184a20,20,0,0,0,20,20H97.23l13.62,22.29a20,20,0,0,0,34.25.08L158.77,204H216a20,20,0,0,0,20-20V56A20,20,0,0,0,216,36Zm-4,144H156.53a20,20,0,0,0-17.1,9.63L128,208.33l-11.41-18.67A20.1,20.1,0,0,0,99.47,180H44V60H212ZM84,100A12,12,0,0,1,96,88h64a12,12,0,0,1,0,24H96A12,12,0,0,1,84,100Zm0,40a12,12,0,0,1,12-12h64a12,12,0,0,1,0,24H96A12,12,0,0,1,84,140Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M216,42H40A14,14,0,0,0,26,56V184a14,14,0,0,0,14,14H99.47a2,2,0,0,1,1.71,1l0,0L116,223.2a14,14,0,0,0,24,.05L154.82,199a2,2,0,0,1,1.71-1H216a14,14,0,0,0,14-14V56A14,14,0,0,0,216,42Zm2,142a2,2,0,0,1-2,2H156.53a14,14,0,0,0-12,6.75L129.72,217a2,2,0,0,1-3.46,0l-14.8-24.22a14.09,14.09,0,0,0-12-6.77H40a2,2,0,0,1-2-2V56a2,2,0,0,1,2-2H216a2,2,0,0,1,2,2Zm-52-80a6,6,0,0,1-6,6H96a6,6,0,0,1,0-12h64A6,6,0,0,1,166,104Zm0,32a6,6,0,0,1-6,6H96a6,6,0,0,1,0-12h64A6,6,0,0,1,166,136Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M88,104a8,8,0,0,1,8-8h64a8,8,0,0,1,0,16H96A8,8,0,0,1,88,104Zm8,40h64a8,8,0,0,0,0-16H96a8,8,0,0,0,0,16ZM232,56V184a16,16,0,0,1-16,16H156.53l-14.84,24.29a16,16,0,0,1-27.41-.06L99.47,200H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216A16,16,0,0,1,232,56Zm-16,0H40V184H99.47a16.08,16.08,0,0,1,13.7,7.73L128,216l14.82-24.32A16.07,16.07,0,0,1,156.53,184H216Z"></path>
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
