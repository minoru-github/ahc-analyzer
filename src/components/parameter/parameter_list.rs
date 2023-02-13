use super::parameter_item::ParameterItem;
use super::types::Parameter;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ParameterListProps {
    pub parameter_items: Vec<Parameter>,
}

#[function_component(ParameterList)]
pub fn parameter_list(props: &ParameterListProps) -> Html {
    html! {
    <ul class="list-group">
        {props.parameter_items.iter().map(|parameter| html!{
            <ParameterItem name={parameter.name.clone()} selected={false} />
        }).collect::<Html>()}
    </ul>
    }
}
