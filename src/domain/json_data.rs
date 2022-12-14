
use serde::Deserialize;

#[derive(Clone,PartialEq,Deserialize, Debug)]
pub struct GpsLocation{
    pub coordinates:(f64,f64)
}

#[derive(Clone,PartialEq,Deserialize, Debug)]
pub struct Airport{
    pub city:String,
    pub continent:String,
    pub elevation_ft:String,
    // pub external_id:usize,
    pub gps_code:String,
    pub gps_location:GpsLocation,
    pub id:String
}

#[derive(Clone,PartialEq,Deserialize)]
pub struct JsonData{
    pub gps_data_url:Option<String>, 
    pub  landing_airport:Airport,
    pub takeoff_airport:Airport
}
