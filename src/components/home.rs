use std::ops::Deref;

use wasm_bindgen::JsCast;
use yew::{prelude::*, html::IntoPropValue};
use weblog::{console_log};
use web_sys::{Event, HtmlSelectElement, console};
use crate::{components::map::{Map}, stores::main_data::{MainDataStore, self}, domain::json_data::Airport};
use yewdux::functional::use_store;

#[function_component(Home)]
pub(crate) fn home() -> Html {

    let (main_data_store,main_data_dispatch) = use_store::<MainDataStore>();
    
    let options = vec![
        "mercator",
        "globe",
        "albers",
        "equalEarth",
        "equirectangular",
        "lambertConformalConic",
        "naturalEarth",
        "winkelTripel",
    ];

    

    let projection = use_state(|| "globe");

    let on_select_change = {
        let projection = projection.clone();
        Callback::from(move |e: Event| {
            let target = e
                .target()
                .unwrap()
                .value_of()
                .dyn_into::<HtmlSelectElement>()
                .unwrap();
            let selected = options[target.selected_index() as usize];
            projection.set(selected);
        })
    };

    html!{
        <div>
            <fieldset class="absolute ml-0 mt-0 z-10">
                <label>{"Select projection"}</label>
                <select id="projection" name="projection" onchange={on_select_change}>
                    <option value="mercator">{"Mercator"}</option>
                    <option value="globe">{"Globe"}</option>
                    <option value="albers">{"Albers"}</option>
                    <option value="equalEarth">{"Equal Earth"}</option>
                    <option value="equirectangular">{"Equirectangular"}</option>
                    <option value="lambertConformalConic">{"Lambert Conformal Conic"}
                    </option>
                    <option value="naturalEarth">{"Natural Earth"}</option>
                    <option value="winkelTripel">{"Winkel Tripel"}</option>
                </select>
            </fieldset>
                <Map projection={*projection} zoom="10" center={
                    let taking_off_airport = main_data_store.airports.get(0);
                    match taking_off_airport {
                        Some(airport)=>{
                            console_log!(airport.gps_location.coordinates.0);
                            airport.gps_location.coordinates
                        },
                        None=>{
                            (0.00,0.00)
                        }
                    }
                }/>
                
            </div>
    }
}
