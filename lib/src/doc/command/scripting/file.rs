use cmake_parser_derive::CMake;

use crate::{
    command::common::{Condition, FileMatch, HashAlgorithm, NewlineStyle, Permission, Permissions},
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// File manipulation command.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/file.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum File<'t> {
    Reading(FileReading<'t>),
    Writing(FileWriting<'t>),
    Filesystem(FileFilesystem<'t>),
    PathConversion(FilePathConversion<'t>),
    Transfer(FileTransfer<'t>),
    Locking(FileLocking<'t>),
    Archiving(FileArchiving<'t>),
}

impl<'t> ToCommandScope for File<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum FileReading<'t> {
    #[cmake(transparent)]
    Read(FileRead<'t>),
    #[cmake(transparent)]
    Strings(FileStrings<'t>),
    Hash(FileHash<'t>),
    #[cmake(transparent)]
    Timestamp(FileTimestamp<'t>),
    #[cmake(transparent)]
    GetRuntimeDependencies(Box<FileGetRuntimeDependencies<'t>>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileRead<'t> {
    #[cmake(positional)]
    pub filename: Token<'t>,
    #[cmake(positional)]
    pub variable: Token<'t>,
    pub offset: Option<Token<'t>>,
    pub limit: Option<Token<'t>>,
    pub hex: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileStrings<'t> {
    #[cmake(positional)]
    pub filename: Token<'t>,
    #[cmake(positional)]
    pub variable: Token<'t>,
    pub options: Option<Vec<StringsOption<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum StringsOption<'t> {
    LengthMaximum(Token<'t>),
    LengthMinimum(Token<'t>),
    LimitCount(Token<'t>),
    LimitInput(Token<'t>),
    LimitOutput(Token<'t>),
    NewlineConsume,
    NoHexConversion,
    Regex(Token<'t>),
    Encoding(StringsEncoding),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum StringsEncoding {
    #[cmake(rename = "UTF-8")]
    Utf8,
    #[cmake(rename = "UTF-16LE")]
    Utf16LE,
    #[cmake(rename = "UTF-16BE")]
    Utf16BE,
    #[cmake(rename = "UTF-32LE")]
    Utf32LE,
    #[cmake(rename = "UTF-32BE")]
    Utf32BE,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct FileHash<'t> {
    pub hash_algorithm: HashAlgorithm,
    pub filename: Token<'t>,
    pub variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "format")]
pub struct FileTimestamp<'t> {
    #[cmake(positional)]
    pub filename: Token<'t>,
    #[cmake(positional)]
    pub variable: Token<'t>,
    #[cmake(rename = "")]
    pub format: Option<Token<'t>>,
    pub utc: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileGetRuntimeDependencies<'t> {
    pub resolved_dependencies_var: Option<Token<'t>>,
    pub unresolved_dependencies_var: Option<Token<'t>>,
    pub conflicting_dependencies_prefix: Option<Token<'t>>,
    pub executables: Option<Vec<Token<'t>>>,
    pub libraries: Option<Vec<Token<'t>>>,
    pub modules: Option<Vec<Token<'t>>>,
    pub directories: Option<Vec<Token<'t>>>,
    pub bundle_executable: Option<Token<'t>>,
    pub pre_include_regexes: Option<Vec<Token<'t>>>,
    pub pre_exclude_regexes: Option<Vec<Token<'t>>>,
    pub post_include_regexes: Option<Vec<Token<'t>>>,
    pub post_exclude_regexes: Option<Vec<Token<'t>>>,
    pub post_include_files: Option<Vec<Token<'t>>>,
    pub post_exclude_files: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum FileWriting<'t> {
    Write(FileWrite<'t>),
    Append(FileWrite<'t>),
    Touch(FileTouch<'t>),
    #[cmake(rename = "TOUCH_NOCREATE")]
    TouchNoCreate(FileTouch<'t>),
    Generate(FileGenerate<'t>),
    Configure(FileConfigure<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct FileWrite<'t> {
    pub filename: Token<'t>,
    pub content: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct FileTouch<'t> {
    pub files: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileGenerate<'t> {
    #[cmake(positional, transparent)]
    pub output: Token<'t>,
    #[cmake(positional)]
    pub input: GenerateInput<'t>,
    pub condition: Option<Condition<'t>>,
    pub target: Option<Token<'t>>,
    pub permissions: Option<Permissions<'t>>,
    pub newline_style: Option<NewlineStyle>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum GenerateInput<'t> {
    Input(Token<'t>),
    Content(Token<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileConfigure<'t> {
    #[cmake(positional, transparent)]
    pub output: Token<'t>,
    #[cmake(positional, transparent)]
    pub content: Token<'t>,
    pub escape_quotes: bool,
    #[cmake(rename = "@ONLY")]
    pub only: bool,
    pub newline_style: Option<NewlineStyle>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum FileFilesystem<'t> {
    Glob(FileGlob<'t>),
    GlobRecurse(FileGlobRecurse<'t>),
    MakeDirectory(FileMakeDirectory<'t>),
    Remove(FileRemove<'t>),
    RemoveRecurse(FileRemove<'t>),
    Rename(FileRename<'t>),
    CopyFile(FileCopyFile<'t>),
    Copy(FileCopy<'t>),
    Install(FileInstall<'t>),
    Size(FileSize<'t>),
    ReadSymlink(FileReadSymlink<'t>),
    CreateLink(FileCreateLink<'t>),
    Chmod(FileChmod<'t>),
    ChmodRecurse(FileChmod<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "globbing_expressions")]
pub struct FileGlob<'t> {
    #[cmake(positional)]
    pub variable: Token<'t>,
    pub list_directories: Option<ListDirectories>,
    pub relative: Option<Token<'t>>,
    pub configure_depends: bool,
    #[cmake(rename = "")]
    pub globbing_expressions: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "globbing_expressions")]
pub struct FileGlobRecurse<'t> {
    #[cmake(positional)]
    pub variable: Token<'t>,
    pub follow_symlinks: bool,
    pub list_directories: Option<ListDirectories>,
    pub relative: Option<Token<'t>>,
    pub configure_depends: bool,
    #[cmake(rename = "")]
    pub globbing_expressions: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum ListDirectories {
    #[cmake(rename = "true")]
    True,
    #[cmake(rename = "false")]
    False,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional, allow_empty)]
pub struct FileMakeDirectory<'t> {
    pub directories: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional, allow_empty)]
pub struct FileRemove<'t> {
    pub files: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileRename<'t> {
    #[cmake(positional)]
    pub oldname: Token<'t>,
    #[cmake(positional)]
    pub newname: Token<'t>,
    pub result: Option<Token<'t>>,
    pub no_replace: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileCopyFile<'t> {
    #[cmake(positional)]
    pub oldname: Token<'t>,
    #[cmake(positional)]
    pub newname: Token<'t>,
    pub result: Option<Token<'t>>,
    pub only_if_different: bool,
    pub input_may_be_recent: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "files")]
pub struct FileCopy<'t> {
    #[cmake(rename = "")]
    pub files: Vec<Token<'t>>,
    pub destination: Token<'t>,
    pub source_permissions: Option<SourcePermissions>,
    pub file_permissions: Option<Vec<Permission>>,
    pub directory_permissions: Option<Vec<Permission>>,
    pub follow_symlink_chain: bool,
    pub files_matching: bool,
    pub file_matches: Option<Vec<FileMatch<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum SourcePermissions {
    #[cmake(rename = "NO_SOURCE_PERMISSIONS")]
    No,
    #[cmake(rename = "USE_SOURCE_PERMISSIONS")]
    Use,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "files")]
pub struct FileInstall<'t> {
    pub files: Vec<Token<'t>>,
    pub destination: Token<'t>,
    pub source_permissions: Option<SourcePermissions>,
    pub file_permissions: Option<Vec<Permission>>,
    pub directory_permissions: Option<Vec<Permission>>,
    pub follow_symlink_chain: bool,
    pub files_matching: bool,
    pub file_matches: Option<Vec<FileMatch<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct FileSize<'t> {
    pub filename: Token<'t>,
    pub variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct FileReadSymlink<'t> {
    pub linkname: Token<'t>,
    pub variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileCreateLink<'t> {
    #[cmake(positional)]
    pub original: Token<'t>,
    #[cmake(positional)]
    pub linkname: Token<'t>,
    pub result: Option<Token<'t>>,
    pub copy_on_error: bool,
    pub symbolic: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "files")]
pub struct FileChmod<'t> {
    #[cmake(rename = "")]
    pub files: Vec<Token<'t>>,
    pub permissions: Option<Vec<Permission>>,
    pub file_permissions: Option<Vec<Permission>>,
    pub directory_permissions: Option<Vec<Permission>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
#[allow(clippy::enum_variant_names)]
pub enum FilePathConversion<'t> {
    RealPath(FileRealPath<'t>),
    RelativePath(FileRelativePath<'t>),
    #[cmake(rename = "TO_CMAKE_PATH")]
    ToCMakePath(FileToCMakePath<'t>),
    ToNativePath(FileToNativePath<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileRealPath<'t> {
    #[cmake(positional)]
    pub path: Token<'t>,
    #[cmake(positional)]
    pub out_var: Token<'t>,
    pub base_directory: Option<Token<'t>>,
    pub expand_tilde: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct FileRelativePath<'t> {
    pub variable: Token<'t>,
    pub directory: Token<'t>,
    pub file: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct FileToCMakePath<'t> {
    pub path: Token<'t>,
    pub variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct FileToNativePath<'t> {
    pub path: Token<'t>,
    pub variable: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum FileTransfer<'t> {
    Download(FileDownload<'t>),
    Upload(FileUpload<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "file")]
pub struct FileDownload<'t> {
    #[cmake(positional)]
    pub url: Token<'t>,
    #[cmake(rename = "")]
    pub file: Option<Token<'t>>,
    pub options: Option<Vec<DownloadOption<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum DownloadOption<'t> {
    InactivityTimeout(Token<'t>),
    Log(Token<'t>),
    ShowProgress,
    Status(Token<'t>),
    Timeout(Token<'t>),
    #[cmake(rename = "USERPWD")]
    UserPwd(Token<'t>),
    #[cmake(rename = "HTTPHEADER")]
    HttpHeader(Token<'t>),
    #[cmake(rename = "NETRC")]
    NetRC(NetRCLevel),
    #[cmake(rename = "NETRC_FILE")]
    NetRCFile(Token<'t>),
    TlsVerify(TlsVerify),
    #[cmake(rename = "TLS_CAINFO")]
    TlsCAInfo(Token<'t>),
    ExpectedHash(Token<'t>), // TODO: specific parser for ALGO=<value>
    ExpectedMD5(Token<'t>),
    RangeStart(Token<'t>),
    RangeEnd(Token<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum NetRCLevel {
    Ignored,
    Optional,
    Required,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum TlsVerify {
    On,
    Off,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileUpload<'t> {
    #[cmake(positional)]
    pub file: Token<'t>,
    #[cmake(positional)]
    pub url: Token<'t>,
    pub options: Option<Vec<UploadOption<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum UploadOption<'t> {
    InactivityTimeout(Token<'t>),
    Log(Token<'t>),
    ShowProgress,
    Status(Token<'t>),
    Timeout(Token<'t>),
    #[cmake(rename = "USERPWD")]
    UserPwd(Token<'t>),
    #[cmake(rename = "HTTPHEADER")]
    HttpHeader(Token<'t>),
    #[cmake(rename = "NETRC")]
    NetRC(NetRCLevel),
    #[cmake(rename = "NETRC_FILE")]
    NetRCFile(Token<'t>),
    TlsVerify(TlsVerify),
    #[cmake(rename = "TLS_CAINFO")]
    TlsCAInfo(Token<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum FileLocking<'t> {
    Lock(FileLock<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileLock<'t> {
    #[cmake(positional)]
    pub path: Token<'t>,
    pub directory: bool,
    pub release: bool,
    pub guard: Option<LockGuard>,
    pub result_variable: Option<Token<'t>>,
    pub timeout: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum LockGuard {
    Function,
    File,
    Process,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum FileArchiving<'t> {
    ArchiveCreate(FileArchiveCreate<'t>),
    ArchiveExtract(FileArchiveExtract<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileArchiveCreate<'t> {
    #[cmake(positional, transparent)]
    pub output: Token<'t>,
    pub paths: Vec<Token<'t>>,
    pub format: Option<ArchiveFormat>,
    pub compression: Option<ArchiveCompression<'t>>,
    pub mtime: Option<Token<'t>>,
    pub verbose: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum ArchiveFormat {
    #[cmake(rename = "7zip")]
    SevenZip,
    #[cmake(rename = "gnutar")]
    GnuTar,
    #[cmake(rename = "pax")]
    Pax,
    #[cmake(rename = "paxr")]
    PaxR,
    #[cmake(rename = "raw")]
    Raw,
    #[cmake(rename = "zip")]
    Zip,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ArchiveCompression<'t> {
    pub compression: Compression,
    #[cmake(transparent)]
    pub compression_level: Option<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum Compression {
    #[cmake(rename = "None")]
    None,
    #[cmake(rename = "BZip2")]
    BZip2,
    #[cmake(rename = "GZip")]
    GZip,
    #[cmake(rename = "XZ")]
    Xz,
    #[cmake(rename = "Zstd")]
    Zstd,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileArchiveExtract<'t> {
    #[cmake(positional, transparent)]
    pub input: Token<'t>,
    pub destination: Option<Token<'t>>,
    pub patterns: Option<Vec<Token<'t>>>,
    pub list_only: bool,
    pub verbose: bool,
    pub touch: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::common::FileMatchKind;
    use crate::doc::cmake_parse::tests::{quoted_token, token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn file() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/file");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::File(Box::new(File::Reading(FileReading::Read(FileRead {
                    filename: token(b"filename1"),
                    variable: token(b"variable1"),
                    offset: Some(token(b"offset1")),
                    limit: Some(token(b"limit1")),
                    hex: true,
                })))),
                Command::File(Box::new(File::Reading(FileReading::Strings(FileStrings {
                    filename: token(b"filename1"),
                    variable: token(b"variable1"),
                    options: None,
                })))),
                Command::File(Box::new(File::Reading(FileReading::Strings(FileStrings {
                    filename: token(b"filename1"),
                    variable: token(b"variable1"),
                    options: Some(vec![
                        StringsOption::LengthMaximum(token(b"length_maximum1")),
                        StringsOption::LengthMinimum(token(b"length_minimum1")),
                        StringsOption::LimitCount(token(b"limit_count1")),
                        StringsOption::LimitInput(token(b"limit_input1")),
                        StringsOption::LimitOutput(token(b"limit_output1")),
                        StringsOption::NewlineConsume,
                        StringsOption::NoHexConversion,
                        StringsOption::Regex(token(b"regex1")),
                        StringsOption::Encoding(StringsEncoding::Utf8),
                        StringsOption::Encoding(StringsEncoding::Utf16LE),
                        StringsOption::Encoding(StringsEncoding::Utf16BE),
                        StringsOption::Encoding(StringsEncoding::Utf32LE),
                        StringsOption::Encoding(StringsEncoding::Utf32BE),
                    ]),
                })))),
                Command::File(Box::new(File::Reading(FileReading::Hash(FileHash {
                    hash_algorithm: HashAlgorithm::SHA1,
                    filename: token(b"filename1"),
                    variable: token(b"variable1"),
                })))),
                Command::File(Box::new(File::Reading(FileReading::Timestamp(
                    FileTimestamp {
                        filename: token(b"filename1"),
                        variable: token(b"variable1"),
                        format: Some(token(b"format1")),
                        utc: true,
                    }
                )))),
                Command::File(Box::new(File::Reading(
                    FileReading::GetRuntimeDependencies(Box::new(FileGetRuntimeDependencies {
                        resolved_dependencies_var: Some(token(b"resolved_dependencies_var1")),
                        unresolved_dependencies_var: Some(token(b"unresolved_dependencies_var1")),
                        conflicting_dependencies_prefix: Some(token(
                            b"conflicting_dependencies_prefix1"
                        )),
                        executables: Some(tokens_vec([b"executables1", b"executables2"])),
                        libraries: Some(tokens_vec([b"libraries1", b"libraries2"])),
                        modules: Some(tokens_vec([b"modules1", b"modules2"])),
                        directories: Some(tokens_vec([b"directories1", b"directories2"])),
                        bundle_executable: Some(token(b"bundle_executable1")),
                        pre_include_regexes: Some(tokens_vec([
                            b"pre_include_regexes1",
                            b"pre_include_regexes2"
                        ])),
                        pre_exclude_regexes: Some(tokens_vec([
                            b"pre_exclude_regexes1",
                            b"pre_exclude_regexes2"
                        ])),
                        post_include_regexes: Some(tokens_vec([
                            b"post_include_regexes1",
                            b"post_include_regexes2"
                        ])),
                        post_exclude_regexes: Some(tokens_vec([
                            b"post_exclude_regexes1",
                            b"post_exclude_regexes2"
                        ])),
                        post_include_files: Some(tokens_vec([
                            b"post_include_files1",
                            b"post_include_files2"
                        ])),
                        post_exclude_files: Some(tokens_vec([
                            b"post_exclude_files1",
                            b"post_exclude_files2"
                        ])),
                    }))
                ))),
                Command::File(Box::new(File::Writing(FileWriting::Write(FileWrite {
                    filename: token(b"filename1"),
                    content: Some(tokens_vec([b"content1"])),
                })))),
                Command::File(Box::new(File::Writing(FileWriting::Append(FileWrite {
                    filename: token(b"filename1"),
                    content: Some(tokens_vec([b"content1"])),
                })))),
                Command::File(Box::new(File::Writing(FileWriting::Touch(FileTouch {
                    files: Some(tokens_vec([b"file1", b"file2"])),
                })))),
                Command::File(Box::new(File::Writing(FileWriting::TouchNoCreate(
                    FileTouch {
                        files: Some(tokens_vec([b"file1", b"file2"])),
                    }
                )))),
                Command::File(Box::new(File::Writing(FileWriting::Generate(
                    FileGenerate {
                        output: token(b"output1"),
                        input: GenerateInput::Input(token(b"input1")),
                        condition: None,
                        target: None,
                        permissions: None,
                        newline_style: None,
                    }
                )))),
                Command::File(Box::new(File::Writing(FileWriting::Generate(
                    FileGenerate {
                        output: token(b"output1"),
                        input: GenerateInput::Content(token(b"content1")),
                        condition: None,
                        target: None,
                        permissions: None,
                        newline_style: None,
                    }
                )))),
                Command::File(Box::new(File::Writing(FileWriting::Generate(
                    FileGenerate {
                        output: token(b"output1"),
                        input: GenerateInput::Input(token(b"input1")),
                        condition: Some(Condition {
                            conditions: tokens_vec([b"condition1"]),
                        }),
                        target: Some(token(b"target1")),
                        permissions: Some(Permissions::File(tokens_vec([b"file1", b"file2"]))),
                        newline_style: Some(NewlineStyle::Dos),
                    }
                )))),
                Command::File(Box::new(File::Writing(FileWriting::Configure(
                    FileConfigure {
                        output: token(b"output1"),
                        content: token(b"content1"),
                        escape_quotes: false,
                        only: false,
                        newline_style: None,
                    }
                )))),
                Command::File(Box::new(File::Writing(FileWriting::Configure(
                    FileConfigure {
                        output: token(b"output1"),
                        content: token(b"content1"),
                        escape_quotes: true,
                        only: true,
                        newline_style: Some(NewlineStyle::CrLf),
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::Glob(FileGlob {
                    variable: token(b"variable1"),
                    list_directories: None,
                    relative: None,
                    configure_depends: false,
                    globbing_expressions: None,
                })))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::Glob(FileGlob {
                    variable: token(b"variable1"),
                    list_directories: Some(ListDirectories::True),
                    relative: Some(token(b"relative1")),
                    configure_depends: true,
                    globbing_expressions: Some(tokens_vec([b"gexp1", b"gexp2"])),
                })))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::GlobRecurse(
                    FileGlobRecurse {
                        variable: token(b"variable1"),
                        follow_symlinks: false,
                        list_directories: None,
                        relative: None,
                        configure_depends: false,
                        globbing_expressions: None,
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::GlobRecurse(
                    FileGlobRecurse {
                        variable: token(b"variable1"),
                        follow_symlinks: true,
                        list_directories: Some(ListDirectories::False),
                        relative: Some(token(b"relative1")),
                        configure_depends: true,
                        globbing_expressions: Some(tokens_vec([b"gexp1", b"gexp2"])),
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::MakeDirectory(
                    FileMakeDirectory { directories: None }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::MakeDirectory(
                    FileMakeDirectory {
                        directories: Some(tokens_vec([b"dir1", b"dir2"])),
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::Remove(
                    FileRemove { files: None }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::Remove(
                    FileRemove {
                        files: Some(tokens_vec([b"file1", b"file2"])),
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::RemoveRecurse(
                    FileRemove { files: None }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::RemoveRecurse(
                    FileRemove {
                        files: Some(tokens_vec([b"file1", b"file2"])),
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::Rename(
                    FileRename {
                        oldname: token(b"oldname1"),
                        newname: token(b"newname1"),
                        result: Some(token(b"result1")),
                        no_replace: true,
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::CopyFile(
                    FileCopyFile {
                        oldname: token(b"oldname1"),
                        newname: token(b"newname1"),
                        result: Some(token(b"result1")),
                        only_if_different: true,
                        input_may_be_recent: true,
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::Copy(FileCopy {
                    files: tokens_vec([b"/opt/foo/lib/libfoo.so"]),
                    destination: token(b"lib"),
                    source_permissions: None,
                    file_permissions: None,
                    directory_permissions: None,
                    follow_symlink_chain: true,
                    files_matching: false,
                    file_matches: None,
                })))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::Install(
                    FileInstall {
                        files: tokens_vec([b"/opt/foo/bin/foo", b"/opt/foo/bin/boo"]),
                        destination: token(b"bin"),
                        source_permissions: None,
                        file_permissions: None,
                        directory_permissions: None,
                        follow_symlink_chain: false,
                        files_matching: true,
                        file_matches: Some(vec![
                            FileMatch {
                                kind: Some(FileMatchKind::Pattern(quoted_token(b"CVS"),),),
                                exclude: true,
                                permissions: None,
                            },
                            FileMatch {
                                kind: Some(FileMatchKind::Pattern(quoted_token(b"scripts/*"),),),
                                exclude: false,
                                permissions: Some(vec![
                                    Permission::OwnerExecute,
                                    Permission::OwnerWrite,
                                    Permission::OwnerRead,
                                    Permission::GroupExecute,
                                    Permission::GroupRead,
                                ]),
                            },
                        ],),
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::Size(FileSize {
                    filename: token(b"filename1"),
                    variable: token(b"variable1"),
                })))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::ReadSymlink(
                    FileReadSymlink {
                        linkname: token(b"linkname1"),
                        variable: token(b"variable1"),
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::CreateLink(
                    FileCreateLink {
                        original: token(b"original1"),
                        linkname: token(b"linkname1"),
                        result: None,
                        copy_on_error: false,
                        symbolic: false,
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::CreateLink(
                    FileCreateLink {
                        original: token(b"original1"),
                        linkname: token(b"linkname1"),
                        result: Some(token(b"result1")),
                        copy_on_error: true,
                        symbolic: true,
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::Chmod(
                    FileChmod {
                        files: tokens_vec([b"file1"]),
                        permissions: Some(vec![Permission::OwnerRead, Permission::OwnerWrite]),
                        file_permissions: None,
                        directory_permissions: None,
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::Chmod(
                    FileChmod {
                        files: tokens_vec([b"file1", b"file2"]),
                        permissions: None,
                        file_permissions: Some(vec![
                            Permission::OwnerWrite,
                            Permission::OwnerExecute
                        ]),
                        directory_permissions: None,
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::Chmod(
                    FileChmod {
                        files: tokens_vec([b"file1", b"file2", b"file3"]),
                        permissions: None,
                        file_permissions: None,
                        directory_permissions: Some(vec![
                            Permission::SetGID,
                            Permission::OwnerWrite
                        ]),
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::ChmodRecurse(
                    FileChmod {
                        files: tokens_vec([b"file1"]),
                        permissions: Some(vec![Permission::OwnerRead, Permission::OwnerWrite]),
                        file_permissions: None,
                        directory_permissions: None,
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::ChmodRecurse(
                    FileChmod {
                        files: tokens_vec([b"file1", b"file2"]),
                        permissions: None,
                        file_permissions: Some(vec![
                            Permission::OwnerWrite,
                            Permission::OwnerExecute
                        ]),
                        directory_permissions: None,
                    }
                )))),
                Command::File(Box::new(File::Filesystem(FileFilesystem::ChmodRecurse(
                    FileChmod {
                        files: tokens_vec([b"file1", b"file2", b"file3"]),
                        permissions: None,
                        file_permissions: None,
                        directory_permissions: Some(vec![
                            Permission::SetGID,
                            Permission::OwnerWrite
                        ]),
                    }
                )))),
                Command::File(Box::new(File::PathConversion(
                    FilePathConversion::RealPath(FileRealPath {
                        path: token(b"path1"),
                        out_var: token(b"out_var1"),
                        base_directory: None,
                        expand_tilde: false,
                    })
                ))),
                Command::File(Box::new(File::PathConversion(
                    FilePathConversion::RealPath(FileRealPath {
                        path: token(b"path1"),
                        out_var: token(b"out_var1"),
                        base_directory: Some(token(b"base_directory1")),
                        expand_tilde: true,
                    })
                ))),
                Command::File(Box::new(File::PathConversion(
                    FilePathConversion::RelativePath(FileRelativePath {
                        variable: token(b"variable1"),
                        directory: token(b"directory1"),
                        file: token(b"file1"),
                    })
                ))),
                Command::File(Box::new(File::PathConversion(
                    FilePathConversion::ToCMakePath(FileToCMakePath {
                        path: token(b"path1"),
                        variable: token(b"variable1"),
                    })
                ))),
                Command::File(Box::new(File::PathConversion(
                    FilePathConversion::ToNativePath(FileToNativePath {
                        path: token(b"path1"),
                        variable: token(b"variable1"),
                    })
                ))),
                Command::File(Box::new(File::Transfer(FileTransfer::Download(
                    FileDownload {
                        url: token(b"url1"),
                        file: None,
                        options: None,
                    }
                )))),
                Command::File(Box::new(File::Transfer(FileTransfer::Download(
                    FileDownload {
                        url: token(b"url1"),
                        file: Some(token(b"file1")),
                        options: Some(vec![
                            DownloadOption::InactivityTimeout(token(b"inactivity_timeout1")),
                            DownloadOption::Log(token(b"log1")),
                            DownloadOption::ShowProgress,
                            DownloadOption::Status(token(b"status1")),
                            DownloadOption::Timeout(token(b"timeout1")),
                            DownloadOption::UserPwd(token(b"userpwd1")),
                            DownloadOption::HttpHeader(token(b"header1:value1")),
                            DownloadOption::NetRC(NetRCLevel::Ignored),
                            DownloadOption::NetRC(NetRCLevel::Optional),
                            DownloadOption::NetRC(NetRCLevel::Required),
                            DownloadOption::NetRCFile(token(b"netrc_file1")),
                            DownloadOption::TlsVerify(TlsVerify::On),
                            DownloadOption::TlsVerify(TlsVerify::Off),
                            DownloadOption::TlsCAInfo(token(b"tls_cainfo1")),
                            DownloadOption::ExpectedHash(token(b"MD5=12345")),
                            DownloadOption::ExpectedMD5(token(b"expected_md5")),
                            DownloadOption::RangeStart(token(b"range_start1")),
                            DownloadOption::RangeEnd(token(b"range_end1")),
                        ]),
                    }
                )))),
                Command::File(Box::new(File::Transfer(FileTransfer::Upload(FileUpload {
                    file: token(b"file1"),
                    url: token(b"url1"),
                    options: None,
                })))),
                Command::File(Box::new(File::Transfer(FileTransfer::Upload(FileUpload {
                    file: token(b"file1"),
                    url: token(b"url1"),
                    options: Some(vec![
                        UploadOption::InactivityTimeout(token(b"inactivity_timeout1")),
                        UploadOption::Log(token(b"log1")),
                        UploadOption::ShowProgress,
                        UploadOption::Status(token(b"status1")),
                        UploadOption::Timeout(token(b"timeout1")),
                        UploadOption::UserPwd(token(b"userpwd1")),
                        UploadOption::HttpHeader(token(b"header1:value1")),
                        UploadOption::NetRC(NetRCLevel::Ignored),
                        UploadOption::NetRC(NetRCLevel::Optional),
                        UploadOption::NetRC(NetRCLevel::Required),
                        UploadOption::NetRCFile(token(b"netrc_file1")),
                        UploadOption::TlsVerify(TlsVerify::On),
                        UploadOption::TlsVerify(TlsVerify::Off),
                        UploadOption::TlsCAInfo(token(b"tls_cainfo1")),
                    ]),
                })))),
                Command::File(Box::new(File::Locking(FileLocking::Lock(FileLock {
                    path: token(b"path1"),
                    directory: false,
                    release: false,
                    guard: None,
                    result_variable: None,
                    timeout: None,
                })))),
                Command::File(Box::new(File::Locking(FileLocking::Lock(FileLock {
                    path: token(b"path1"),
                    directory: false,
                    release: false,
                    guard: Some(LockGuard::File),
                    result_variable: None,
                    timeout: None,
                })))),
                Command::File(Box::new(File::Locking(FileLocking::Lock(FileLock {
                    path: token(b"path1"),
                    directory: false,
                    release: false,
                    guard: Some(LockGuard::Process),
                    result_variable: None,
                    timeout: None,
                })))),
                Command::File(Box::new(File::Locking(FileLocking::Lock(FileLock {
                    path: token(b"path1"),
                    directory: true,
                    release: true,
                    guard: Some(LockGuard::Function),
                    result_variable: Some(token(b"result_variable1")),
                    timeout: Some(token(b"timeout1")),
                })))),
                Command::File(Box::new(File::Archiving(FileArchiving::ArchiveCreate(
                    FileArchiveCreate {
                        output: token(b"output1"),
                        paths: tokens_vec([b"path1", b"path2"]),
                        format: None,
                        compression: None,
                        mtime: None,
                        verbose: false,
                    }
                )))),
                Command::File(Box::new(File::Archiving(FileArchiving::ArchiveCreate(
                    FileArchiveCreate {
                        output: token(b"output1"),
                        paths: tokens_vec([b"path1", b"path2"]),
                        format: Some(ArchiveFormat::PaxR),
                        compression: Some(ArchiveCompression {
                            compression: Compression::Xz,
                            compression_level: Some(token(b"5")),
                        }),
                        mtime: Some(token(b"mtime1")),
                        verbose: true,
                    }
                )))),
                Command::File(Box::new(File::Archiving(FileArchiving::ArchiveExtract(
                    FileArchiveExtract {
                        input: token(b"input1"),
                        destination: None,
                        patterns: None,
                        list_only: false,
                        verbose: false,
                        touch: false,
                    }
                )))),
                Command::File(Box::new(File::Archiving(FileArchiving::ArchiveExtract(
                    FileArchiveExtract {
                        input: token(b"input1"),
                        destination: Some(token(b"destination1")),
                        patterns: Some(tokens_vec([b"pattern1", b"pattern2"])),
                        list_only: true,
                        verbose: true,
                        touch: true,
                    }
                )))),
            ])
        )
    }
}
