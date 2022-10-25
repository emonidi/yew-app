use std::rc::Rc;

use crate::{domain::json_data::Airport};
use geo::{coord, LineString, Rect};
use geojson::FeatureCollection;
use weblog::console_log;
use yewdux::prelude::*;

use super::animation::AnimationStore;

#[derive(Debug, Clone, PartialEq)]
pub struct MainDataStore {
    pub airports: Vec<Airport>,
    pub path: LineString<f64>,
    pub geo_json_path: Option<FeatureCollection>,
    pub geo_json_line: Option<geojson::Feature>,
    pub center_2d: (f64, f64),
    pub bounds: Option<Rect>,
    pub is2d: bool,
    
}

impl MainDataStore {
   
}

impl Default for MainDataStore {
    fn default() -> Self {
        Self {
            airports: Default::default(),
            path: LineString::from(vec![coord! {x:0.00,y:0.00}, coord! {x:1f64,y:1f64}]),
            geo_json_path: None,
            geo_json_line: None,
            center_2d: (0.00, 0.00),
            bounds: None,
            is2d: true,
        }
    }
}

impl Store for MainDataStore {
    fn new() -> Self {
        init_listener(MainDataStoreListener);
        Self {
            airports: Default::default(),
            path: LineString::from(vec![coord! {x:0.00,y:0.00}, coord! {x:1f64,y:1f64}]),
            geo_json_path: None,
            geo_json_line: None,
            center_2d: (0.00, 0.00),
            bounds: None,
            is2d: true,
        }
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}


impl Reducer<MainDataStore> for MainDataStore{

    fn apply(&self, state: Rc<MainDataStore>) -> Rc<MainDataStore> {
        console_log!("APPLY!");
        let animation_dispatch = Dispatch::<AnimationStore>::new();
        let path = &state.geo_json_path;
        match path {
            Some(p) => {
                console_log!("YEY!");
            }
            None => {
                console_log!("AAAAAW!")
            },
        }
        state.clone()
    }
}

struct MainDataStoreListener;
impl Listener for MainDataStoreListener {
    type Store = MainDataStore;

    fn on_change(&mut self, _state: std::rc::Rc<Self::Store>) {
       
    }
}
