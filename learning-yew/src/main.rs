use yew::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "はろーわーるど" }</h1>
            <img src={ "images/512x384.png" } />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
