use super::{ThreadError, ThreadType};

#[derive(Debug, Clone, PartialEq)]
pub struct ThreadPitchOption {
    pub value: String,
    pub label: String,
    pub pitch_mm: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThreadSizeOption {
    pub value: String,
    pub label: String,
    pub major_diameter_mm: f64,
    pub pitches: Vec<ThreadPitchOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThreadTypeOption {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThreadOptions {
    pub types: Vec<ThreadTypeOption>,
    pub metric: Vec<ThreadSizeOption>,
    pub unc: Vec<ThreadSizeOption>,
    pub unf: Vec<ThreadSizeOption>,
    pub bsp: Vec<ThreadSizeOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThreadSpec {
    pub thread_type: ThreadType,
    pub size: String,
    pub pitch: String,
    pub major_diameter_mm: f64,
    pub pitch_mm: f64,
    pub tap_drill_mm: f64,
    pub radial_thread_depth_mm: f64,
}

pub fn list_thread_options() -> ThreadOptions {
    ThreadOptions {
        types: [
            ThreadType::Metric,
            ThreadType::Unc,
            ThreadType::Unf,
            ThreadType::Bsp,
        ]
        .into_iter()
        .map(|thread_type| ThreadTypeOption {
            value: thread_type.as_str().to_string(),
            label: thread_type.label().to_string(),
        })
        .collect(),
        metric: metric_options(),
        unc: unc_options(),
        unf: unf_options(),
        bsp: bsp_options(),
    }
}

pub fn resolve_thread_spec(
    thread_type: ThreadType,
    size: &str,
    pitch: &str,
) -> Result<ThreadSpec, ThreadError> {
    let size_option = options_for_type(thread_type)
        .into_iter()
        .find(|option| option.value == size)
        .ok_or_else(|| ThreadError::UnsupportedThreadSize {
            thread_type: thread_type.to_string(),
            size: size.to_string(),
        })?;

    let pitch_option = size_option
        .pitches
        .iter()
        .find(|option| option.value == pitch)
        .ok_or_else(|| ThreadError::UnsupportedThreadPitch {
            thread_type: thread_type.to_string(),
            size: size.to_string(),
            pitch: pitch.to_string(),
        })?;

    Ok(ThreadSpec {
        thread_type,
        size: size_option.value,
        pitch: pitch_option.value.clone(),
        major_diameter_mm: size_option.major_diameter_mm,
        pitch_mm: pitch_option.pitch_mm,
        tap_drill_mm: size_option.major_diameter_mm - pitch_option.pitch_mm,
        radial_thread_depth_mm: thread_type.depth_factor() * pitch_option.pitch_mm,
    })
}

fn options_for_type(thread_type: ThreadType) -> Vec<ThreadSizeOption> {
    match thread_type {
        ThreadType::Metric => metric_options(),
        ThreadType::Unc => unc_options(),
        ThreadType::Unf => unf_options(),
        ThreadType::Bsp => bsp_options(),
    }
}

fn metric_options() -> Vec<ThreadSizeOption> {
    vec![
        metric("M3", 3.0, &[0.5]),
        metric("M4", 4.0, &[0.7]),
        metric("M5", 5.0, &[0.8]),
        metric("M6", 6.0, &[1.0]),
        metric("M8", 8.0, &[1.0, 1.25]),
        metric("M10", 10.0, &[1.0, 1.25, 1.5]),
        metric("M12", 12.0, &[1.25, 1.5, 1.75]),
        metric("M16", 16.0, &[1.5, 2.0]),
        metric("M20", 20.0, &[1.5, 2.5]),
    ]
}

fn unc_options() -> Vec<ThreadSizeOption> {
    vec![
        unified("#6", 0.138, &[32]),
        unified("#8", 0.164, &[32]),
        unified("#10", 0.190, &[24]),
        unified("1/4", 0.250, &[20]),
        unified("5/16", 0.3125, &[18]),
        unified("3/8", 0.375, &[16]),
        unified("1/2", 0.500, &[13]),
        unified("5/8", 0.625, &[11]),
    ]
}

fn unf_options() -> Vec<ThreadSizeOption> {
    vec![
        unified("#6", 0.138, &[40]),
        unified("#8", 0.164, &[36]),
        unified("#10", 0.190, &[32]),
        unified("1/4", 0.250, &[28]),
        unified("5/16", 0.3125, &[24]),
        unified("3/8", 0.375, &[24]),
        unified("1/2", 0.500, &[20]),
        unified("5/8", 0.625, &[18]),
    ]
}

fn bsp_options() -> Vec<ThreadSizeOption> {
    vec![
        bsp("G1/8", 9.728, 28),
        bsp("G1/4", 13.157, 19),
        bsp("G3/8", 16.662, 19),
        bsp("G1/2", 20.955, 14),
        bsp("G3/4", 26.441, 14),
        bsp("G1", 33.249, 11),
    ]
}

fn metric(label: &str, major_diameter_mm: f64, pitches_mm: &[f64]) -> ThreadSizeOption {
    ThreadSizeOption {
        value: label.to_string(),
        label: label.to_string(),
        major_diameter_mm,
        pitches: pitches_mm
            .iter()
            .map(|pitch| ThreadPitchOption {
                value: format_metric_pitch(*pitch),
                label: format!("{} mm", format_metric_pitch(*pitch)),
                pitch_mm: *pitch,
            })
            .collect(),
    }
}

fn unified(label: &str, major_diameter_inches: f64, tpis: &[i32]) -> ThreadSizeOption {
    ThreadSizeOption {
        value: label.to_string(),
        label: label.to_string(),
        major_diameter_mm: major_diameter_inches * 25.4,
        pitches: tpis
            .iter()
            .map(|tpi| ThreadPitchOption {
                value: tpi.to_string(),
                label: format!("{tpi} TPI"),
                pitch_mm: 25.4 / f64::from(*tpi),
            })
            .collect(),
    }
}

fn bsp(label: &str, major_diameter_mm: f64, tpi: i32) -> ThreadSizeOption {
    ThreadSizeOption {
        value: label.to_string(),
        label: label.to_string(),
        major_diameter_mm,
        pitches: vec![ThreadPitchOption {
            value: tpi.to_string(),
            label: format!("{tpi} TPI"),
            pitch_mm: 25.4 / f64::from(tpi),
        }],
    }
}

fn format_metric_pitch(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}
