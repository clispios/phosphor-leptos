//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "development"))]
#[component]
pub fn Circuitry(
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
                <path d="M208,28H48A20,20,0,0,0,28,48V208a20,20,0,0,0,20,20H208a20,20,0,0,0,20-20V48A20,20,0,0,0,208,28ZM52,52H76V152a20,20,0,1,0,24,0V125l40,40v39H52ZM204,204H164V160a12,12,0,0,0-3.51-8.49L100,91V52h24V72a12,12,0,0,0,3.51,8.49l20.71,20.7a20.17,20.17,0,1,0,17-17L148,67V52h56Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,40H48a8,8,0,0,0-8,8V208a8,8,0,0,0,8,8H208a8,8,0,0,0,8-8V48A8,8,0,0,0,208,40ZM88,184a16,16,0,1,1,16-16A16,16,0,0,1,88,184Zm80-64a16,16,0,1,1,16-16A16,16,0,0,1,168,120Z"
        opacity="0.2"
    ></path>
    <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM88,160a8,8,0,1,1-8,8A8,8,0,0,1,88,160ZM48,48H80v97.38a24,24,0,1,0,16,0V115.31l48,48V208H48ZM208,208H160V160a8,8,0,0,0-2.34-5.66L96,92.69V48h32V72a8,8,0,0,0,2.34,5.66l16,16A23.74,23.74,0,0,0,144,104a24,24,0,1,0,24-24,23.74,23.74,0,0,0-10.34,2.35L144,68.69V48h64V208ZM168,96a8,8,0,1,1-8,8A8,8,0,0,1,168,96Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M88,111.31l48,48V220a4,4,0,0,1-4,4H48a16,16,0,0,1-16-16V48A16,16,0,0,1,48,32H68a4,4,0,0,1,4,4V153.38a24,24,0,1,0,16,0ZM80,184a8,8,0,1,0-8-8A8,8,0,0,0,80,184Zm104-80a8,8,0,1,0-8,8A8,8,0,0,0,184,104Zm24-72H156a4,4,0,0,0-4,4V68.69l13.66,13.66a24,24,0,1,1-11.31,11.31l-16-16A8,8,0,0,1,136,72V36a4,4,0,0,0-4-4H92a4,4,0,0,0-4,4V88.69l61.66,61.65A8,8,0,0,1,152,156v64a4,4,0,0,0,4,4h52a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M208,34H48A14,14,0,0,0,34,48V208a14,14,0,0,0,14,14H208a14,14,0,0,0,14-14V48A14,14,0,0,0,208,34ZM88,158a10,10,0,1,1-10,10A10,10,0,0,1,88,158ZM46,208V48a2,2,0,0,1,2-2H82V146.84a22,22,0,1,0,12,0V110.49l52,52V210H48A2,2,0,0,1,46,208Zm164,0a2,2,0,0,1-2,2H158V160a6,6,0,0,0-1.76-4.24L94,93.51V46h36V72a6,6,0,0,0,1.76,4.24l17,17a22,22,0,1,0,8.48-8.48L142,69.51V46h66a2,2,0,0,1,2,2ZM168,94a10,10,0,1,1-10,10A10,10,0,0,1,168,94Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM88,160a8,8,0,1,1-8,8A8,8,0,0,1,88,160ZM48,48H80v97.38a24,24,0,1,0,16,0V115.31l48,48V208H48ZM208,208H160V160a8,8,0,0,0-2.34-5.66L96,92.69V48h32V72a8,8,0,0,0,2.34,5.66l16,16A23.74,23.74,0,0,0,144,104a24,24,0,1,0,24-24,23.74,23.74,0,0,0-10.34,2.35L144,68.69V48h64V208ZM168,96a8,8,0,1,1-8,8A8,8,0,0,1,168,96Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M208,36H48A12,12,0,0,0,36,48V208a12,12,0,0,0,12,12H208a12,12,0,0,0,12-12V48A12,12,0,0,0,208,36ZM88,156a12,12,0,1,1-12,12A12,12,0,0,1,88,156ZM44,208V48a4,4,0,0,1,4-4H84V148.4a20,20,0,1,0,8,0V105.66l56,56V212H48A4,4,0,0,1,44,208Zm168,0a4,4,0,0,1-4,4H156V160a4,4,0,0,0-1.17-2.83L92,94.34V44h40V72a4,4,0,0,0,1.17,2.83L151.33,93A20,20,0,1,0,157,87.33l-17-17V44h68a4,4,0,0,1,4,4ZM168,92a12,12,0,1,1-12,12A12,12,0,0,1,168,92Z"></path>
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
