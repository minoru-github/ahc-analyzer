use yew::{function_component, html, Html, InputEvent};

#[function_component(DataReader)]
pub fn data_reader() -> Html {
    html! {
        <div class="input-group mb-3">
            <input type="file" class="form-control" id="inputGroupFile01" accept=".csv, .txt" />
        </div>
    }
}
