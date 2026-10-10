//! Pre-release API versions that a generally available version of the same
//! API supersedes. They are left out of the generated SDK to keep the
//! published crate under the crates.io size limit.

use super::{Package, Proto};
use std::{collections::HashSet, fmt};

/// Pre-release API versions generated even though a GA version of the same
/// API exists. An entry keeps every package under that version, sub-packages
/// such as `google.cloud.aiplatform.v1beta1.schema.predict.instance` included.
const KEPT_PRERELEASE_VERSIONS: &[&str] = &["google.cloud.aiplatform.v1beta1"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    GenerallyAvailable,
    PreRelease,
}

/// Where a package sits among its API's versions, read from the first
/// segment of the package shaped like an API version.
struct ApiVersion<'a> {
    /// The segments before the version, e.g. `google.cloud.container`.
    api: &'a str,
    /// The API and its version segment, e.g. `google.cloud.container.v1beta1`.
    version: &'a str,
    stage: Stage,
}

impl Package {
    fn api_version(&self) -> Option<ApiVersion<'_>> {
        let mut offset = 0;
        for segment in self.raw.split('.') {
            let end = offset + segment.len();
            if let Some(stage) = version_stage(segment) {
                return Some(ApiVersion {
                    api: self.raw[..offset].trim_end_matches('.'),
                    version: &self.raw[..end],
                    stage,
                });
            }
            offset = end + 1;
        }
        None
    }
}

/// Classifies `v1` as GA and `v1alpha`, `v1beta1`, `v1p1beta1` as
/// pre-release. Segments such as `v1small` or `v1test2` are not versions.
fn version_stage(segment: &str) -> Option<Stage> {
    let rest = strip_number(segment.strip_prefix('v')?)?;
    if rest.is_empty() {
        return Some(Stage::GenerallyAvailable);
    }
    let rest = match rest.strip_prefix('p') {
        Some(point) => strip_number(point)?,
        None => rest,
    };
    let number = rest
        .strip_prefix("alpha")
        .or_else(|| rest.strip_prefix("beta"))?;
    number
        .bytes()
        .all(|byte| byte.is_ascii_digit())
        .then_some(Stage::PreRelease)
}

/// Strips a leading decimal number, which must be present.
fn strip_number(text: &str) -> Option<&str> {
    let digits = text.bytes().take_while(u8::is_ascii_digit).count();
    (digits > 0).then(|| &text[digits..])
}

/// The pre-release packages of a set that a GA version in the same set
/// supersedes: a package is superseded when its version is alpha or beta, a
/// version of the same API without a suffix is in the set, and its version is
/// not in [`KEPT_PRERELEASE_VERSIONS`].
pub struct SupersededPrereleases {
    /// APIs with a GA version in the set, e.g. `google.cloud.container` when
    /// `google.cloud.container.v1` is present.
    stable_apis: HashSet<String>,
}

impl<'a> FromIterator<&'a Package> for SupersededPrereleases {
    fn from_iter<T: IntoIterator<Item = &'a Package>>(packages: T) -> Self {
        let stable_apis = packages
            .into_iter()
            .filter_map(Package::api_version)
            .filter(|version| version.stage == Stage::GenerallyAvailable)
            .map(|version| version.api.to_owned())
            .collect();
        Self { stable_apis }
    }
}

impl SupersededPrereleases {
    pub fn contains(&self, package: &Package) -> bool {
        package.api_version().is_some_and(|version| {
            version.stage == Stage::PreRelease
                && self.stable_apis.contains(version.api)
                && !KEPT_PRERELEASE_VERSIONS.contains(&version.version)
        })
    }

    /// Removes the protos of superseded packages.
    ///
    /// Fails when a remaining proto imports a superseded package, directly or
    /// through other imports: the importer does not compile without it, so its
    /// version has to go into [`KEPT_PRERELEASE_VERSIONS`].
    pub fn remove_from(&self, mut protos: Vec<Proto>) -> Result<Vec<Proto>, SupersededImports> {
        protos.retain(|proto| !self.contains(&proto.package));

        let mut visited = HashSet::new();
        let mut pending = protos
            .iter()
            .flat_map(|proto| proto.imports.iter().map(move |import| (proto, import)))
            .collect::<Vec<_>>();
        let mut refused = Vec::new();
        while let Some((importer, import)) = pending.pop() {
            if !visited.insert(import.path.as_path()) {
                continue;
            }
            if self.contains(&import.package) {
                refused.push(SupersededImport {
                    importer: importer.package.clone(),
                    imported: import.package.clone(),
                });
            } else {
                pending.extend(import.imports.iter().map(|nested| (import, nested)));
            }
        }

        if refused.is_empty() {
            Ok(protos)
        } else {
            Err(SupersededImports(refused))
        }
    }
}

/// Superseded packages that a generated proto imports.
#[derive(Debug)]
pub struct SupersededImports(Vec<SupersededImport>);

#[derive(Debug, PartialEq, Eq)]
struct SupersededImport {
    importer: Package,
    imported: Package,
}

impl fmt::Display for SupersededImports {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "generated protos import superseded pre-release packages; \
             add their versions to KEPT_PRERELEASE_VERSIONS:"
        )?;
        for import in &self.0 {
            writeln!(f, "  {:?} imports {:?}", import.importer, import.imported)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn superseded_among(names: &[&str]) -> SupersededPrereleases {
        let packages = names
            .iter()
            .map(|name| Package::from(*name))
            .collect::<Vec<_>>();
        packages.iter().collect()
    }

    fn proto(path: &str, package: &str, imports: Vec<Proto>) -> Proto {
        Proto {
            path: PathBuf::from(path),
            package: package.into(),
            imports,
        }
    }

    #[test]
    fn prerelease_with_ga_version_is_superseded() {
        let superseded = superseded_among(&["google.container.v1", "google.container.v1beta1"]);
        assert!(superseded.contains(&"google.container.v1beta1".into()));
        assert!(!superseded.contains(&"google.container.v1".into()));
    }

    #[test]
    fn unnumbered_prerelease_with_ga_version_is_superseded() {
        let superseded = superseded_among(&["google.cloud.ces.v1", "google.cloud.ces.v1beta"]);
        assert!(superseded.contains(&"google.cloud.ces.v1beta".into()));
    }

    #[test]
    fn point_release_beta_with_ga_version_is_superseded() {
        let superseded =
            superseded_among(&["google.cloud.speech.v1", "google.cloud.speech.v1p1beta1"]);
        assert!(superseded.contains(&"google.cloud.speech.v1p1beta1".into()));
    }

    #[test]
    fn subpackage_of_superseded_version_is_superseded() {
        let superseded = superseded_among(&["grafeas.v1", "grafeas.v1beta1.build"]);
        assert!(superseded.contains(&"grafeas.v1beta1.build".into()));
    }

    #[test]
    fn prerelease_without_ga_version_is_kept() {
        let superseded = superseded_among(&[
            "google.analytics.admin.v1alpha",
            "google.analytics.admin.v1beta",
        ]);
        assert!(!superseded.contains(&"google.analytics.admin.v1alpha".into()));
        assert!(!superseded.contains(&"google.analytics.admin.v1beta".into()));
    }

    #[test]
    fn kept_prerelease_version_is_not_superseded() {
        let superseded = superseded_among(&[
            "google.cloud.aiplatform.v1",
            "google.cloud.aiplatform.v1beta1",
            "google.cloud.aiplatform.v1.schema.predict.instance",
            "google.cloud.aiplatform.v1beta1.schema.predict.instance",
        ]);
        assert!(!superseded.contains(&"google.cloud.aiplatform.v1beta1".into()));
        assert!(
            !superseded.contains(&"google.cloud.aiplatform.v1beta1.schema.predict.instance".into())
        );
    }

    #[test]
    fn superseded_protos_are_removed() {
        let protos = vec![
            proto(
                "google/container/v1/cluster_service.proto",
                "google.container.v1",
                Vec::new(),
            ),
            proto(
                "google/container/v1beta1/cluster_service.proto",
                "google.container.v1beta1",
                Vec::new(),
            ),
        ];
        let superseded = protos
            .iter()
            .map(|proto| &proto.package)
            .collect::<SupersededPrereleases>();

        let kept = superseded.remove_from(protos).unwrap();

        assert_eq!(
            kept.into_iter()
                .map(|proto| proto.package)
                .collect::<Vec<_>>(),
            vec![Package::from("google.container.v1")]
        );
    }

    #[test]
    fn import_of_superseded_package_is_refused() {
        let beta_cluster = proto(
            "google/container/v1beta1/cluster_service.proto",
            "google.container.v1beta1",
            Vec::new(),
        );
        let protos = vec![
            proto(
                "google/container/v1/cluster_service.proto",
                "google.container.v1",
                Vec::new(),
            ),
            proto(
                "google/cloud/fleet/v1/fleet.proto",
                "google.cloud.fleet.v1",
                vec![beta_cluster.clone()],
            ),
            beta_cluster,
        ];
        let superseded = protos
            .iter()
            .map(|proto| &proto.package)
            .collect::<SupersededPrereleases>();

        let refused = superseded.remove_from(protos).unwrap_err();

        assert_eq!(
            refused.0,
            vec![SupersededImport {
                importer: "google.cloud.fleet.v1".into(),
                imported: "google.container.v1beta1".into(),
            }]
        );
    }
}
