use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
                <h1>{ "Hello World!" }</h1>
                <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            </main>
        }
    }
}
