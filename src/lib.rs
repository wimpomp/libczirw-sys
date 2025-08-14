extern crate link_cplusplus;

mod functions;
mod handle;
mod interop;
mod misc;
pub mod sys;

pub use functions::*;
pub use handle::*;
pub use interop::*;
pub use misc::{Dimension, LibCZIApiError, PixelType, RawDataType};

#[cfg(test)]
mod tests {
    use crate::handle::{CziReader, InputStream};
    use crate::interop::{LibCZIBuildInformation, ReaderOpenInfo};
    use crate::misc::Dimension;
    use anyhow::{Error, Result};
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_read_shape() -> Result<()> {
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
        println!("pyramid statistics: {:?}", czi.get_pyramid_statistics()?);
        println!("file header info: {:?}", czi.get_file_header_info()?);
        let statistics_simple = czi.get_statistics_simple()?;
        println!("statistics simple: {:?}", czi.get_statistics_simple()?);
        let bounding_box = statistics_simple.get_bounding_box();
        let dim_bounds = statistics_simple.get_dim_bounds();
        let dimensions = Dimension::vec_from_bitflags(dim_bounds.get_dimensions_valid());
        let size = dim_bounds.get_size();
        for (i, d) in dimensions.iter().enumerate() {
            println!("{:?}: {}", d, size[i]);
        }
        println!("X: {}", bounding_box.get_w());
        println!("Y: {}", bounding_box.get_h());
        Ok(())
    }

    #[test]
    fn test_read_bytes() -> Result<()> {
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
        let sub_block = czi.read_sub_block(0)?;
        let bitmap = sub_block.create_bitmap()?.lock()?;
        let bitmap_info = bitmap.get_info()?;
        println!(
            "height: {}, width: {} pixel type: {:#?}",
            bitmap_info.get_height(),
            bitmap_info.get_width(),
            bitmap_info.get_pixel_type()?
        );
        let bytes = bitmap.lock_info.get_data_roi();
        println!("bytes: {:?}", bytes.as_slice()[..100].to_vec());
        Ok(())
    }

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
