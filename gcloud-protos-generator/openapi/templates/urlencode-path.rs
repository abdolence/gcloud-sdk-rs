
/// Encodes a path parameter that holds a full resource name (e.g. `projects/my-project`),
/// preserving `/` as a path separator while encoding each segment.
pub fn urlencode_path<T: AsRef<str>>(s: T) -> String {
    s.as_ref()
        .split('/')
        .map(urlencode)
        .collect::<Vec<String>>()
        .join("/")
}
