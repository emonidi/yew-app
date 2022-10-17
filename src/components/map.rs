use crate::interop;
use interop::use_mapbox;
use stdweb::js;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::{use_effect};

#[derive(Properties, PartialEq)]
pub struct MapProps{
    pub projection:AttrValue, 
    pub zoom: AttrValue, 
    pub center: (f64,f64)
}

#[function_component(Map)]
pub(crate) fn map(props:&MapProps) -> Html {

    let map_ready = use_mapbox();
    let map_ref = use_node_ref();
    
    {
        let projection = props.projection.clone();
        use_effect_with_deps(move |projectection|{
            let proj = projectection.clone().to_string();
            js! {
                if(window.mapInstance){
                    mapInstance.setProjection(@{proj})
                }
            } || {}
        } , projection)
    }

    {
        let center = props.center.clone();
        use_effect_with_deps(|center|{
            js! {
                if(window.mapInstance){
                    mapInstance.setCenter([@{center.0},@{center.1}]);
                }
            } || {}
        }, center)
    }
 
    {
        let proj = props.projection.clone().into_string();
        let zoom = props.zoom.clone().into_string();
        let center = props.center.clone();
        let map_ref = map_ref.clone();
        use_effect(
            move || {
                
                 let map_div = map_ref.cast::<HtmlElement>();
                 match map_div {
                    HtmlElement => {
                       
                        js!{
                            console.log(window.mapInstance);
                            if(window.mapboxgl && !window.mapInstance){
                               mapboxgl.accessToken = "pk.eyJ1IjoiZW1vbmlkaSIsImEiOiJjajdqd3pvOHYwaThqMzJxbjYyam1lanI4In0.V_4P8bJqzHxM2W9APpkf1w";
                               window.mapInstance = new mapboxgl.Map({
                                    container:"map",
                                    style: "mapbox://styles/mapbox/satellite-streets-v11",
                                    center:[@{center.0},@{center.1}],
                                    zoom: @{zoom},
                                    projection:@{proj}
                               });
                            }
                        }
                    },
                    _ => {

                    }
                 }
                 
                || {}
            }
        );
    }

    html! {
       if(map_ready){
           <div ref={map_ref} id="map" class="w-full min-h-screen" style="min-height:100vh"></div>
       }else{
            <h1>{"LOADING MAP...."}</h1>
       }
    }
}
