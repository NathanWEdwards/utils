//! MIT license.

/// If the system time is less than the timestamp provided then sleep for a duration of time provided in milliseconds.
/// 
/// # Example
///
/// ```
/// let five_seconds = std::time::Duration::from_millis(5_000);
/// let moment = std::time::Instant::now();
/// utils::time::sleep_until_time_elapsed(std::time::SystemTime::now(), 5_000 as u64);
/// assert!(moment.elapsed() >= five_seconds)
/// ```
pub fn sleep_until_time_elapsed(timestamp: std::time::SystemTime, milliseconds: u64) {
    let current_time = std::time::SystemTime::now();
    let time_to_sleep_until = timestamp + std::time::Duration::from_millis(milliseconds);


    if time_to_sleep_until.gt(&current_time) {
        match time_to_sleep_until.duration_since(current_time) {
            Ok(duration_to_wait) => {
                std::thread::sleep(duration_to_wait);
            },
            Err(error)  => {
                eprintln!("{{\"error\": \"{}\"}}", error);
            }
        }
    }
}