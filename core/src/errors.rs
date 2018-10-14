use std::io;

error_wrapper!(FileOpenError: io::Error = "Failed to access file");
error_wrapper!(FileReadError: io::Error = "Failed to read file");
error_wrapper!(DirTraversalError: io::Error = "Failed to traverse the directory");

error_derive_from!(Error = {
    ::serde_json::Error["Error during the serialization of JSON"] => JSONSerialization,
    DirTraversalError[""] => DirTraversal,
    FileOpenError[""] => FileOpen,
    FileReadError[""] => FileRead,
});
