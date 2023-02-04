use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <h1>{ "Welcome to the Counter App!" }</h1>
                <p>{ "Please, press the below button to see the counter change" }</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <h2>{ self.value }</h2>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}