use yew::prelude::*;
pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <footer>
                    <div class="footer-wrapper">
                        <div class="footer-content">
                            <p>{ "Copyright ©️ 2023 n0ta. All rights reserved." }</p>
                        </div>
                    </div>
                </footer>
            </>
        }
    }
}