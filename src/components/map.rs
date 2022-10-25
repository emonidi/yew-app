use crate::helpers::path_2d::GeoPath2D;
use crate::stores::animation::AnimationAction;
use crate::stores::animation::AnimationStore;
use crate::stores::main_data::MainDataStore;
use geojson::FeatureCollection;
use stdweb::js;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::HtmlElement;

use web_sys::window;
use weblog::console_log;
use yew::prelude::*;
use yew::use_effect;
use yew::virtual_dom::AttrValue;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct MapProps {
    pub projection: AttrValue,
    pub zoom: AttrValue,
    pub center: (f64, f64),
    pub path_2d: Option<FeatureCollection>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Map)]
pub(crate) fn map(props: &MapProps) -> Html {
    let map_loaded = use_state(|| false);
    let (main_data_store, _dispatch) = use_store::<MainDataStore>();
    
   
    

    // let map_ready = use_mapbox();
    let map_ref = use_node_ref();
    {
        let projection = props.projection.clone();
        use_effect_with_deps(
            move |projectection| {
                let proj = projectection.clone().to_string();
                js! {
                    if(window.mapInstance){
                        mapInstance.setProjection(@{proj})
                    }
                }
                || {}
            },
            projection,
        )
    }

    {
        use_effect_with_deps(
            |center_2d| {
                let center = center_2d.clone();
                js! {

                    if(window.mapInstance){

                        mapInstance.setCenter([@{center.0},@{center.1}]);
                    }
                }
                || {}
            },
            main_data_store.center_2d,
        )
    }

    {
        use_effect_with_deps(
            move |main_data_store| {
                let airports = main_data_store.airports.clone();
                let geo_json_path = main_data_store.geo_json_line.clone();
                let is2d = main_data_store.is2d.clone();
                let center_2d = main_data_store.center_2d;
                if (airports.len() > 0) {
                    let path_2d = GeoPath2D::new(airports, geo_json_path, center_2d);
                    path_2d.get();
                };
                || ()
            },
            main_data_store.clone(),
        )
    }

    
    {
        use_effect_with_deps(
            move |(main_data_store, _map_loaded)| {
                let bounds = main_data_store.bounds.clone();

                match bounds {
                    Some(bounds) => {
                        // let b = vec![bounds.min().x,bounds.min().y,bounds.max().x,bounds.max().y];
                        js! {

                           if(window.mapInstance){
                            mapInstance.setZoom(1);
                            mapInstance.fitBounds([
                                @{bounds.min().x},
                                @{bounds.min().y},
                                @{bounds.max().x},
                                @{bounds.max().y},
                            ], { padding: 50, duration: 2000 });
                           }
                        }
                    }
                    None => {
                        js! {
                            if(window.mapInstance){

                                mapInstance.zoomTo(12,{duration:2000});
                            }
                        }
                    }
                }
                || ()
            },
            (main_data_store.clone(), map_loaded),
        )
    }

    {
        let proj = props.projection.clone().into_string();
        let zoom = props.zoom.clone().into_string();
        let center = props.center.clone();
        let map_ref = map_ref.clone();
        use_effect(move || {
            let map_div = map_ref.cast::<HtmlElement>();
            match map_div {
                _html_element => {
                    js! {
                        if(window.mapboxgl && !window.mapInstance){


                            mapboxgl.accessToken = "pk.eyJ1IjoiZW1vbmlkaSIsImEiOiJjajdqd3pvOHYwaThqMzJxbjYyam1lanI4In0.V_4P8bJqzHxM2W9APpkf1w";
                            window.mapInstance = new mapboxgl.Map({
                                 container:"map",
                                 style: {
                                     version:8,
                                     sources:{
                                         "map-tiler":{
                                             type:"raster",
                                             tiles:[
                                                 "https://api.mapbox.com/styles/v1/mapbox/streets-v9/tiles/{z}/{x}/{y}?access_token=pk.eyJ1IjoiZW1vbmlkaSIsImEiOiJjajdqd3pvOHYwaThqMzJxbjYyam1lanI4In0.V_4P8bJqzHxM2W9APpkf1w"
                                             ]
                                         }
                                     },
                                     layers:[
                                         {
                                             id: "map-tiler",
                                             source: "map-tiler",
                                             type: "raster"
                                         },
                                     ]
                                 },
                                 center:[@{center.0},@{center.1}],
                                 zoom: @{zoom},
                                 projection:@{proj}
                            });
                         }
                    }
                }
            }

            || ()
        });
    }

    html! {
        <div ref={map_ref} id="map" class="w-full min-h-screen" style="min-height:100vh">{props.children.clone()}</div>
    }
}
