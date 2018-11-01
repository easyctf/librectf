use std::fs::copy;
use std::io::Write;
use std::path::PathBuf;

use actix_web::{
    dev::Payload,
    error::{ErrorInternalServerError, MultipartError, PayloadError},
    multipart::{Field, MultipartItem},
};
use futures::{future, Future, Stream};
use sha2::{Digest, Sha256};

use tempfile::NamedTempFile;

fn save_file(
    private: bool,
    storage_dir: PathBuf,
    field: Field<Payload>,
) -> Box<Future<Item = String, Error = actix_web::Error>> {
    let mut file = NamedTempFile::new().unwrap();
    let temp_path = file.path().to_path_buf();

    Box::new(
        field
            .fold(Sha256::new(), move |mut hasher, bytes| {
                let result = file
                    .write_all(bytes.as_ref())
                    .map(|_| {
                        hasher.input(bytes.as_ref());
                        hasher
                    }).map_err(|err| {
                        error!("Failed to write to file: {:?}", err);
                        MultipartError::Payload(PayloadError::Io(err))
                    });;
                future::result(result)
            }).map_err(|err| {
                error!("Multipart error: {:?}", err);
                ErrorInternalServerError(err)
            }).map(move |hasher| {
                let hash = format!("{:x}", hasher.result());
                let target_path = storage_dir
                    .join(if private { "private" } else { "public" })
                    .join(&hash);
                copy(temp_path, target_path).unwrap();
                hash
            }),
    )
}

pub(crate) fn handle_multipart(
    private: bool,
    storage_dir: PathBuf,
    item: MultipartItem<Payload>,
) -> Box<Stream<Item = String, Error = actix_web::Error>> {
    match item {
        MultipartItem::Field(field) => {
            Box::new(save_file(private, storage_dir, field).into_stream())
        }
        MultipartItem::Nested(nested) => Box::new(
            nested
                .map_err(ErrorInternalServerError)
                .map(move |item| handle_multipart(private, storage_dir.clone(), item))
                .flatten(),
        ),
    }
}
