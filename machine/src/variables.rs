pub fn get_special_variables() -> Vec<String> {
    vec![
        "$Position".to_string(), // Read-only position
        "$Rotation".to_string(), // Read-only Rotation
        "$RayDist".to_string(),
        "$RayType".to_string(),
        "$Velocity".to_string(),
        "$Moment".to_string(),
    ]
}
