use std::collections::HashMap;

use substreams::errors::Error;
use substreams_antelope::Block;
use substreams_sink_prometheus::{PrometheusOperations, Gauge};
use crate::abi;

#[substreams::handlers::map]
pub fn prom_out(block: Block) -> Result<PrometheusOperations, Error> {

    let mut prom_out = PrometheusOperations::default();
    for trx in block.all_transaction_traces() {
        for trace in &trx.action_traces {
            // unwrap action_trace
            let action_trace = trace.action.as_ref().unwrap();
            let _name = action_trace.name.clone();
            let account = action_trace.account.clone();

            // skip additional receivers (i.e. not the contract account)
            if trace.receiver != account { continue; }
            if account != "iot.taiss" { continue; }

            // Temperature
            let temperature_data = abi::Temperature::try_from(action_trace.json_data.as_str());
            match temperature_data {
                Ok(temperature_data) => {
                    let device_id = temperature_data.device_id;
                    let temperature: f64 = temperature_data.temperature.parse().unwrap();
                    let device_label = HashMap::from([("device_id".to_string(), device_id.to_string())]);
                    prom_out.push(Gauge::from("temperature").with(device_label).set(temperature));
                },
                Err(_) => {}
            }

            // Location
            let location_data = abi::Location::try_from(action_trace.json_data.as_str());
            match location_data {
                Ok(location_data) => {
                    let device_id = location_data.device_id;
                    let x: f64 = location_data.x.parse().unwrap();
                    let y: f64 = location_data.x.parse().unwrap();
                    let z: f64 = location_data.x.parse().unwrap();
                    let device_label = HashMap::from([("device_id".to_string(), device_id.to_string())]);
                    prom_out.push(Gauge::from("x").with(device_label.clone()).set(x));
                    prom_out.push(Gauge::from("y").with(device_label.clone()).set(y));
                    prom_out.push(Gauge::from("z").with(device_label.clone()).set(z));
                },
                Err(_) => {}
            }
        }
    }
    Ok(prom_out)
}
