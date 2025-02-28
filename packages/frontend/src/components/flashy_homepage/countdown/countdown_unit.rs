use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub unit_name: String,
    pub value: i64,
    #[prop_or_default]
    pub skip_leading_zeroes: bool,
    pub light: bool,
}

#[function_component(CountdownUnit)]
pub fn countdown_unit(props: &Props) -> Html {
    let formatted_value = if props.skip_leading_zeroes {
        props.value.to_string()
    } else {
        format!("{:0>2}", props.value)
    };

    let div_classes = classes!(
        "px-6",
        "py-2",
        "border-r",
        "last:border-none",
        "flex",
        "items-center",
        "flex-col",
        if props.light {
            "border-bcss-300"
        } else {
            "border-bcss-400"
        },
    );

    html! {
        <div class={div_classes}>
            <p class="text-3xl text-bcss-950 font-medium leading-none">{ formatted_value }</p>
            <p class="text-bcss-900 leading-none">{ props.unit_name.to_owned() }</p>
        </div>
    }
}
