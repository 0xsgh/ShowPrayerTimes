use clap::Parser;
use chrono::prelude::*;
use serde_json::Value;

#[derive(Debug, Parser)]
struct CLIArguments
{
    /// The longitude of the location for which the times should be fetched
    #[arg(short = 'g', long)]
    longitude: String,
    
    /// The latitude of the location for which the times should be fetched
    #[arg(short = 'a', long)]
    latitude: String
}

fn main()
{
    let cli_args = CLIArguments::parse();

    match show_prayer_times( &cli_args.longitude, &cli_args.latitude )
    {
        Ok(_) => (),
        Err(_) => println!( "Could not fetch prayer times!" )
    };
}

fn show_prayer_times( longitude_string: &str, latitude_string: &str ) -> Result<(), Box<dyn std::error::Error>>
{
    let base_url = "http://api.aladhan.com/v1/timings";

    let local_date_time = Local::now();

    let date_today = std::format!( "{:02}-{:02}-{:04}", local_date_time.day(), local_date_time.month(), local_date_time.year() );
    let longitude_component = std::format!( "longitude={}", longitude_string );
    let latitude_component = std::format!( "latitude={}", latitude_string );
    let method_component = "method=2";

    let request_url = std::format!( "{}/{}?{}&{}&{}", base_url, date_today, latitude_component, longitude_component, method_component );

    let response_string = reqwest::blocking::get( request_url )?.text()?;

    let json_value = serde_json::from_str( &response_string )?;

    let mut prayer_times_list = get_prayer_timings( &json_value );

    prayer_times_list.sort_by( |first, second|
                                {
                                    first.1.cmp( &second.1 )
                                } );

    println!( "\nPrayer times for {}:\n", date_today );
    for ( prayer_name, time_string ) in prayer_times_list
    {
        println!( "    {:>10} : {}", prayer_name, time_string );
    }
    println!();

    Ok(())
}

fn get_prayer_timings( json_response: &Value ) -> Vec<(String, String)>
{
    let data_map_key = "data";

    if let Value::Object( top_level_map ) = json_response
    {
        if top_level_map.contains_key( data_map_key )
        {
            match top_level_map.get( data_map_key )
            {
                Some( data_map ) => return extract_timings_list( data_map ),
                None => ()
            };
        }
    }
    
    Vec::<(String, String)>::new()
}

fn extract_timings_list( data_map: &Value ) -> Vec<(String, String)>
{
    let timings_map_key = "timings";

    if let Value::Object( data_map_entries ) = data_map
    {
        if data_map_entries.contains_key( timings_map_key )
        {
            match data_map_entries.get( timings_map_key )
            {
                Some( timings_map_as_value ) =>
                {
                    if let Value::Object( timings_map ) = timings_map_as_value
                    {
                        let mut timings_list = Vec::<(String, String)>::new();
            
                        for ( prayer_name, time_value ) in timings_map
                        {
                            timings_list.push( ( prayer_name.clone(), get_string_value( time_value ) ) );
                        }
            
                        return timings_list;
                    }
                },
                None => ()
            }
        }
    }
    
    Vec::<(String, String)>::new()
}

fn get_string_value( json_value: &Value ) -> String
{
    if let Value::String( string_value ) = json_value
    {
        string_value.clone()
    }
    else
    {
        String::from( "" )
    }
}