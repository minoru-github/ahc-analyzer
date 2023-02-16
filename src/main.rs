use components::header::*;
use components::parameter::parameter_form::ParameterForm;
use components::parameter::parameter_list::ParameterList;
use components::parameter::types::Parameter;
use components::data::data_reader::DataReader;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    let parameter_items = use_state(|| {
        Vec::<Parameter>::from([Parameter {
            name: "N".to_string(),
        }])
    });

    let on_add = {
        let parameter_items = parameter_items.clone();
        Callback::from(move |name: String| {
            let mut current_parameter_items = (*parameter_items).clone();
            current_parameter_items.push(Parameter { name });
            parameter_items.set(current_parameter_items);
        })
    };

    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
                <ParameterForm {on_add} />
                <ParameterList parameter_items = {(*parameter_items).clone()} />
                <DataReader />
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
