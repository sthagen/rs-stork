use super::{ReadResult, ReaderConfig, WordListGenerationError};

#[cfg(not(feature = "web-scraping"))]
pub(crate) fn read(
    _url: &str,
    _config: &ReaderConfig,
) -> Result<ReadResult, WordListGenerationError> {
    Err(WordListGenerationError::UnknownContentType)
}

#[cfg(feature = "web-scraping")]
pub(crate) fn read(
    url: &str,
    config: &ReaderConfig,
) -> Result<ReadResult, WordListGenerationError> {
    use crate::config::Filetype;
    use mime::Mime;
    use std::io::Read;

    fn filetype_from_mime(mime: &Mime) -> Option<Filetype> {
        use crate::config::Filetype;
        match (mime.type_(), mime.subtype()) {
            (mime::TEXT, mime::PLAIN) => Some(Filetype::PlainText),
            (mime::TEXT, mime::HTML) => Some(Filetype::HTML),
            _ => None,
        }
    }

    let mut resp =
        reqwest::blocking::get(url).map_err(|_| WordListGenerationError::WebPageNotFetched)?;

    let _status = resp
        .error_for_status_ref()
        .map_err(|_| WordListGenerationError::WebPageNotFetched)?;

    let mime_type: Mime = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .ok_or(WordListGenerationError::UnknownContentType)?
        .to_str()
        .map_err(|_| WordListGenerationError::UnknownContentType)?
        .parse()
        .map_err(|_| WordListGenerationError::UnknownContentType)?;

    let mut buffer = String::new();
    let _bytes_read = resp.read_to_string(&mut buffer);

    Ok(ReadResult {
        buffer,
        filetype: config
            .file
            .filetype
            .clone()
            .or(filetype_from_mime(&mime_type)),
        frontmatter_fields: None,
    })
}
