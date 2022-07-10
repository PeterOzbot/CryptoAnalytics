use gloo_timers::callback::Interval;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    EventBusMsg,
}

pub enum Msg {
    UpdateTime,
}

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>,
    //interval: Option<Interval>,
}

impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = String;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
            //interval: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {

        //self.link.send_future();

        // match msg {
        //     Msg::UpdateTime => {
        //         for sub in self.subscribers.iter() {
        //             self.link.respond(*sub, "changed".to_owned());
        //         }
        //     }
        // }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::EventBusMsg => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, "changed".to_owned());
                }
                // self.interval = Some({
                //     let link = self.link.clone();
                //     Interval::new(10 * 1000, move || link.send_message(Msg::UpdateTime))
                // });
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
        //self.link.respond(id, "changed".to_owned());
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
