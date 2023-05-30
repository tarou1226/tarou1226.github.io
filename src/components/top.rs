use yew::prelude::*;
pub struct Top;

impl Component for Top {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <main>
                    <div class="top-wrapper">
                        <h1>{ "n0ta's PortFolio Web Site!" }</h1>
                        <div class="top-container">
                            <p>{ "This website is an introduction site for n0ta" }</p>
                        </div>
                    </div>
                </main>
            </>
        }
    }
}