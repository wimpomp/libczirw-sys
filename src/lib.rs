extern crate link_cplusplus;

mod functions;
mod handle;
mod interop;
mod misc;
pub mod sys;

pub use functions::*;
pub use handle::*;
pub use interop::*;
pub use misc::{LibCZIApiError, PixelType, RawDataType};

#[cfg(test)]
mod tests {
    use crate::handle::{CziReader, InputStream};
    use crate::interop::{LibCZIBuildInformation, ReaderOpenInfo};
    use anyhow::{Error, Result};
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_libczi_xml() -> Result<()> {
        let path = env::home_dir()
            .unwrap()
            .join("code/rust/ndbioimage/tests/files/Experiment-2029.czi");
        assert!(path.exists());
        let czi = CziReader::create()?;
        let stream = InputStream::create_from_file_utf8(
            path.to_str().ok_or(Error::msg("cannot into str"))?,
        )?;
        let open_info = ReaderOpenInfo::new(&stream);
        czi.open(open_info)?;
        let metadata_segment = czi.get_metadata_segment()?;
        let xml = metadata_segment.get_metadata_as_xml()?;
        let s = String::try_from(&xml)?;
        println!("xml: {}", &s[..s.len().min(100)]);
        Ok(())
    }

    #[test]
    fn test_libczi_pyramid_statistics() -> Result<()> {
        let path = PathBuf::from("test-files/Experiment-2029.czi");
        assert!(path.exists());
        let czi = CziReader::create()?;
        let stream = InputStream::create_from_file_utf8(
            path.to_str().ok_or(Error::msg("cannot into str"))?,
        )?;
        let open_info = ReaderOpenInfo::new(&stream);
        czi.open(open_info)?;
        let s = czi.get_pyramid_statistics()?;
        println!("xml: {}", &s[..s.len().min(100)]);
        Ok(())
    }

    #[test]
    fn test_libczi_document_info() -> Result<()> {
        let path = PathBuf::from("test-files/Experiment-2029.czi");
        assert!(path.exists());
        let czi = CziReader::create()?;
        let stream = InputStream::create_from_file_utf8(
            path.to_str().ok_or(Error::msg("cannot into str"))?,
        )?;
        let open_info = ReaderOpenInfo::new(&stream);
        czi.open(open_info)?;
        let metadata_segment = czi.get_metadata_segment()?;
        let document_info = metadata_segment.get_czi_document_info()?;
        let general_document_info = document_info.get_general_document_info()?;
        println!(
            "xml: {}",
            &general_document_info[..general_document_info.len().min(100)]
        );
        Ok(())
    }

    #[test]
    fn test_lib_czi_build_information() -> Result<()> {
        let build_info = LibCZIBuildInformation::get()?;
        println!(
            "compiler information: {:?}",
            build_info.get_compiler_information()
        );
        println!("repository url: {:?}", build_info.get_repository_url());
        println!(
            "repository branch: {:?}",
            build_info.get_repository_branch()
        );
        println!("repository tag: {:?}", build_info.get_repository_tag());
        Ok(())
    }
}
