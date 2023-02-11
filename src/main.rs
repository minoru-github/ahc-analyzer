use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Hello World!"}</h1>
            <button>{"Click me"}</button>
            <br/>
            <input type="text"/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
