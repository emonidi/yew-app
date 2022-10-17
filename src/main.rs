mod components;
mod domain;
mod stores;
mod helpers;

use geo::ChaikinSmoothing;
use geojson::Feature;
use helpers::create_json::create_json;
use stdweb::{js, unstable::TryInto};
use weblog::console_log;
use yew::prelude::*;
use yew_router::prelude::*;
use geo_types::{LineString};

use crate::components::home::Home;
use gloo_net::http::Request;

use interop::ResourceProvider;

use web_sys::{Event, HtmlBaseElement, HtmlSelectElement};
use domain::json_data::{JsonData,Airport};
use yewdux::prelude::*;
use crate::stores::main_data::MainDataStore;

mod interop {
    use yew_interop::declare_resources;

    declare_resources!(
        mapbox
        "https://cdn.jsdelivr.net/npm/mapbox-gl@2.10.0/dist/mapbox-gl.min.js"
        "https://cdn.jsdelivr.net/npm/mapbox-gl@2.10.0/dist/mapbox-gl.css"
    );
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home/>
        },
    }
}




#[function_component(App)]
fn app() -> Html {
    
    let (data,dispatch) = use_store::<MainDataStore>();
    
    use_effect_with_deps(move |_|{
        wasm_bindgen_futures::spawn_local(async move {
            let data:JsonData = Request::get("https://localhost:8000/assets/flights/main_data.json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

                dispatch.reduce_mut(|state|{
                    state.airports.push(data.takeoff_airport);
                    state.airports.push(data.landing_airport);
                });

            match data.gps_data_url {
                Some(url)=>{
                    let data:geojson::FeatureCollection = Request::get(format!("https://localhost:8000/{}",&url).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                    let path_geo_json = data.features.get(1).unwrap();
                    let path_geom:LineString<f64> = LineString::try_from(path_geo_json.geometry.clone().unwrap()).unwrap();
                    let interpolated = path_geom.chaikin_smoothing(10);
                    console_log!(interpolated.0.to_vec().len());
                    let json_collection = create_json(path_geo_json);

                    dispatch.reduce_mut(|state|{
                        state.path = interpolated;
                        state.geo_json_path = Some(json_collection.clone());
                        state.geo_json_line = Some(path_geo_json.clone());
                    });
                },
                None=>{

                }
            }
            
        }); || ()
    }, ());

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
