use yew::{classes, html, ComponentLink, Html, ShouldRender};

use super::message::Message;

pub struct Component {
    link: ComponentLink<Self>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("portfolio-container")>
                {"....loading..."}
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
