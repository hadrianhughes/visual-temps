use lm_sensors;

fn main() -> Result<(), lm_sensors::errors::Error> {
    let sensors = lm_sensors::Initializer::default().initialize()?;

    for chip in sensors.chip_iter(None) {
        if let Some(path) = chip.path() {
            println!("chip: {} at {} ({})", chip, chip.bus(), path.display());
        } else {
            println!("chip: {} at {}", chip, chip.bus());
        }

        for feature in chip.feature_iter() {
            let name = feature.name().transpose()?.unwrap_or("N/A");
            println!("    {}: {}", name, feature);

            for sub_feature in feature.sub_feature_iter() {
                if let Ok(value) = sub_feature.value() {
                    println!("        {}: {}", sub_feature, value);
                } else {
                    println!("        {}: N/A", sub_feature);
                }
            }
        }
    }

    Ok(())
}
