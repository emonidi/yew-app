mod components;
mod domain;
mod stores;
mod helpers;

use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::home::Home;
use interop::ResourceProvider;



mod interop {
    use yew_interop::declare_resources;

    declare_resources!(
        // mapbox
        // "https://cdn.jsdelivr.net/npm/mapbox-gl@2.10.0/dist/mapbox-gl.min.js"
        // "https://cdn.jsdelivr.net/npm/mapbox-gl@2.10.0/dist/mapbox-gl.css"
    );
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/flyover/:pilot_id/:flight_id")]
    Home{pilot_id:String,flight_id:String},
}

fn switch(routes: &Route) -> Html {
   
    match routes {
        Route::Home{flight_id,pilot_id} => html! {
            <Home flight_id={flight_id.clone()} pilot_id={pilot_id.clone()}/>
        },
    }
}




#[function_component(App)]
fn app() -> Html {
    
   

    html! {
        <ResourceProvider>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ResourceProvider>
    }
}

fn main() {
    yew::start_app::<App>();
}
