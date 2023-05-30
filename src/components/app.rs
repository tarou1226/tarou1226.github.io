use yew::prelude::*;
use super::header::Header;
use super::footer::Footer;
use super::top::Top;
pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header />
                <Top />
                <Footer />
            </>
        }
    }
}