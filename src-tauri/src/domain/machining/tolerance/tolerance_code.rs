pub fn parse_tolerance_code(code: &str) -> Result<(String, i32), String> {
    let trimmed = code.trim();
    if trimmed.is_empty() {
        return Err("Invalid tolerance code ''. Use letters followed by digits, for example H7, JS7, g6, or js6".to_string());
    }

    let split_idx = trimmed
        .find(|ch: char| ch.is_ascii_digit())
        .ok_or_else(|| invalid_code_message(code))?;
    let (zone, grade_text) = trimmed.split_at(split_idx);

    if zone.is_empty()
        || grade_text.is_empty()
        || !zone.chars().all(|ch| ch.is_ascii_alphabetic())
        || !grade_text.chars().all(|ch| ch.is_ascii_digit())
    {
        return Err(invalid_code_message(code));
    }

    let grade = grade_text
        .parse::<i32>()
        .map_err(|_| invalid_code_message(code))?;

    Ok((zone.to_string(), grade))
}

fn invalid_code_message(code: &str) -> String {
    format!(
        "Invalid tolerance code '{}'. Use letters followed by digits, for example H7, JS7, g6, or js6",
        code
    )
}
