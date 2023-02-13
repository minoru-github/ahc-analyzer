use components::header::*;
use components::parameter::parameter_form::ParameterForm;
use components::parameter::parameter_list::ParameterList;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    let on_add = {
        Callback::from(move |name: String| {
            log::info!("on_add: {:?}", name);
        })
    };

    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
                <ParameterForm {on_add} />
                <ParameterList />
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
