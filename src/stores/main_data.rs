use crate::{domain::json_data::Airport};
use geo::{coord, LineString, Rect};
use geojson::FeatureCollection;
use yewdux::prelude::*;

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

// impl Reducer<MainDataStore> for MainDataStore{

//     fn apply(&self, state: Rc<MainDataStore>) -> Rc<MainDataStore> {
//         console_log!("REDUCER APPLY!");
//         if state.airports.len() > 1 {
//             let line_string  = line_string![
//                 (
//                     x: state.airports.get(0).unwrap().gps_location.coordinates.0,
//                     y: state.airports.get(0).unwrap().gps_location.coordinates.1
//                 ),
//                 (
//                     x: state.airports.get(1).unwrap().gps_location.coordinates.0,
//                     y:state.airports.get(0).unwrap().gps_location.coordinates.1
//                 )
//             ];
//             let center = line_string.centroid().unwrap();
//             // let state = Rc::make_mut(&mut state);
//             let state = &*state;
//             console_log!(center.0.x,center.0.y);
//             Rc::new({
//                 MainDataStore {
//                     airports:(*state.airports).to_vec(),
//                     path: state.path.clone(),
//                     geo_json_path: state.geo_json_path.clone(),
//                     geo_json_line:self.geo_json_line.clone(),
//                     center_2d: (center.0.x,center.0.y)}
//             })
//         }else{
//             state
//         }
//     }
// }

struct MainDataStoreListener;
impl Listener for MainDataStoreListener {
    type Store = MainDataStore;

    fn on_change(&mut self, _state: std::rc::Rc<Self::Store>) {
        
    }
}
