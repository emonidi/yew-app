use chrono::Utc;
use core::time::Duration;
use geo::algorithm::line_interpolate_point::LineInterpolatePoint;
use geo::{ClosestPoint, EuclideanDistance, Line, LineLocatePoint, LineString};
use geojson::{Feature, FeatureCollection, Geometry, Value};
use interpolation::lerp;
use std::{rc::Rc, vec};
use wasm_bindgen::{prelude::Closure, JsCast};

use web_sys::window;
use weblog::console_log;
use yewdux::prelude::*;

fn getClosure() -> Closure<dyn FnMut()> {
    Closure::new(|| {
        let dispatch = Dispatch::<AnimationStore>::new();
        if (dispatch.get().is_playing == true) {
            dispatch.apply(AnimationAction::Render);
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationStore {
    pub is_playing: bool,
    pub duration: Option<f64>,
    pub elapsed: Option<f64>,
    pub geo_json_path: Option<FeatureCollection>,
    pub current: f64,
    pub path: Option<LineString>,
    pub position: PlanePostion,
    pub flight_lines: Option<Vec<Feature>>,
    pub lines_vec: Option<Vec<Line>>,
    last: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct PlanePostion {
    pub lat: f64,
    pub lng: f64,
    pub bearing: f64,
}

impl Store for AnimationStore {
    fn new() -> Self {
        Self {
            is_playing: false,
            duration: None,
            elapsed: None,
            geo_json_path: None,
            current: 0.00,
            last: 0.00,
            path: None,
            position: PlanePostion::default(),
            lines_vec: None,
            flight_lines: None,
        }
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl Reducer<AnimationStore> for AnimationStore {
    fn apply(&self, mut state: Rc<AnimationStore>) -> Rc<AnimationStore> {
        todo!()
    }
}

pub enum AnimationAction {
    SetDuration,
    Play,
    Render,
    Pause,
}

impl Reducer<AnimationStore> for AnimationAction {
    fn apply(&self, mut state: std::rc::Rc<AnimationStore>) -> std::rc::Rc<AnimationStore> {
        let state = Rc::make_mut(&mut state);
        let dispatch: Dispatch<AnimationStore> = Dispatch::new();
        // let mut n_state = state.clone();
        // let n_state = Rc::get_mut(&mut n_state).unwrap();
        match self {
            AnimationAction::SetDuration => {
                match state.geo_json_path.clone() {
                    Some(p) => {
                        let start_time = p
                            .clone()
                            .features
                            .get(0)
                            .unwrap()
                            .clone()
                            .properties
                            .unwrap()
                            .get("time")
                            .unwrap()
                            .as_f64()
                            .unwrap();
                        let end_time = p
                            .clone()
                            .features
                            .last()
                            .unwrap()
                            .clone()
                            .properties
                            .unwrap()
                            .get("time")
                            .unwrap()
                            .as_f64()
                            .unwrap();

                        let mut line: Vec<Line> = Vec::new();
                        let flight_line = FeatureCollection {
                            bbox: None,
                            features: Vec::new(),
                            foreign_members: None,
                        };

                        let path = state.path.clone().unwrap();

                        for (i, l) in path.into_iter().enumerate() {
                            let p = state.path.clone().unwrap().into_inner().to_vec();

                            if (i > 0) {
                                let start = p.get(i - 1).unwrap().clone();
                                let end = p.get(i).unwrap().clone();
                                let l = Line::new(start, end);
                                line.push(l);
                            }
                        }

                        let mut geo_json_path_new: Vec<Feature> = Vec::new();
                        let features = state.clone().geo_json_path.clone().unwrap().features;
                        for (i, f) in features.iter().enumerate() {
                            if (i < features.iter().len() - 1) {
                                let mut start_point: Vec<f64> = vec![0.00, 0.00];
                                if let Value::Point(coords) =
                                    features.get(i).unwrap().geometry.clone().unwrap().value
                                {
                                    start_point = coords;
                                }
                                let mut end_point: Vec<f64> = vec![0.00, 0.00];
                                if let Value::Point(coords) =
                                    features.get(i + 1).unwrap().geometry.clone().unwrap().value
                                {
                                    end_point = coords;
                                }

                                let geom = Geometry {
                                    bbox: None,
                                    foreign_members: None,
                                    value: Value::LineString(vec![
                                        start_point as Vec<f64>,
                                        end_point,
                                    ]),
                                };
                          
                                let mut properties = serde_json::Map::new();
                                properties.insert(
                                    "altitude".to_string(),
                                    serde_json::Value::Array(vec![
                                        features
                                            .get(i)
                                            .unwrap()
                                            .properties
                                            .clone()
                                            .unwrap()
                                            .get("altitude")
                                            .unwrap()
                                            .clone(),
                                        features
                                            .get(i + 1)
                                            .unwrap()
                                            .properties
                                            .clone()
                                            .unwrap()
                                            .get("altitude")
                                            .unwrap()
                                            .clone(),
                                    ]),
                                );

                                properties.insert(
                                    "time".to_string(),
                                    serde_json::Value::Array(vec![
                                        features
                                            .get(i)
                                            .unwrap()
                                            .properties
                                            .clone()
                                            .unwrap()
                                            .get("time")
                                            .unwrap()
                                            .clone(),
                                        features
                                            .get(i + 1)
                                            .unwrap()
                                            .properties
                                            .clone()
                                            .unwrap()
                                            .get("time")
                                            .unwrap()
                                            .clone(),
                                    ]),
                                );

                                properties.insert(
                                    "bearing".to_string(),
                                    serde_json::Value::Array(vec![
                                        features
                                            .get(i)
                                            .unwrap()
                                            .properties
                                            .clone()
                                            .unwrap()
                                            .get("bearing")
                                            .unwrap()
                                            .clone(),
                                        features
                                            .get(i + 1)
                                            .unwrap()
                                            .properties
                                            .clone()
                                            .unwrap()
                                            .get("bearing")
                                            .unwrap()
                                            .clone(),
                                    ]),
                                );

                                properties.insert(
                                    "speed".to_string(),
                                    serde_json::Value::Array(vec![
                                        features
                                            .get(i)
                                            .unwrap()
                                            .properties
                                            .clone()
                                            .unwrap()
                                            .get("speed")
                                            .unwrap()
                                            .clone(),
                                        features
                                            .get(i + 1)
                                            .unwrap()
                                            .properties
                                            .clone()
                                            .unwrap()
                                            .get("speed")
                                            .unwrap()
                                            .clone(),
                                    ]),
                                );

                                let feature = Feature {
                                    bbox: None,
                                    foreign_members: None,
                                    id: None,
                                    geometry: Some(geom),
                                    properties: Some(properties),
                                };
                                
                                geo_json_path_new.push(feature);
                            }
                        }

                        return AnimationStore {
                            duration: Some(end_time - start_time),
                            elapsed: Some(end_time - start_time),
                            geo_json_path: state.geo_json_path.clone(),
                            is_playing: state.is_playing,
                            current: state.current,
                            last: state.last,
                            path: state.path.clone(),
                            position: state.position,
                            lines_vec: Some(line),
                            flight_lines: Some(geo_json_path_new),
                        }
                        .into();
                    }
                    None => console_log!("NONE"),
                }
                Rc::from(state.clone())
            }
            AnimationAction::Play => {
                state.is_playing = true;
                dispatch.apply(AnimationAction::Render);
                AnimationStore { ..state.clone() }.into()
            }
            AnimationAction::Render => {
                let callback = getClosure();
             
                if !state.is_playing {
                    callback.forget();
                    return AnimationStore{..state.clone()}.into();
                }

                let now = Utc::now().timestamp_millis() as f64;
                let mut delta;

                if (state.last == 0.00) {
                    delta = 0.00;
                } else {
                    delta = now - state.last;
                }
               
                delta = delta*16.0; 

                let phase = (state.current / state.duration.unwrap());
           
                let path = state.path.clone().unwrap();

                let point_from_line = path.line_interpolate_point(phase).unwrap();

                let mut line_index = 0;
                let mut distance: f64 = 100000000000000.0;
                state
                    .lines_vec
                    .as_ref()
                    .unwrap()
                    .iter()
                    .enumerate()
                    .for_each(|(index, line)| {
                        let dist = line.euclidean_distance(&point_from_line);
                        if (dist < distance) {
                            distance = dist;
                            line_index = index;
                        }
                    });

                let current_line_segment =
                    state.lines_vec.as_ref().unwrap().get(line_index).unwrap();
                let current_json_segment = state
                    .geo_json_path
                    .as_ref()
                    .unwrap()
                    .features
                    .get(line_index)
                    .unwrap();
                let segment_phase = current_line_segment
                    .line_locate_point(&point_from_line)
                    .unwrap();

                let current_segment_feature = state
                    .flight_lines
                    .clone()
                    .unwrap()
                    .get(line_index)
                    .unwrap()
                    .clone();
                let current_bearing_values = current_segment_feature.properties.unwrap();
                let bearing_props = current_bearing_values
                    .get("bearing")
                    .unwrap()
                    .as_array()
                    .unwrap();
                let bearing = lerp(
                    &bearing_props.get(0).unwrap().as_f64().unwrap(),
                    &bearing_props.get(1).unwrap().as_f64().unwrap(),
                    &segment_phase,
                );
                
                // console_log!(bearing_props.get(0).unwrap().as_f64().unwrap(),bearing_props.get(1).unwrap().as_f64().unwrap());
               
                state.position = PlanePostion {
                    lat: point_from_line.x(),
                    lng: point_from_line.y(),
                    bearing: bearing
                };

             

                if (phase <= 1.0) {
                    let val = window()
                        .unwrap()
                        .request_animation_frame(callback.as_ref().unchecked_ref());
                    
                    state.last = now;
                    state.current += delta;
                } else {
                    state.current = 0.00;
                    state.last = 0.00;
                    state.is_playing = false;
                }

                callback.forget();
                AnimationStore { ..state.clone() }.into()
            }
            AnimationAction::Pause => {
                state.is_playing = false;
                state.last = 0.00;
                AnimationStore{..state.clone()}.into()
            },
          
        }
    }
}
