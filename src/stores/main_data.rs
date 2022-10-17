use geo::{LineString, coord};
use geojson::FeatureCollection;
use yewdux::prelude::*;
use crate::domain::json_data::Airport;



#[derive(Debug,Clone,PartialEq,Store)]
pub struct MainDataStore{
    pub airports:Vec<Airport>,
    pub path:LineString<f64>,
    pub geo_json_path:Option<FeatureCollection>,
    pub geo_json_line:Option<geojson::Feature>
}

impl Default for MainDataStore {
    fn default() -> Self {
        Self { 
            airports: Default::default(), 
            path: LineString::from(vec![coord!{x:0.00,y:0.00},coord!{x:1f64,y:1f64}]),
            geo_json_path: None,
            geo_json_line: None
        }
    }
}