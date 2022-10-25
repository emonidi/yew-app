use crate::stores::{main_data::MainDataStore, animation::AnimationStore};
use geo::{point, Bearing, Point};
use stdweb::{
    js,
    web::{document, window},
};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlImageElement;
use weblog::console_log;
use yew::{function_component, html, use_effect_with_deps, use_node_ref};
use yewdux::prelude::use_store;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = window)]
    #[wasm_bindgen(js_namespace = mapboxgl)]
    type Marker;

    #[wasm_bindgen(constructor)]
    fn new() -> Marker;
    #[wasm_bindgen(method)]
    fn setLngLat(this: &Marker, val: Vec<f64>);
    #[wasm_bindgen(method)]
    fn addTo(this: &Marker, val: String);
}

#[function_component(Plane2d)]
pub fn plane_2d() -> Html {
    let image = use_node_ref();
    let (main_data_store, _dispatch) = use_store::<MainDataStore>();
    let (animation_store, animation_dispatch) = use_store::<AnimationStore>();
    {
        let image = image.clone();
        use_effect_with_deps(
            move |image| {
                js! {
                    let interval = setInterval(()=>{
                        if(mapInstance){
                          let image = new Image();
                          image.classList.add("w-20");
                          image.src = "/assets/models/svg_plane.svg";

                          window.plane_2d_marker = new mapboxgl.Marker(image);
                          console.log(window.plane_2d_marker);
                          clearInterval(interval);
                        }

                    },100)
                }
                || ()
            },
            (),
        );
    }

    {
        let animation_store = animation_store.clone();  
        use_effect_with_deps(move |position|{
            js!{
                if(window.plane_2d_marker){
                    plane_2d_marker.setLngLat([@{position.lat},@{position.lng}]);
                    // plane_2d_marker.addTo(mapInstance);
                    plane_2d_marker.setRotation(@{position.bearing});
                }
            }
            || ()
        }, animation_store.position);
    }


    {
        let geo_json_path = main_data_store.geo_json_path.clone();
        let path = main_data_store.path.clone();
        use_effect_with_deps(
            move |(path, geo_json_path)| {
                let coords = path.0.get(0).unwrap();
                let next_coords = path.0.get(1).unwrap();
                let bearing = point! {x:coords.x,y:coords.y}
                    .bearing(point! {x:next_coords.x,y:next_coords.y});
                js! {
                   if(!window.plane_2d_marker){
                    let interval = setInterval(()=>{
                        if(window.plane_2d_marker){
                            plane_2d_marker.setLngLat([@{coords.x},@{coords.y}]);
                            plane_2d_marker.addTo(mapInstance);
                            plane_2d_marker.setRotation(@{bearing});
                            clearInterval(interval);
                        }
                    },100)
                   }else{
                    if(@{coords.y} !== 0){
                        plane_2d_marker.setLngLat([@{coords.x},@{coords.y}]);
                        plane_2d_marker.addTo(mapInstance);
                        plane_2d_marker.setRotation(@{bearing});
                    }
                   }
                }
                match geo_json_path {
                    None => {
                        js!{
                            let interval = setInterval(()=>{
                                if(window.plane_2d_marker){
                                    plane_2d_marker.remove();
                                    clearInterval(interval);
                                }
                            },100)
                        }
                    }
                    _ => {}
                }
                || ()
            },
            (path, geo_json_path),
        )
    }

    html! {}
}
