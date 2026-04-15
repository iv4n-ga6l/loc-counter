/// This module provides functionality to calculate COCOMO estimates for a given codebase.
/// The Basic COCOMO model is used with the following formula:
///
/// Effort (person-months) = a * (KLOC)^b
/// Time (months) = c * (Effort)^d
/// Developers = Effort / Time
///
/// For now, this implementation assumes an Organic project type with the following coefficients:
/// - a = 2.4
/// - b = 1.05
/// - c = 2.5
/// - d = 0.38

/// Represents the result of a COCOMO calculation.
pub struct CocomoResult {
    pub effort: f64,      // Effort in person-months
    pub time: f64,        // Time in months
    pub developers: f64,  // Number of developers
}

/// Calculates the COCOMO estimates based on the total lines of code (LOC).
///
/// # Arguments
/// * `loc` - The total lines of code in the project.
///
/// # Returns
/// A `CocomoResult` containing the calculated effort, time, and number of developers.
///
/// # Assumptions
/// This function assumes an Organic project type with the following coefficients:
/// - a = 2.4
/// - b = 1.05
/// - c = 2.5
/// - d = 0.38
pub fn calculate_cocomo(loc: usize) -> CocomoResult {
    // Convert LOC to KLOC (thousands of lines of code)
    let kloc = loc as f64 / 1000.0;

    // COCOMO coefficients for Organic projects
    let a = 2.4;
    let b = 1.05;
    let c = 2.5;
    let d = 0.38;

    // Calculate effort, time, and developers
    let effort = a * kloc.powf(b);
    let time = c * effort.powf(d);
    let developers = effort / time;

    CocomoResult { effort, time, developers }
}
