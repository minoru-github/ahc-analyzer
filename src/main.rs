use yew::prelude::*;
use components::header::*;
use components::parameter::parameter_list::ParameterList;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
                <ParameterList />
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
