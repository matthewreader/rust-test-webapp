use yew_router::prelude::*;
use yew::prelude::*;
use yew_router::__macro::Router;

use crate::pages;
use crate::routes::AppRoute;

pub struct AppRouter;

impl Component for AppRouter {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        todo!()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}
