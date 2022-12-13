use yew::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

enum Msg {
    Next,
    Back,
}

struct App {
    counter: u64,
    seed: u32,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            counter: 0,
            seed: 0,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Next => {
                self.seed = (self.seed.rotate_left(1) + 1) ^ 0x873CA9E5;
                self.counter = self.counter.wrapping_add(1);
                true
            }
            Msg::Back => {
                self.seed = ((self.seed ^ 0x873CA9E5) - 1).rotate_right(1);
                self.counter = self.counter.wrapping_sub(1);
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <button onclick={ctx.link().callback(|_| Msg::Back)}>{"back"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Next)}>{"next"}</button>
                <p>{format!("{}: {:08X}", self.counter, self.seed)}</p>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
