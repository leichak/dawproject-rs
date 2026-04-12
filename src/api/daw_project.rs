#![allow(unused)]

use {
    super::{error::DawProjectError, meta_data::MetaData, project::Project},
    quick_xml::{de::from_str, se::to_string},
    std::{
        any::type_name,
        collections::HashMap,
        fs,
        io::{Read, Write},
        path::Path,
        str,
    },
    zip::ZipWriter,
};

const FORMAT_NAME: &str = "DAWProject exchange format";
const FILE_EXTENSION: &str = "dawproject";
const PROJECT_FILE: &str = "project.xml";
const METADATA_FILE: &str = "metadata.xml";

/// Main entry point for reading and writing `.dawproject` files (ZIP archives).
pub struct DawProject {}

pub enum ObjectType {
    P(Box<Project>),
    M(MetaData),
}

impl DawProject {
    fn new() -> Self {
        DawProject {}
    }

    fn to_xml(object: ObjectType) -> Result<String, DawProjectError> {
        match object {
            ObjectType::P(o) => Ok(to_string(&o)?),
            ObjectType::M(o) => Ok(to_string(&o)?),
        }
    }

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    fn project_from_xml(xml_string: String) -> Result<Project, DawProjectError> {
        Ok(from_str(&xml_string)?)
    }

    fn metadata_from_xml(xml_string: String) -> Result<MetaData, DawProjectError> {
        Ok(from_str(&xml_string)?)
    }

    fn save_xml(project: Project, path: &Path) -> Result<(), DawProjectError> {
        let project_xml = Self::to_xml(ObjectType::P(Box::new(project)))?;
        fs::write(path, project_xml)?;
        Ok(())
    }

    fn validate(_project: Project) -> Result<(), DawProjectError> {
        Ok(())
    }

    pub fn save(
        project: Project,
        meta_data: MetaData,
        embedded_files: HashMap<&Path, String>,
        zip_file_path: &Path,
    ) -> Result<(), DawProjectError> {
        let file = std::fs::File::create(zip_file_path)?;
        let mut zip_writer = zip::ZipWriter::new(file);

        let project_xml = Self::to_xml(ObjectType::P(Box::new(project)))?;
        let meta_data_xml = Self::to_xml(ObjectType::M(meta_data))?;

        Self::add_file_to_zip_from_str(&mut zip_writer, &project_xml, PROJECT_FILE)?;
        Self::add_file_to_zip_from_str(&mut zip_writer, &meta_data_xml, METADATA_FILE)?;

        for (p, s) in &embedded_files {
            let file_name_with_path = p
                .to_str()
                .ok_or_else(|| DawProjectError::InvalidPath(format!("{:?}", p)))?;
            Self::add_file_to_zip_from_str(&mut zip_writer, s, file_name_with_path)?;
        }

        zip_writer.finish()?;
        Ok(())
    }

    fn add_bytes_to_zip<W: std::io::Write + std::io::Seek>(
        zip_writer: &mut ZipWriter<W>,
        content: &[u8],
        file_name: &str,
    ) -> Result<(), DawProjectError> {
        let name = format!("./{}", file_name);
        zip_writer.start_file(name, zip::write::SimpleFileOptions::default())?;
        zip_writer.write_all(content)?;
        Ok(())
    }

    fn add_file_to_zip_from_str<W: std::io::Write + std::io::Seek>(
        zip_writer: &mut ZipWriter<W>,
        content: &str,
        file_name: &str,
    ) -> Result<(), DawProjectError> {
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        zip_writer.start_file(file_name, options)?;
        zip_writer.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn load(fname: &Path) -> Result<(Project, MetaData), DawProjectError> {
        let zip_file = std::fs::File::open(fname)?;
        let mut archive = zip::ZipArchive::new(zip_file)?;

        let mut p = String::new();
        let mut m = String::new();

        for i in 0..archive.len() {
            if let Ok(mut file) = archive.by_index(i) {
                let _out_path = match file.enclosed_name() {
                    Some(path) => path.to_owned(),
                    None => continue,
                };

                if file.name() == PROJECT_FILE {
                    file.read_to_string(&mut p)?;
                }

                if file.name() == METADATA_FILE {
                    file.read_to_string(&mut m)?;
                }
            }
        }

        let project: Project = from_str(&p)?;
        let metadata: MetaData = from_str(&m)?;

        Ok((project, metadata))
    }

    fn load_metadata(fname: &Path) -> Result<MetaData, DawProjectError> {
        let zip_file = std::fs::File::open(fname)?;
        let mut archive = zip::ZipArchive::new(zip_file)?;

        let mut contents = String::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let _out_path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if file.name() == METADATA_FILE {
                file.read_to_string(&mut contents)?;
            }
        }

        let metadata: MetaData = from_str(&contents)?;
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::DawProject,
        std::{error::Error, io::Read, str::FromStr},
    };

    #[test]
    fn create_zip_with_text_and_load_test() -> Result<(), Box<dyn Error>> {
        let zip_file_path = "./tests/test.zip";
        let file = std::fs::File::create(zip_file_path)?;

        let mut zip_writer = zip::ZipWriter::new(file);

        let contents = String::from_str("This is just the test").unwrap();
        let filename = "./tests/test.dawproject";

        DawProject::add_file_to_zip_from_str(&mut zip_writer, &contents, filename)?;
        zip_writer.finish()?;

        let zip_file = std::fs::File::open(zip_file_path)?;
        let mut archive = zip::ZipArchive::new(zip_file)?;

        let mut contents_read = String::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let _out_path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if file.name().contains(filename) {
                file.read_to_string(&mut contents_read)?;
                println!("File {}", file.name());
            }
        }

        assert_eq!(contents, contents_read);

        Ok(())
    }
}
