use geo::{line_string, Centroid, LineString, BoundingRect, Rect};
use crate::domain::json_data::Airport;

pub fn get_center_from_airports(airports:Vec<Airport>,path:Option<LineString<f64>>) -> ((f64,f64),Option<Rect>){
    match path {
        Some(p)=>{
            let ret = p.centroid().unwrap();
            ((ret.0.x,ret.0.y),Some(p.bounding_rect().unwrap()))
        },
        None=>{
            if airports.get(0).unwrap().id != airports.get(1).unwrap().id{
        
                let line_string  = line_string![
                    (
                        x: airports.get(0).unwrap().gps_location.coordinates.0,
                        y: airports.get(0).unwrap().gps_location.coordinates.1
                    ),
                    (
                        x:airports.get(1).unwrap().gps_location.coordinates.0,
                        y:airports.get(1).unwrap().gps_location.coordinates.1
                    )
                ];
                let ret = line_string.centroid().unwrap();
                
                ((ret.0.x,ret.0.y),Some(line_string.bounding_rect().unwrap()))
            }else{
                let airport = airports.get(0).unwrap().gps_location.coordinates;
                ((airport.0,airport.1),None)
            }
        }
    }
    
}