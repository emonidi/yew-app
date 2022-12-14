use crate::{
    components::map::Map,
    domain::json_data::JsonData,
    helpers::{create_json::create_json, get_center_from_airports::get_center_from_airports},
    stores::{main_data::MainDataStore, animation::AnimationStore},
    components::plane_2d::Plane2d,
    components::chart::Chart,
    stores::animation::{AnimationAction}
};
use geo::{LineString};
use gloo_net::http::Request;
use js_sys::*;
use web_sys::window;
use weblog::console_log;
use yew::{prelude::*, virtual_dom::AttrValue};
use yewdux::functional::use_store;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub flight_id: String,
    pub pilot_id: String,
}

#[function_component(Home)]
pub(crate) fn home(props: &HomeProps) -> Html {
    let (main_data_store, dispatch) = use_store::<MainDataStore>();
    let (animation_flight_store, animation_dispatch) = use_store::<AnimationStore>(); 
    let pilot_id = props.pilot_id.clone();
    let flight_id = props.flight_id.clone();
    let button_text = use_state(|| "Play");

    let on_btn_click = {
        let animation_dispatch = animation_dispatch.clone();
        let button_text_clone = button_text.clone();
        Callback::from(move |_|{
            // animation_dispatch.reduce_mut(|state|{
            //     state.current =  state.duration.unwrap() - js_sys::Math::random() * state.duration.unwrap();
            //     console_log!(state.duration.unwrap(),state.current);
            // });
            // animation_dispatch.apply(AnimationAction::Render);
            if(animation_flight_store.is_playing == false){
                animation_dispatch.apply(AnimationAction::Play);
                button_text_clone.set("Pause");
                animation_dispatch.apply(AnimationAction::Play);
            }else{
                animation_dispatch.apply(AnimationAction::Pause);
                button_text_clone.set("Play")
            }
            
        })
    };
    
    {
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let data:JsonData = Request::get(
                        // &format!("https://api.allorigins.win/raw?url=https://api.followingpilots.com/api/user/public/{}/flyover/{}/?format=json",pilot_id,flight_id)
                        "https://localhost:8000/assets/flights/main_data.json"
                    )
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

                    dispatch.reduce_mut(|state| {
                        state.airports.push(data.takeoff_airport);
                        state.airports.push(data.landing_airport);
                    });

                    match data.gps_data_url {
                        Some(url) => {
                            if url != "" {
                                let data: geojson::FeatureCollection = Request::get(
                                    // format!("https://api.allorigins.win/raw?url={}", &url).as_str(),
                                    "https://localhost:8000/assets/flights/flight.json"
                                )
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();

                                let path_geo_json = data.features.get(1).unwrap();
                                let path_geom: LineString<f64> =
                                    LineString::try_from(path_geo_json.geometry.clone().unwrap())
                                        .unwrap();
                                let interpolated = path_geom;

                                let json_collection = create_json(path_geo_json);

                                dispatch.reduce_mut(|state| {
                                    let interpolated = interpolated.clone();
                                    state.path = interpolated;
                                    state.geo_json_path = Some(json_collection.clone());
                                    state.geo_json_line = Some(path_geo_json.clone());
                                    let (center_2d, bounds) = get_center_from_airports(
                                        (*state.airports).to_vec(),
                                        Some(state.path.clone()),
                                    );
                                    state.center_2d = center_2d;
                                    state.bounds = bounds;
                                });

                                animation_dispatch.reduce_mut(|state|{
                                    state.geo_json_path =   Some(json_collection.clone());
                                    state.path = Some(interpolated);
                                });

                                animation_dispatch.apply(AnimationAction::SetDuration)

                               
                                
                            } else {
                                dispatch.reduce_mut(|state| {
                                    let (center_2d, bounds) =
                                        get_center_from_airports((*state.airports).to_vec(), None);
                                    state.center_2d = center_2d;
                                    state.bounds = bounds;
                                    
                                });
                            }
                        }
                        _ => {
                            dispatch.reduce_mut(|state| {
                                let (center_2d, bounds) =
                                    get_center_from_airports((*state.airports).to_vec(), None);
                                state.center_2d = center_2d;
                                state.bounds = bounds;
                            });
                        }
                    }
                });
               
                || ()
            },
            (),
        );
    }
  
    html! {
            <div>
                <button onclick={on_btn_click} class="absolute ml-0 mt-0 z-20 bg-white px-5 py-3">{*button_text}</button>
                <Map projection={"globe"} zoom="1" center={main_data_store.center_2d}>
                    <Plane2d/>
                </Map>
                <Chart width={window().unwrap().inner_width().unwrap().as_f64().unwrap() as i32} height={190}/>
                
            </div>
    }
}
