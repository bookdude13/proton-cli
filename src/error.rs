extern crate openssl;

use openssl::error as openssl_err;
use postgres::error as postgres_err;
use rustc_serialize::json;
use std::{io, error, fmt};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    JsonEncode(json::EncoderError),
    JsonDecode(json::DecoderError),
    JsonParse(json::ParserError),
    Ssl(openssl_err::ErrorStack),
    Rsfml(String),
    Postgres(postgres_err::Error),
    PostgresConnection(postgres_err::ConnectError),
    FolderNotEmpty(String, usize),
    InvalidFileName,
    InvalidFrameDuration(u32),
    InvalidLayout(String),
    InvalidNumResults(usize),
    InvalidPatch(String),
    InvalidPermissionName(String),
    InvalidProjectName(String),
    InvalidPublicKey(String),
    InvalidSequenceName(String),
    InvalidSequenceSection(u32),
    InvalidVixenData(String),
    LoadProjectError,
    MissingPermissionArg,
    OffsetOutOfBounds(u32, u32),
    DuplicateUser(String, String),
    DuplicateSequence(String),
    MusicFileNotFound(String),
    UnsupportedFileType(String),
    AdminNotFound,
    ChannelNotFound(u32),
    ChannelDataNotFound(u32, u32),
    FixtureNotFound(u32),
    LayoutNotFound(u32),
    ProjectNotFound(String),
    SequenceNotFound(u32),
    UserNotFound,
    UnauthorizedAction,
    TodoErr,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "IO error occurred",
            Error::JsonDecode(_) => "Json decoding error occurred",
            Error::JsonEncode(_) => "Json encoding error occurred",
            Error::JsonParse(_) => "Json parsing error occurred",
            Error::Ssl(_) => "SSL error occured",
            Error::Rsfml(_) => "Rsfml error occured",
            Error::Postgres(_) => "Postgres error occured",
            Error::PostgresConnection(_) => "Postgres connection error occured",
            Error::FolderNotEmpty(_, _) => "Root folder was not empty",
            Error::InvalidFileName => "Invalid file name",
            Error::InvalidFrameDuration(_) => "Invalid frame duration",
            Error::InvalidLayout(_) => "Invalid layout",
            Error::InvalidNumResults(_) => "Invalid number of results returned",
            Error::InvalidPatch(_) => "Invalid patch file",
            Error::InvalidPermissionName(_) => "Invalid permission name",
            Error::InvalidProjectName(_) => "Invalid project name",
            Error::InvalidPublicKey(_) => "Invalid public key",
            Error::InvalidSequenceName(_) => "Invalid sequence name",
            Error::InvalidSequenceSection(_) => "Invalid sequence section",
            Error::InvalidVixenData(_) => "Invalid Vixen data",
            Error::LoadProjectError => "Loading project failed",
            Error::MissingPermissionArg => "Permission argument required but missing (seqid or secid)",
            Error::OffsetOutOfBounds(_, _) => "Offset out of bouds",
            Error::DuplicateUser(_, _) => "User already exists",
            Error::DuplicateSequence(_) => "Sequence already exists",
            Error::MusicFileNotFound(_) => "Music file not found",
            Error::UnsupportedFileType(_) => "Unsupported file type",
            Error::AdminNotFound => "Admin not found",
            Error::ChannelNotFound(_) => "Channel not found",
            Error::ChannelDataNotFound(_, _) => "Channel data not found",
            Error::FixtureNotFound(_) => "Fixture not found",
            Error::LayoutNotFound(_) => "Layout not found",
            Error::ProjectNotFound(_) => "Project not found",
            Error::SequenceNotFound(_) => "Sequence not found",
            Error::UserNotFound => "User not found",
            Error::UnauthorizedAction => "Unauthorized action",
            Error::TodoErr => "Todo",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
           Error::Io(ref err) => Some(err),
           Error::JsonDecode(ref err) => Some(err),
           Error::JsonEncode(ref err) => Some(err),
           Error::JsonParse(ref err) => Some(err),
           Error::Ssl(ref err) => Some(err),
           Error::Rsfml(_) => None,
           Error::Postgres(ref err) => Some(err),
           Error::PostgresConnection(ref err) => Some(err),
           Error::FolderNotEmpty(_, _) => None,
           Error::InvalidFileName => None,
           Error::InvalidFrameDuration(_) => None,
           Error::InvalidLayout(_) => None,
           Error::InvalidNumResults(_) => None,
           Error::InvalidPatch(_) => None,
           Error::InvalidPermissionName(_) => None,
           Error::InvalidProjectName(_) => None,
           Error::InvalidPublicKey(_) => None,
           Error::InvalidSequenceName(_) => None,
           Error::InvalidSequenceSection(_) => None,
           Error::InvalidVixenData(_) => None,
           Error::LoadProjectError => None,
           Error::MissingPermissionArg => None,
           Error::OffsetOutOfBounds(_, _) => None,
           Error::DuplicateUser(_, _) => None,
           Error::DuplicateSequence(_) => None,
           Error::MusicFileNotFound(_) => None,
           Error::UnsupportedFileType(_) => None,
           Error::AdminNotFound => None,
           Error::ChannelNotFound(_) => None,
           Error::ChannelDataNotFound(_, _) => None,
           Error::FixtureNotFound(_) => None,
           Error::LayoutNotFound(_) => None,
           Error::ProjectNotFound(_) => None,
           Error::SequenceNotFound(_) => None,
           Error::UserNotFound => None,
           Error::UnauthorizedAction => None,
           Error::TodoErr => None,
       }
   }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f,
                "IO error occurred: {}", error::Error::description(err)),
            Error::JsonDecode(ref err) => write!(f,
                "Json decoding error occurred: {}", err),
            Error::JsonEncode(ref err) => write!(f,
                "Json encoding error occurred: {}", err),
            Error::JsonParse(ref err) => write!(f,
                "Json parsing error occurred: {}", err),
            Error::Ssl(ref err) => write!(f,
                "SSL error occured: {}", error::Error::description(err)),
            Error::Rsfml(ref description) => write!(f, 
                "Rsfml error: {}", description),
            Error::Postgres(ref err) => write!(f, 
                "Postgress error occured: {}", err),
            Error::PostgresConnection(ref err) => write!(f, 
                "Postgress connection error occured: {}", err),
            Error::FolderNotEmpty(ref root, count) => write!(f,
                "{} was not empty: {} files exist", root, count),
            Error::InvalidFileName => write!(f,
                "File name provided is invalid and cannot be retrieved"),
            Error::InvalidFrameDuration(ref duration) => write!(f,
                "Invalid frame duration: {}", duration),
            Error::InvalidLayout(ref description) => write!(f,
                "The layout being read or decoded is invalid: {}", description),
            Error::InvalidNumResults(ref num_results) => write!(f,
                "Invalid number of results returned: {}", num_results),
            Error::InvalidPatch(ref description) => write!(f,
                "Invalid patch file: {}", description),
            Error::InvalidPermissionName(ref name) => write!(f,
                "Invalid permission name provided: {}", name),
            Error::InvalidProjectName(ref name) => write!(f,
                "Invalid project name provided: {}", name),
            Error::InvalidPublicKey(ref key) => write!(f, 
                "Public key is invalid: {}", key),
            Error::InvalidSequenceName(ref seq_name) => write!(f,
                "Sequence name had invalid characters: {}", seq_name),
            Error::InvalidVixenData(ref details) => write!(f,
                "Invalid Vixen data provided: {}", details),
            Error::InvalidSequenceSection(ref section) => write!(f,
                "Invalid sequence section: {}", section),
            Error::LoadProjectError => write!(f, "Loading project failed"),
            Error::MissingPermissionArg => write!(f,
              "Permission argument required but missing (did you forget seqid or secid?)"),
            Error::OffsetOutOfBounds(ref offset, ref upper_bound) => write!(f,
              "Offset {} not between 0 and {} (inclusive)", offset, upper_bound),
            Error::DuplicateUser(ref key, ref user) => write!(f,
                "Duplicate user '{}' or key '{}'", user, key),
            Error::DuplicateSequence(ref name) => write!(f,
                "Duplicate sequence with name '{}'", name),
            Error::MusicFileNotFound(ref path) => write!(f,
                "Music file not found at path '{}'", path),
            Error::UnsupportedFileType(ref file_type) => write!(f, 
                "Unsupported file type: {}", file_type),
            Error::AdminNotFound => write!(f, "Admin not found"),
            Error::ChannelNotFound(ref chanid) => write!(f,
                "Channel not found: {}", chanid),
            Error::ChannelDataNotFound(ref seqid, ref chanid) => write!(f,
                "Channel data not found. seqid: {}, chanid: {}", seqid, chanid),
            Error::FixtureNotFound(ref fix_id) => write!(f,
                "Fixture not found: {}", fix_id),
            Error::LayoutNotFound(ref layout_id) => write!(f,
                "Layout not found: {}", layout_id),
            Error::ProjectNotFound(ref proj_name) => write!(f,
                "Project not found: {}", proj_name),
            Error::SequenceNotFound(ref name) => write!(f,
                "Sequence not found: '{}'", name),
            Error::UserNotFound => write!(f, "User not found"),
            Error::UnauthorizedAction => write!(f, "Unauthorized action"),
            Error::TodoErr => write!(f, "TodoErr"),
        }
    }
}
