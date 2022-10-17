use geo::algorithm::bearing::Bearing;
use geo::{Geometry, Point};
use geojson::{Error, Feature, GeoJson, Value, FeatureCollection};
use web_sys::console;
use weblog::console_log;

use std::{result, vec};

pub fn create_json(lineString: &Feature) -> FeatureCollection{
    let line = lineString.clone().geometry.unwrap();
    let mut features:Vec<Feature> = Vec::new();
    match line.value {
        Value::LineString(line) => {
            let mut index = 0;
            for coord in line.clone() {
                let point =
                    geojson::Value::Point(vec![*coord.get(0).unwrap(), *coord.get(1).unwrap()]);
                let mut feature = Feature::from(geojson::Geometry::new(point));

                feature.set_property("altitude", *coord.get(2).unwrap());
                feature.set_property("time", *coord.get(3).unwrap());
                let pointFrom = Point::new(*coord.get(0).unwrap(), *coord.get(1).unwrap());
                let mut pointTo;
                let nextLine = line.get(index + 1).clone();
                match nextLine {
                    Some(c) => {
                        pointTo = Point::new(*c.get(0).unwrap(), *c.get(1).unwrap());
                    }
                    None => {
                        pointTo = Point::new(*coord.get(0).unwrap(), *coord.get(1).unwrap());
                    }
                }
                feature.set_property("bearing", pointFrom.bearing(pointTo));
                features.push(feature);
                index += 1;
            }
        }
        _ => {}
    }
    let collection = FeatureCollection{
        bbox:None,
        features,
        foreign_members:None
    };
    console_log!(collection.features.len());
    return collection;

}
