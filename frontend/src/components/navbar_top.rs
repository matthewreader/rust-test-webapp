use yew::prelude::*;

pub struct NavbarTopComponent;

impl Component for NavbarTopComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="topnav">
              <a class="active" href="#home"> {"Home" }</a>
                <a href="#about">{ "Home" }</a>
            </div>
        }
    }
}

