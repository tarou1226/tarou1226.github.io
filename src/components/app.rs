use yew::prelude::*;
use super::header::Header;
use super::footer::Footer;
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
                <main>
                    <div class="home-wrapper">
                        <h1>{ "Hello, World!" }</h1>
                        <div class="home-container">
                            <p>{ "魔法の言葉でターノシーなっかまーが" }</p>
                            <strong>{ "ポポポポーン" }</strong>
                        </div>
                    </div>
                </main>
                <Footer />
            </>
        }
    }
}