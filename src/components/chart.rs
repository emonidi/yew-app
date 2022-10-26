use std::{borrow::Cow, fmt::Debug, ops::Deref, process::Command};

use geo::{point, LineString, Point};
use scales::prelude::*;
use stdweb::web::window;
use svg::node::element::{path::Data, Path};

use web_sys::Document;
use weblog::console_log;
use yew::{prelude::*, virtual_dom::AttrValue};
use yewdux::prelude::use_store;

use crate::stores::{animation::AnimationStore, main_data::MainDataStore};

#[derive(Properties, PartialEq, Clone)]
pub struct ChartProps {
    pub width: i32,
    pub height: i32,
}

#[function_component(Chart)]
pub(crate) fn chart(props: &ChartProps) -> Html {
    let props = props.clone();
    let (main_data_store, main_data_dispatch) = use_store::<MainDataStore>();
    let (animation_store, animation_dispatch) = use_store::<AnimationStore>();
    let points: UseStateHandle<Vec<(Point<f64>, Point<f64>)>> =
        use_state(|| vec![(point! {x:0.00,y:0.00}, point! {x:0.00,y:0.00})]);
    let alt_line: UseStateHandle<String> = use_state(|| String::new());
    let speed_line: UseStateHandle<String> = use_state(|| String::new());
    let control_line_x = use_state(|| 0_f64);

    {
        let points = points.clone();
        let path = main_data_store.geo_json_path.clone();
        let alt_line_clone = alt_line.clone();
        let speed_line_clone = speed_line.clone();
        use_effect_with_deps(
            move |path| {
                if let Some(path) = path {
                    let mut t: Vec<f64> = path
                        .features
                        .iter()
                        .map(|f| {
                            f.properties
                                .clone()
                                .unwrap()
                                .get("time")
                                .unwrap()
                                .as_f64()
                                .unwrap()
                        })
                        .collect();

                    let mut sorted_time = Vec::from(t);
                    sorted_time.sort_by(|a, b| a.partial_cmp(b).unwrap());

                    let mut alt: Vec<f64> = path
                        .features
                        .iter()
                        .map(|f| {
                            f.properties
                                .clone()
                                .unwrap()
                                .get("altitude")
                                .unwrap()
                                .as_f64()
                                .unwrap()
                        })
                        .collect();

                    let mut sorted_alt = Vec::from(alt);
                    sorted_alt.sort_by(|a, b| b.partial_cmp(a).unwrap());

                    let mut speed: Vec<f64> = path
                        .features
                        .iter()
                        .map(|f| {
                            f.properties
                                .clone()
                                .unwrap()
                                .get("speed")
                                .unwrap()
                                .as_f64()
                                .unwrap()
                        })
                        .collect();

                    let mut sorted_speed = Vec::from(speed);
                    sorted_speed.sort_by(|a, b| b.partial_cmp(a).unwrap());

                    let y_scale = LinearScale::new(
                        *sorted_alt.get(sorted_alt.len() - 1).unwrap(),
                        sorted_alt[0],
                    );
                    let x_scale = LinearScale::new(
                        *sorted_time.get(sorted_time.len() - 1).unwrap(),
                        *sorted_time.get(0).unwrap(),
                    );

                    let speed_scale = LinearScale::new(
                        *sorted_speed.get(sorted_speed.len() - 1).unwrap(),
                        *sorted_speed.get(0).unwrap(),
                    );

                    let chart_points: Vec<(Point<f64>, Point<f64>)> = path
                        .features
                        .iter()
                        .map(move |f| {
                            let alt = f
                                .properties
                                .clone()
                                .unwrap()
                                .get("altitude")
                                .unwrap()
                                .as_f64()
                                .unwrap();
                            let time = f
                                .properties
                                .clone()
                                .unwrap()
                                .get("time")
                                .unwrap()
                                .as_f64()
                                .unwrap();
                            let speed = f
                                .properties
                                .clone()
                                .unwrap()
                                .get("speed")
                                .unwrap()
                                .as_f64()
                                .unwrap();

                            let x: f64 = x_scale.to_relative(time) * (props.width as f64 - 20.0);

                            let y = (props.height as f64 - 20.0)
                                - y_scale.to_relative(alt) * (props.height as f64 - 20.0);

                            let speed_y = (props.height as f64 - 20.0)
                                - speed_scale.to_relative(speed) * (props.height as f64 - 20.0);

                            return (point! {x:x,y:y}, point! {x:x,y:speed_y});
                        })
                        .collect();

                    points.set(chart_points.clone());

                    let mut path_data: Vec<String> = vec![];
                    let mut speed_data: Vec<String> = vec![];

                    chart_points.iter().enumerate().for_each(|(index, point)| {
                        if index == 0 {
                            path_data.push(
                                format!("M {} {}", point.0.x() + 10.0, point.0.y() + 10.0)
                                    .to_string(),
                            );
                            speed_data.push(
                                format!("M {} {}", point.1.x() + 10.0, point.1.y() + 10.0)
                                    .to_string(),
                            )
                        } else {
                            path_data.push(
                                format!("L {} {}", point.0.x() + 10.0, point.0.y() + 10.0)
                                    .to_string(),
                            );
                            speed_data.push(
                                format!("L {} {}", point.1.x() + 10.0, point.1.y() + 10.0)
                                    .to_string(),
                            )
                        }
                    });

                    alt_line_clone.set(path_data.join("").clone());
                    speed_line_clone.set(speed_data.join("").clone());
                }
                || ()
            },
            path,
        )
    }

    {
        let current_time = animation_store.clone().current.clone();
        let duration = animation_store.clone().duration;
        let control_line_x_clone = control_line_x.clone();
        let width = props.width.clone() as f64 - 20.0;
        if let Some(duration) = duration {
            let time_scale = LinearScale::new(0.00,duration);
            use_effect_with_deps(
               move |time| {
                    let x = time_scale.to_relative(*time);
                    control_line_x_clone.set((x*width)+10.0);
                    || ()
                },
                current_time,
            )
        }
    }

    html! {
        <div class="z-10 w-full min-h-min fixed bottom-0 bg-white">

            <svg width={props.width.to_string()} height={props.height.to_string()}>
                <g class="points">
                    // {
                    //     points.iter().map(|p|{
                    //         html!{
                    //             <circle cx={(p.x()+10.0).to_string()} cy={(p.y()+10.0).to_string()} r="2"></circle>
                    //         }
                    //   }).collect::<Html>()
                    // }
                </g>
                <g class="alt_line">
                    {
                        html!{
                            <path stroke-width="1.5" stroke="rgb(63, 216, 152)" fill="rgba(102, 193, 203, 0.08)" d={AttrValue::from(String::new()+&*alt_line)} />
                        }
                    }
                </g>

                <g class="speed_line">
                {
                    html!{
                        <path stroke-width="1.5" stroke="rgb(255, 86, 96)" fill="rgba(253, 53, 53, 0.08)" d={AttrValue::from(String::new()+&*speed_line)} />
                    }
                }
                </g>

                <line 
                    stroke-width="1.5"
                    stroke="black"
                    x1={AttrValue::from(String::new()+&*control_line_x.to_string())} 
                    x2={AttrValue::from(String::new()+&*control_line_x.to_string())}
                    y1={10}
                    y2={AttrValue::from(String::new()+&*(props.height as f64-10.0).to_string())}
                />
               
            </svg>
        </div>
    }
}
