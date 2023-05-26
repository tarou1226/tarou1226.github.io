use yew::prelude::*;
pub struct Header;

const ROOT_ENTRYPOINT: &str = "/n0ta_pages";

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <header>
                    <div class="logo">
                        <img src="logo.png" alt="Logo" />
                    </div>

                    <nav>
                        <ul>
                            <li><a href={ ROOT_ENTRYPOINT }>{ "Home" }</a></li>
                            <li><a href={ ROOT_ENTRYPOINT.to_owned() + "/products" }>{ "Products" }</a></li>
                            <li><a href={ ROOT_ENTRYPOINT.to_owned() + "/services" }>{ "Services" }</a></li>
                            <li><a href={ ROOT_ENTRYPOINT.to_owned() + "/contact" }>{ "Contact" }</a></li>
                        </ul>
                    </nav>
                </header>
            </>
        }
    }
}