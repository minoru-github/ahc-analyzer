use super::parameter_item::ParameterItem;
use super::types::Parameter;
use yew::{function_component, html, Html};

#[function_component(ParameterList)]
pub fn parameter_list() -> Html {
    let parameter = Parameter {
        name: String::from("N"),
    };
    html! {
    <ul class="list-group">
        <ParameterItem name={parameter.name} selected={false} />
    </ul>
    }
}
