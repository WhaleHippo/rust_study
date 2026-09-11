/// 프로젝트를 이루는 단위를 보여 주는 실제 모듈이다.
pub mod project_units {
    /// Cargo 프로젝트에서 구분할 세 가지 단위다.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ProjectUnit {
        /// `Cargo.toml`이 설명하는 빌드 및 배포 단위다.
        Package,
        /// 컴파일러가 한 번에 컴파일하는 단위다.
        Crate,
        /// 크레이트 안의 이름 공간과 가시성 단위다.
        Module,
    }

    /// 프로젝트 단위의 영문 이름을 반환한다.
    #[must_use]
    pub const fn unit_label(unit: ProjectUnit) -> &'static str {
        match unit {
            ProjectUnit::Package => "package",
            ProjectUnit::Crate => "crate",
            ProjectUnit::Module => "module",
        }
    }
}

pub use project_units::{ProjectUnit, unit_label};

#[cfg(feature = "learner-practice")]
pub mod practice;

#[cfg(test)]
mod tests {
    #[test]
    fn unit_label_identifies_a_crate_when_given_crate_kind() {
        // Given
        let unit = super::ProjectUnit::Crate;

        // When
        let label = super::unit_label(unit);

        // Then
        assert_eq!(label, "crate");
    }
}
