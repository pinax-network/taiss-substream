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
        }
    }
    Ok(prom_out)
}
