use std::{
    collections::{HashMap, HashSet},
    fmt, fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Package {
    raw: String,
    escaped: String,
    escaped_vec: Vec<String>,
}

impl fmt::Debug for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.escaped)
    }
}

impl From<&str> for Package {
    fn from(v: &str) -> Self {
        let vec = v
            .split('.')
            .map(escape)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        Self {
            raw: v.to_owned(),
            escaped: vec.join("."),
            escaped_vec: vec,
        }
    }
}

impl Package {
    fn from_escaped_vec(vec: Vec<String>) -> Self {
        let raw = vec
            .iter()
            .map(|v| {
                if v.starts_with("r#") {
                    v.chars().skip(2).collect::<String>()
                } else {
                    v.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join(".");
        Self {
            raw,
            escaped: vec.join("."),
            escaped_vec: vec,
        }
    }

    // https://doc.rust-lang.org/cargo/reference/features.html#features
    // crates.io requires feature names to only contain ASCII letters, digits, _, or -.
    fn feature_name(&self) -> String {
        self.raw.split('.').collect::<Vec<_>>().join("-")
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Proto {
    path: PathBuf,
    package: Package,
    imports: Vec<Proto>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    package: Package,
    include: bool,
    imported_by: HashSet<Package>,
    children: HashMap<String, Module>,
}

impl Module {
    fn empty(package: Package) -> Self {
        Self {
            package,
            include: false,
            imported_by: HashSet::new(),
            children: HashMap::new(),
        }
    }
}

impl Module {
    fn gen_code(&self) -> String {
        let include = if self.include {
            let mut attr = self
                .imported_by
                .iter()
                .map(|p| p.feature_name())
                .collect::<HashSet<_>>();
            attr.insert(self.package.feature_name());
            let mut attr = attr.into_iter().collect::<Vec<_>>();
            attr.sort();
            let attr = attr
                .into_iter()
                .map(|f| format!(r#"feature = "{}","#, f))
                .collect::<String>();
            let attr = format!("#[cfg(any({}))]", attr);
            format!(
                "{attr}\ninclude_proto!(\"{package}\");\n",
                attr = attr,
                package = self.package.escaped,
            )
        } else {
            String::new()
        };

        let mut vec = self.children.iter().collect::<Vec<_>>();
        vec.sort_by_key(|v| v.0);
        let children = vec
            .into_iter()
            .map(|(k, v)| {
                format!(
                    "pub mod {name} {{ {children} }}\n",
                    name = k,
                    children = v.gen_code(),
                )
            })
            .collect::<String>();

        format!(
            "{include} {children}",
            include = include,
            children = children
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootModule(HashMap<String, Module>);

impl RootModule {
    pub fn gen_code(&self) -> String {
        let mut vec = self.0.iter().collect::<Vec<_>>();
        vec.sort_by_key(|v| v.0);
        vec.into_iter()
            .map(|(k, v)| {
                format!(
                    "pub mod {name} {{ {children} }}\n",
                    name = k,
                    children = v.gen_code(),
                )
            })
            .collect()
    }
}

// https://doc.rust-lang.org/reference/keywords.html
fn escape(s: &str) -> &str {
    match s {
        // Lexer
        "as" => "r#as",
        "break" => "r#break",
        "const" => "r#const",
        "continue" => "r#continue",
        "crate" => "r#crate",
        "else" => "r#else",
        "enum" => "r#enum",
        "extern" => "r#extern",
        "false" => "r#false",
        "fn" => "r#fn",
        "for" => "r#for",
        "if" => "r#if",
        "impl" => "r#impl",
        "in" => "r#in",
        "let" => "r#let",
        "loop" => "r#loop",
        "match" => "r#match",
        "mod" => "r#mod",
        "move" => "r#move",
        "mut" => "r#mut",
        "pub" => "r#pub",
        "ref" => "r#ref",
        "return" => "r#return",
        "self" => "r#self",
        "Self" => "r#Self",
        "static" => "r#static",
        "struct" => "r#struct",
        "super" => "r#super",
        "trait" => "r#trait",
        "true" => "r#true",
        "type" => "r#type",
        "unsafe" => "r#unsafe",
        "use" => "r#use",
        "where" => "r#where",
        "while" => "r#while",
        // Lexer 2018+
        "async" => "r#async",
        "await" => "r#await",
        "dyn" => "r#dyn",
        // Reserved keywords
        "abstract" => "r#abstract",
        "become" => "r#become",
        "box" => "r#box",
        "do" => "r#do",
        "final" => "r#final",
        "macro" => "r#macro",
        "override" => "r#override",
        "priv" => "r#priv",
        "typeof" => "r#typeof",
        "unsized" => "r#unsized",
        "virtual" => "r#virtual",
        "yield" => "r#yield",
        _ => s,
    }
}

// https://developers.google.com/protocol-buffers/docs/reference/proto3-spec#package
// package = "package" fullIdent ";"
fn parse_package(line: &str) -> Option<Package> {
    const PACKAGE: &str = "package";

    if line.starts_with(PACKAGE) {
        let package = line
            .chars()
            .skip(PACKAGE.len())
            .take_while(|&c| c != ';')
            .collect::<String>()
            .trim()
            .to_owned();
        Some(package.as_str().into())
    } else {
        None
    }
}

// https://developers.google.com/protocol-buffers/docs/reference/proto3-spec#import_statement
// import = "import" [ "weak" | "public" ] strLit ";"
fn parse_import(line: &str) -> Option<String> {
    const IMPORT: &str = "import";

    if line.starts_with(IMPORT) {
        let import = line
            .chars()
            .skip(IMPORT.len())
            .skip_while(|&c| c != '\'' && c != '"')
            .skip(1)
            .take_while(|&c| c != '\'' && c != '"') // TODO: Should I check to see if it's the same quart?
            .collect::<String>() // TODO: Should I check to see if the following letter is semicolon?
            .trim()
            .to_owned();
        Some(import)
    } else {
        None
    }
}

pub fn find_proto(root: PathBuf) -> Vec<Proto> {
    let mut cache = HashMap::new();
    find_proto_rec(root.clone(), root, &mut cache)
}

fn find_proto_rec(
    root: PathBuf,
    dir: impl AsRef<Path>,
    cache: &mut HashMap<PathBuf, Proto>,
) -> Vec<Proto> {
    let mut ret = Vec::new();

    let cur = fs::read_dir(dir.as_ref())
        .unwrap()
        .map(Result::unwrap)
        .map(|dir| dir.path())
        .map(|path| (path.metadata().unwrap(), path))
        .collect::<Vec<_>>();

    let iter = cur
        .clone()
        .into_iter()
        .filter(|(meta, _)| meta.is_file())
        .filter(|(_, path)| path.extension().filter(|ex| ex == &"proto").is_some());
    for (_, path) in iter {
        let proto = if let Some(proto) = cache.get(&path) {
            proto.clone()
        } else {
            let prot = proto_rec(root.clone(), path.clone(), cache);
            cache.insert(path, prot.clone());
            prot
        };
        ret.push(proto);
    }

    let iter = cur.into_iter().filter(|(meta, _)| meta.is_dir());
    for (_, path) in iter {
        let mut protos = find_proto_rec(root.clone(), path, cache);
        ret.append(&mut protos);
    }

    ret
}

fn proto_rec(root: PathBuf, path: PathBuf, map: &mut HashMap<PathBuf, Proto>) -> Proto {
    let mut package = None;
    let mut imports = Vec::new();

    for line in fs::read_to_string(path.as_path()).unwrap().lines() {
        if let Some(pkg) = parse_package(line) {
            package = Some(pkg);
        } else if let Some(import) = parse_import(line) {
            if !import.starts_with("google/protobuf") {
                let mut path = root.clone();
                path.push(import);
                if let Some(proto) = map.get(path.as_path()) {
                    imports.push(proto.clone());
                } else {
                    let prot = proto_rec(root.clone(), path.clone(), map);
                    imports.push(prot.clone());
                    map.insert(path, prot);
                }
            }
        }
    }

    Proto {
        path,
        package: package.unwrap(),
        imports,
    }
}

fn add_deps_rec(src: &Proto, proto: &Proto, map: &mut HashMap<Package, HashSet<Package>>) {
    for import in proto.imports.iter() {
        let e = map
            .entry(import.package.clone())
            .or_insert_with(HashSet::new);
        e.insert(src.package.clone());

        for import in import.imports.iter() {
            add_deps_rec(src, &import, map);
        }
    }
}

fn deps_resolver(protos: &[Proto]) -> HashMap<Package, HashSet<Package>> {
    let mut map = HashMap::new();
    for p in protos.iter() {
        add_deps_rec(p, p, &mut map);
    }
    map
}

#[allow(dead_code)]
pub fn feature_gates(protos: &[Proto]) -> String {
    let pkgs = protos
        .iter()
        .map(|p| p.package.feature_name())
        .collect::<HashSet<_>>();
    let mut pkgs = pkgs.into_iter().collect::<Vec<_>>();
    pkgs.sort();
    pkgs.into_iter()
        .map(|f| format!("{} = []", f))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn proto_path(protos: &[Proto]) -> Vec<PathBuf> {
    let mut ret = protos.iter().map(|p| p.path.clone()).collect::<Vec<_>>();
    ret.sort();
    ret
}

pub fn from_protos(protos: Vec<Proto>) -> RootModule {
    let mut map = HashMap::new();
    let resolver = deps_resolver(&protos);

    for proto in protos {
        let mut iter = proto.package.escaped_vec.clone().into_iter();

        let mut package = Vec::new();
        let pkg = iter.next().unwrap();
        package.push(pkg.clone());

        let mut e = map
            .entry(pkg)
            .or_insert_with(|| Module::empty(Package::from_escaped_vec(package.clone())));

        while let Some(pkg) = iter.next() {
            package.push(pkg.clone());
            e = e
                .children
                .entry(pkg)
                .or_insert_with(|| Module::empty(Package::from_escaped_vec(package.clone())));
        }

        assert_eq!(proto.package, e.package);
        e.include = true;
        if let Some(pkgs) = resolver.get(&e.package) {
            for pkg in pkgs {
                e.imported_by.insert(pkg.clone());
            }
        }
    }

    RootModule(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacakge_from_escaped_vec() {
        let vec = vec!["mechiru".into(), "r#type".into(), "r#as".into()];
        assert_eq!(
            Package::from_escaped_vec(vec.clone()),
            Package {
                raw: "mechiru.type.as".into(),
                escaped: "mechiru.r#type.r#as".into(),
                escaped_vec: vec,
            }
        );
    }

    #[test]
    fn test_package_from() {
        assert_eq!(
            Package::from("mechiru.type.as"),
            Package::from_escaped_vec(vec!["mechiru".into(), "r#type".into(), "r#as".into()])
        );
    }

    #[test]
    fn test_feature_name() {
        assert_eq!(
            Package::from("mechiru.type.as").feature_name(),
            "mechiru-type-as".to_owned()
        );
    }

    #[test]
    fn test_parse_package() {
        assert_eq!(parse_package("package mechiru;"), Some("mechiru".into()));
        assert_eq!(
            parse_package("package mechiru.storage;"),
            Some("mechiru.storage".into())
        );
    }

    #[test]
    fn test_parse_import() {
        assert_eq!(
            parse_import(r#"import "mechiru/common.proto";"#),
            Some("mechiru/common.proto".into())
        );

        assert_eq!(
            parse_import(r#"import weak "mechiru/common.proto";"#),
            Some("mechiru/common.proto".into())
        );

        assert_eq!(
            parse_import(r#"import public "mechiru/common.proto";"#),
            Some("mechiru/common.proto".into())
        );
    }

    fn protos() -> Vec<Proto> {
        vec![
            Proto {
                path: PathBuf::from("/a/b.proto"),
                package: "a".into(),
                imports: vec![
                    Proto {
                        path: PathBuf::from("/a/c.proto"),
                        package: "a".into(),
                        imports: vec![Proto {
                            path: PathBuf::from("/c/e.proto"),
                            package: "c".into(),
                            imports: vec![Proto {
                                path: PathBuf::from("/d/f.proto"),
                                package: "d".into(),
                                imports: Vec::new(),
                            }],
                        }],
                    },
                    Proto {
                        path: PathBuf::from("/b/d.proto"),
                        package: "b".into(),
                        imports: vec![Proto {
                            path: PathBuf::from("/c/e.proto"),
                            package: "c".into(),
                            imports: Vec::new(),
                        }],
                    },
                ],
            },
            Proto {
                path: PathBuf::from("/a/c.proto"),
                package: "a".into(),
                imports: vec![Proto {
                    path: PathBuf::from("/c/e.proto"),
                    package: "c".into(),
                    imports: vec![Proto {
                        path: PathBuf::from("/d/f.proto"),
                        package: "d".into(),
                        imports: Vec::new(),
                    }],
                }],
            },
            Proto {
                path: PathBuf::from("/c/e.proto"),
                package: "c".into(),
                imports: vec![Proto {
                    path: PathBuf::from("/d/f.proto"),
                    package: "d".into(),
                    imports: Vec::new(),
                }],
            },
            Proto {
                path: PathBuf::from("/d/f.proto"),
                package: "d".into(),
                imports: Vec::new(),
            },
            Proto {
                path: PathBuf::from("/b/d.proto"),
                package: "b".into(),
                imports: vec![Proto {
                    path: PathBuf::from("/c/e.proto"),
                    package: "c".into(),
                    imports: Vec::new(),
                }],
            },
            Proto {
                path: PathBuf::from("/c/e.proto"),
                package: "c".into(),
                imports: Vec::new(),
            },
        ]
    }

    #[test]
    fn test_add_deps_rec() {
        let protos = protos();
        let mut map = HashMap::new();
        for proto in protos {
            add_deps_rec(&proto, &proto, &mut map);
        }

        assert_eq!(map, {
            let mut map = HashMap::new();
            map.insert("a".into(), {
                let mut set = HashSet::new();
                set.insert("a".into());
                set
            });
            map.insert("b".into(), {
                let mut set = HashSet::new();
                set.insert("a".into());
                set
            });
            map.insert("c".into(), {
                let mut set = HashSet::new();
                set.insert("a".into());
                set.insert("b".into());
                set
            });
            map.insert("d".into(), {
                let mut set = HashSet::new();
                set.insert("a".into());
                set.insert("c".into());
                set
            });
            map
        });
    }

    #[test]
    fn test_from_protos() {
        let protos = protos();

        assert_eq!(from_protos(protos), {
            let mut map = HashMap::new();
            map.insert(
                "a".into(),
                Module {
                    package: "a".into(),
                    include: true,
                    imported_by: {
                        let mut set = HashSet::new();
                        set.insert("a".into());
                        set
                    },
                    children: HashMap::new(),
                },
            );
            map.insert(
                "b".into(),
                Module {
                    package: "b".into(),
                    include: true,
                    imported_by: {
                        let mut set = HashSet::new();
                        set.insert("a".into());
                        set
                    },
                    children: HashMap::new(),
                },
            );
            map.insert(
                "c".into(),
                Module {
                    package: "c".into(),
                    include: true,
                    imported_by: {
                        let mut set = HashSet::new();
                        set.insert("a".into());
                        set.insert("b".into());
                        set
                    },
                    children: HashMap::new(),
                },
            );
            map.insert(
                "d".into(),
                Module {
                    package: "d".into(),
                    include: true,
                    imported_by: {
                        let mut set = HashSet::new();
                        set.insert("a".into());
                        set.insert("c".into());
                        set
                    },
                    children: HashMap::new(),
                },
            );
            RootModule(map)
        });
    }

    #[test]
    fn test_root_module_gen_code() {
        let root = from_protos(protos());
        assert_eq!(
            root.gen_code(),
            r###"pub mod a { #[cfg(any(feature = "a",))]
include_proto!("a");
  }
pub mod b { #[cfg(any(feature = "a",feature = "b",))]
include_proto!("b");
  }
pub mod c { #[cfg(any(feature = "a",feature = "b",feature = "c",))]
include_proto!("c");
  }
pub mod d { #[cfg(any(feature = "a",feature = "c",feature = "d",))]
include_proto!("d");
  }
"###
        );
    }

    #[test]
    fn test_module_gen_code() {
        let module = Module {
            package: "mechiru.type".into(),
            include: true,
            imported_by: HashSet::new(),
            children: HashMap::new(),
        };
        assert_eq!(
            module.gen_code(),
            r###"#[cfg(any(feature = "mechiru-type",))]
include_proto!("mechiru.r#type");
 "###
        );
    }
}
