use geo::HaversineDestination;
use geo::geometry::Point;
use geojson::{Feature, GeoJson, Value};

pub fn circle(center:Point, radius:i128,steps:i32)-> GeoJson{
    
    let mut coordinates:Vec<Vec<f64>> = Vec::new();

    (0..steps).for_each(|i| {
        let destination = center.haversine_destination(((i as f64)* - 360f64)/(steps as f64), (radius * 1000) as f64);
        coordinates.push(vec![destination.0.x,destination.0.y]);
       
    });
    let initial_coord = coordinates.get(0).unwrap().clone();
    coordinates.push(initial_coord);

    // let line = LineString::from(coordinates);
    // let polygon = Polygon::new(line,vec![]);
    let geometry = geojson::Geometry::new(Value::Polygon(vec![coordinates]));
    GeoJson::Feature(Feature{
        bbox:None,
        geometry:Some(geometry),
        id:None,
        properties:None, 
        foreign_members:None
    })
}