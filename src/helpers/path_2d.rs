use super::cirlcle::circle;
use crate::domain::json_data::Airport;
use geo::point;
use geojson::{Feature, GeoJson, Geometry, Value};
use stdweb::{js, JsSerialize};

#[derive(Debug, Clone)]
enum PathType {
    NONE,
    LINE,
    CIRCLE,
    FLIGHT,
}

#[derive(Debug, Clone)]
pub struct GeoPath2D {
    airports: Vec<Airport>,
    geo_json_path: Option<Feature>,
    center_2d: (f64, f64),
    path_type: PathType,
}

impl GeoPath2D {
    pub fn new(
        airports: Vec<Airport>,
        geo_json_path: Option<Feature>,
        center_2d: (f64, f64),
    ) -> GeoPath2D {
        let takeoff_airport = airports.get(0).unwrap();
        let landing_airport = airports.get(1).unwrap();

        let mut path_type;

        match geo_json_path.clone() {
            Some(path) => {
                path_type = PathType::FLIGHT;
            }
            None => {
                if (takeoff_airport.id == landing_airport.id) {
                    path_type = PathType::CIRCLE;
                } else {
                    path_type = PathType::LINE;
                }
            }
        }

        GeoPath2D {
            path_type,
            airports,
            geo_json_path,
            center_2d,
        }
    }

    fn circle(&self) -> (GeoJson, String) {
        let circle = circle(point! {x:self.center_2d.0, y: self.center_2d.1}, 5, 64);
        (circle, "circle_path_2d".to_string())
    }

    fn line(&self) -> (GeoJson, String) {
        let start_point = self.airports.get(0).unwrap().gps_location.coordinates;
        let end_point = self.airports.get(1).unwrap().gps_location.coordinates;
        let geometry = Geometry::new(Value::LineString(vec![
            vec![start_point.0, start_point.1],
            vec![end_point.0, end_point.1],
        ]));
        let geo_json = GeoJson::Feature(Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            properties: None,
            foreign_members: None,
        });

        (geo_json, "line_path_2d".to_string())
    }

    fn flight(&self) -> (GeoJson, String) {
        return (
            geojson::GeoJson::Feature(self.geo_json_path.clone().unwrap()),
            "flight_path_2d".to_string(),
        );
    }

    pub fn get(&self) {
        let mut geometry: GeoJson;
        let mut id: String;
        
        match self.path_type {
            PathType::FLIGHT => {
                let flight = self.flight();
                geometry = flight.0;
                id = flight.1;
            }
            PathType::NONE => todo!(),
            PathType::LINE => {
                let line = self.line();
                geometry = line.0;
                id = line.1;
            }
            PathType::CIRCLE => {
                let line = self.circle();
                geometry = line.0;
                id = line.1;
            }
        }
        js! {
          
          if(window.mapInstance){

            ["flight_path_2d","line_path_2d","circle_path_2d"].forEach(id=>{
                if(mapInstance.getSource(id)){
                    mapInstance.removeLayer(id);
                    mapInstance.removeSource(id);
    
                 }
            });
            
           

             mapInstance.addSource(@{id.clone()},{
                type:"geojson",
                data:JSON.parse(@{GeoJson::from(geometry).to_string()})
             });

             let layer = mapInstance.addLayer({
                "id":@{id.clone()},
                "type":"line",
                "source":@{id.clone()},
                "paint":{
                    "line-color":"#000",
                    "line-width":3
                }
             });

         
          }
        }
    }
}
