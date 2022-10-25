use core::time;
use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use chrono::Utc;
use geo::{LineString, ClosestPoint, Line, Within, EuclideanDistance, bearing, Bearing};
use geojson::{FeatureCollection, GeoJson, de};
use geo::algorithm::line_interpolate_point::LineInterpolatePoint;
use js_sys::Date;
use stdweb::js;
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::{use_effect_with_deps, callback};
use yew_hooks::{use_effect_update_with_deps, use_raf};
use geo::algorithm::contains::Contains;
use super::main_data::MainDataStore;
use web_sys::window;
use weblog::console_log;
use yewdux::prelude::*;

fn getClosure() -> Closure<dyn FnMut()> {
    Closure::new(|| {
        let dispatch = Dispatch::<AnimationStore>::new();
        if(dispatch.get().is_playing == true){
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
    pub current:f64,
    pub path:Option<LineString>,
    pub position: PlanePostion,
    pub lines_vec:Option<Vec<Line>>,
    last: f64
}

#[derive(Debug,Clone,Copy,Default, PartialEq)]
pub struct PlanePostion{
    pub lat:f64,
    pub lng:f64,
    pub bearing:f64
}

impl Store for AnimationStore {
    fn new() -> Self {
        Self {
            is_playing: false,
            duration: None,
            elapsed: None,
            geo_json_path: None,
            current:0.00,
            last:0.00,
            path:None,
            position:PlanePostion::default(),
            lines_vec: None,
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
}

impl Reducer<AnimationStore> for AnimationAction {
    fn apply(&self, mut state: std::rc::Rc<AnimationStore>) -> std::rc::Rc<AnimationStore> {
        let state = Rc::make_mut(&mut state);
        let dispatch:Dispatch<AnimationStore> = Dispatch::new();
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

                        let mut line:Vec<Line> = Vec::new();

                        let path = state.path.clone().unwrap();

                        for (i,l) in path.into_iter().enumerate() {
                            let p = state.path.clone().unwrap().into_inner().to_vec();
                            console_log!(i);
                            if(i > 0){
                                let start =  p.get(i-1).unwrap().clone();
                                let end = p.get(i).unwrap().clone();
                                let l = Line::new(start,end);
                                line.push(l);
                            }
                        }


                        return AnimationStore {
                            duration: Some(end_time - start_time),
                            elapsed: Some(end_time - start_time),
                            geo_json_path: state.geo_json_path.clone(),
                            is_playing: state.is_playing,
                            current:state.current,
                            last:state.last,
                            path:state.path.clone(),
                            position:state.position,
                            lines_vec:Some(line)
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
               
                let now = Utc::now().timestamp_millis() as f64;
                let delta;

                if(state.last == 0.00){
                    delta = 0.00;
                }else{
                    delta = now - state.last;
                }
                

               
               
                let phase = (state.current/state.duration.unwrap()) * 36.0;
                
                let path = state.path.clone().unwrap();
                

                let point_from_line = path.line_interpolate_point(phase).unwrap();

                let mut line_index=0;
                let mut distance: f64 = 100000000000000.0;
                state.lines_vec.as_ref().unwrap().iter()
                .enumerate()
                .for_each(|(index, line)| {
                    let dist = line.euclidean_distance(&point_from_line);
                    if (dist < distance) {
                        distance = dist;
                        line_index = index;
                    }
                });

                let current_line_segment = state.lines_vec.as_ref().unwrap().get(line_index).unwrap();
                
                let bearing = point_from_line.bearing(geo::Point(current_line_segment.end));
             
                let next_point = path.closest_point(&point_from_line);
                state.position = PlanePostion{
                    lat:point_from_line.x(),
                    lng:point_from_line.y(),
                    bearing
                };

                

                if(phase <= 1.0){
                    window().unwrap().request_animation_frame(callback.as_ref().unchecked_ref());
                }
               
                
                state.last = now;
                state.current += delta;
                callback.forget();
                AnimationStore{..state.clone()}.into()
            }
        }
    }
}

pub struct AnimationController {
    callback: Closure<dyn FnMut()>,
}

pub struct ClosureHandle(Closure<dyn FnMut()>);

impl AnimationController {
    pub fn new() -> AnimationController {
        AnimationController {
            callback: Closure::wrap(Box::new(move || {
                web_sys::console::log_1(&"raf called".into());
            }) as Box<dyn FnMut()>),
        }
    }

    pub fn play(&self) {
        
        
        // Return the ClosureHandle so the lifetime of the closure will be correct, otherwise
        // this code will panic as the closure will have been removed already.
    }
}
 