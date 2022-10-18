use geo::algorithm::bearing::Bearing;
use geo::Point;
use geojson::{Feature, Value, FeatureCollection};
use weblog::console_log;
use std::vec;

pub fn create_json(line_string: &Feature) -> FeatureCollection{
    let line = line_string.clone().geometry.unwrap();
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
                let point_from = Point::new(*coord.get(0).unwrap(), *coord.get(1).unwrap());
                let point_to;
                let next_line = line.get(index + 1).clone();
                match next_line {
                    Some(c) => {
                        point_to = Point::new(*c.get(0).unwrap(), *c.get(1).unwrap());
                    }
                    None => {
                        point_to = Point::new(*coord.get(0).unwrap(), *coord.get(1).unwrap());
                    }
                }
                feature.set_property("bearing", point_from.bearing(point_to));
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
