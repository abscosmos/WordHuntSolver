pub fn collect_valid_words(path: &std::path::Path) -> Result<Vec<String>, std::io::Error> {
    let words = std::fs::read_to_string(path)?
        .lines()
        .filter(|&s| s.len() <= 14)
        .filter(|&word| (3..=14).contains(&word.len()) && word.chars().all(|c| c.is_ascii_alphabetic()))
        .map(str::to_ascii_lowercase)
        .map(String::from)
        .collect();

    Ok(words)
}

pub fn compile_valid_words(input: &std::path::Path, output: &std::path::Path) -> std::io::Result<()> {
    std::fs::write(output, collect_valid_words(input)?.join("\n"))
}