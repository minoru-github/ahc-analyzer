use yew::{function_component, html, Html, Properties};

// Propertiesトレイトを継承するためにはPartialEqが必要
// Propsのメンバーにkeyという名前を付けるとエラーになる
#[derive(Properties, PartialEq)]
pub struct ParameterItemProps {
    pub name: String,
    pub selected: bool,
}

// String型の変数をhtmlマクロを使って描画するには参照で渡す必要がある
// そうしないと所有権がprops.keyに残り続けコンパイルエラーになる。
#[function_component(ParameterItem)]
pub fn parameter_item(props: &ParameterItemProps) -> Html {
    html! {
        <li class="list-group-item">
        <input class="form-check-input me-2" type="checkbox" checked={props.selected} />
            {&props.name}
        </li>
    }
}
