use bevy_egui::egui::Color32;

// Define colors for different syntax elements
pub struct AfgSyntaxColors {
    pub keyword: Color32,
    pub system_var: Color32,
    pub number: Color32,
    pub operator: Color32,
    pub comment: Color32,
    pub string: Color32,
    pub function_name: Color32,
    pub variable: Color32,
    pub default: Color32,
}

impl Default for AfgSyntaxColors {
    fn default() -> Self {
        Self {
            keyword: Color32::from_rgb(86, 156, 214), // Blue - keywords
            system_var: Color32::from_rgb(156, 220, 254), // Light blue - $variables
            number: Color32::from_rgb(181, 206, 168), // Green - numbers
            operator: Color32::from_rgb(212, 212, 212), // Light gray - operators
            comment: Color32::from_rgb(106, 153, 85), // Dark green - comments
            string: Color32::from_rgb(206, 145, 120), // Orange - strings (if needed)
            function_name: Color32::from_rgb(220, 220, 170), // Yellow - function names
            variable: Color32::from_rgb(156, 220, 254), // Light blue - variables
            default: Color32::from_rgb(212, 212, 212), // Default text color
        }
    }
}
