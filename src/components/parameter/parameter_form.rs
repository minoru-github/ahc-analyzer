use yew::{function_component, html, Html};

#[function_component(ParameterForm)]
pub fn parameter_form() -> Html {
    html! {
        <form class="mb-5">
            <div>
                <label for="input-parameter" class="form-label">{"入力パラメーター"}</label>
                <input type="text" class="form-control" id="input-parameter" />
            </div>
            <button type="submit" class="btn btn-primary">{"追加"}</button>
        </form>
    }
}
