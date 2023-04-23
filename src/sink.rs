use std::collections::HashMap;

use substreams::errors::Error;
use substreams_antelope::Block;
use substreams_sink_prometheus::{PrometheusOperations, Counter, Gauge};
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
            if account != "d.iot.taiss" { continue; }

            // Temperature
            let temperature_data = abi::Temperature::try_from(action_trace.json_data.as_str());
            match temperature_data {
                Ok(temperature_data) => {
                    // data
                    let transmitter = temperature_data.transmitter;
                    let receiver = temperature_data.receiver;
                    let temperature: f64 = temperature_data.temperature.parse().unwrap();

                    // labels
                    let device_label = HashMap::from([
                        ("transmitter".to_string(), transmitter.to_string()),
                        ("receiver".to_string(), receiver.to_string())
                    ]);
                    let receiver_label = HashMap::from([("signature".to_string(), receiver.to_string())]);
                    let transmitter_label = HashMap::from([("signature".to_string(), transmitter.to_string())]);

                    // gauges
                    prom_out.push(Gauge::from("temperature").with(device_label).set(temperature));

                    // counters
                    prom_out.push(Counter::from("transmitter").with(transmitter_label).inc());
                    prom_out.push(Counter::from("receiver").with(receiver_label).inc());
                },
                Err(_) => {}
            }

            // Location
            let location_data = abi::Location::try_from(action_trace.json_data.as_str());
            match location_data {
                Ok(location_data) => {
                    // data
                    let transmitter = location_data.transmitter;
                    let receiver = location_data.receiver;
                    let location = location_data.location;
                    let x: f64 = location[0].parse().unwrap();
                    let y: f64 = location[1].parse().unwrap();
                    let z: f64 = location[2].parse().unwrap();

                    // labels
                    let device_label = HashMap::from([
                        ("transmitter".to_string(), transmitter.to_string()),
                        ("receiver".to_string(), receiver.to_string()),
                        ("x".to_string(), x.to_string()),
                        ("y".to_string(), y.to_string()),
                        ("z".to_string(), z.to_string())
                    ]);
                    let receiver_label = HashMap::from([("signature".to_string(), receiver.to_string())]);
                    let transmitter_label = HashMap::from([("signature".to_string(), transmitter.to_string())]);

                    // gauges
                    prom_out.push(Gauge::from("location").with(device_label.clone()).set(1.0));

                    // counters
                    prom_out.push(Counter::from("transmitter").with(transmitter_label).inc());
                    prom_out.push(Counter::from("receiver").with(receiver_label).inc());
                },
                Err(_) => {}
            }
        }
    }
    Ok(prom_out)
}
