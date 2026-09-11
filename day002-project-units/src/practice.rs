use crate::ProjectUnit;

/// 아직 분류하지 않았음을 나타내는 안전한 시작값이다.
pub const PRACTICE_SENTINEL: Option<ProjectUnit> = None;

/// 파일 경로가 속하는 프로젝트 단위를 분류한다.
#[must_use]
pub const fn classify_project_file(_path: &str) -> Option<ProjectUnit> {
    PRACTICE_SENTINEL
}

#[cfg(test)]
mod tests {
    use super::classify_project_file;
    use crate::ProjectUnit;

    #[test]
    #[ignore = "learning exercise"]
    fn classifies_manifest_as_package_when_given_cargo_toml() {
        // Given
        let path = "Cargo.toml";

        // When
        let unit = classify_project_file(path);

        // Then
        assert_eq!(unit, Some(ProjectUnit::Package));
    }
}
