use yew::{
    function_component, html, use_state, Callback, Html, InputEvent, MouseEvent, Properties,
};

#[derive(Properties, PartialEq)]
pub struct ParameterFormProps {
    pub on_add: Callback<String>,
}

#[function_component(ParameterForm)]
pub fn parameter_form(props: &ParameterFormProps) -> Html {
    let name = use_state(|| "".to_string());

    let oninput = {
        let name = name.clone();
        // moveでnameの所有権をクロージャに強制的に移し、
        // name.set()で値の更新ができるようにする。
        Callback::from(move |e: InputEvent| {
            let value = e.data();

            match value {
                Some(value) => {
                    name.set((*name).clone() + &value);
                }
                None => {
                    // input内を全消しした時とかがココ
                    name.set("".to_string());
                }
            }
        })
    };

    let onclick = {
        let on_add = props.on_add.clone();
        let name = name.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); // Web APIのEvent.preventDefault()と同じ
            name.set("".to_string());
            on_add.emit((*name).clone());
        })
    };

    html! {
        <form class="input-group mb-3">
                <button class="btn btn-outline-secondary" type="button" {onclick} >{"add"}</button>
                <input type="text" value={(*name).clone()} {oninput} class="form-control" placeholder="input parameter" />
        </form>
    }
}
