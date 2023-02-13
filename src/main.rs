use components::header::*;
use components::parameter::parameter_form::ParameterForm;
use components::parameter::parameter_list::ParameterList;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
                <ParameterForm />
                <ParameterList />
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
