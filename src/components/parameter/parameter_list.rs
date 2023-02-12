use super::parameter_item::ParameterItem;
use super::types::Parameter;
use yew::{function_component, html, Html};

#[function_component(ParameterList)]
pub fn parameter_list() -> Html {
    let parameter_items = vec![
        Parameter {
            name: String::from("N"),
        },
        Parameter {
            name: String::from("M"),
        },
        Parameter {
            name: String::from("K"),
        },
    ];

    html! {
    <ul class="list-group">
        {parameter_items.iter().map(|parameter| html!{
            <ParameterItem name={parameter.name.clone()} selected={false} />
        }).collect::<Html>()}
    </ul>
    }
}
