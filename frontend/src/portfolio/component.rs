use yew::{classes, html, services::ConsoleService, ComponentLink, Html, ShouldRender};

use super::message::Message;

pub struct Component {
    link: ComponentLink<Self>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::LoadData);
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::LoadData => {
                ConsoleService::info(&format!("Portfolio -> Loading data: ????"));
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("portfolio-container")>
                {"?.?S.loading.?.?"}
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.link.send_message(Message::LoadData);
        false
    }
}
