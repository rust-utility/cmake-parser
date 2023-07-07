use cmake_parser_derive::CMake;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum WindowsRegistryView {
    #[cmake(rename = "64")]
    Bits64,
    #[cmake(rename = "32")]
    Bits32,
    #[cmake(rename = "64_32")]
    Bits64Fallback32,
    #[cmake(rename = "32_64")]
    Bits32Fallback64,
    Host,
    Target,
    Both,
}
