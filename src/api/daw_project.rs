#![allow(unused)]

use {
    super::{meta_data::MetaData, project::Project},
    quick_xml::{de::from_str, se::to_string},
    std::{
        any::type_name,
        collections::HashMap,
        error::Error,
        fs,
        io::{Read, Write},
        path::Path,
        str,
    },
    zip::{write::FileOptions, ZipWriter},
};

const FORMAT_NAME: &str = "DAWProject exchange format";
const FILE_EXTENSION: &str = "dawproject";
const PROJECT_FILE: &str = "project.xml";
const METADATA_FILE: &str = "metadata.xml";

pub struct DawProject {}

pub enum ObjectType {
    P(Project),
    M(MetaData),
}

impl DawProject {
    fn new() -> Self {
        DawProject {}
    }

    fn export_schema() -> Result<String, Box<dyn Error>> {
        /*
        This probably be able to export xml schema.xsd, but it is unnecessary since we will always derive it
        from its parent project in Java. // or not?
         */
        Ok(String::new())
    }

    fn to_xml(object: ObjectType) -> Result<String, Box<dyn Error>> {
        /*
        Function that takes object and returns String from that object that represents XML
         */

        match object {
            ObjectType::P(o) => match to_string(&o) {
                Ok(o_string) => Ok(o_string),
                Err(err) => Err(err.into()),
            },
            ObjectType::M(o) => match to_string(&o) {
                Ok(o_string) => Ok(o_string),
                Err(err) => Err(err.into()),
            },
        }
    }

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    fn project_from_xml(xml_string: String) -> Result<Project, Box<dyn Error>> {
        match from_str(&xml_string) {
            Ok(project) => Ok(project),
            Err(err) => Err(err.into()),
        }
    }

    fn metadata_from_xml(xml_string: String) -> Result<MetaData, Box<dyn Error>> {
        match from_str(&xml_string) {
            Ok(project) => Ok(project),
            Err(err) => Err(err.into()),
        }
    }

    fn create_context() -> Result<(), Box<dyn Error>> {
        /*
        This functions is creating some JABCONTEXT, whatever it is, unnecessary I think
         */
        Ok(())
    }

    fn save_xml(project: Project, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut project_xml: String = String::new();

        match DawProject::to_xml(ObjectType::P(project)) {
            Ok(x) => {
                project_xml = x;
            }
            Err(err) => return Err(err),
        };

        match fs::write(path, project_xml) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    fn validate(_project: Project) -> Result<(), Box<dyn Error>> {
        /*
        This `validate` function takes a `Project` object, converts it to XML using `toXML`,
        then validates the XML against an XML schema for `Project` objects, throwing an `IOException`
        if validation fails due to a JAXB or SAX exception.

        So it is safeguard
        */
        Ok(())
    }

    pub fn save(
        project: Project,
        meta_data: MetaData,
        embedded_files: HashMap<&Path, String>,
        zip_file_path: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let file = match std::fs::File::create(zip_file_path) {
            Ok(f) => f,
            Err(err) => return Err(err.into()),
        };

        let mut zip_writer = zip::ZipWriter::new(file);

        let project_xml = match Self::to_xml(ObjectType::P(project)) {
            Ok(s) => s,
            Err(err) => return Err(err),
        };

        println!("{}", &project_xml);

        let meta_data_xml = match Self::to_xml(ObjectType::M(meta_data)) {
            Ok(s) => s,
            Err(err) => return Err(err),
        };

        match Self::add_file_to_zip_from_str(&mut zip_writer, &project_xml, PROJECT_FILE) {
            Ok(()) => (),
            Err(err) => return Err(err),
        }

        match Self::add_file_to_zip_from_str(&mut zip_writer, &meta_data_xml, METADATA_FILE) {
            Ok(()) => (),
            Err(err) => return Err(err),
        }

        for (p, s) in &embedded_files {
            let file_name_with_path = p.to_str().unwrap();
            match Self::add_file_to_zip_from_str(&mut zip_writer, s, file_name_with_path) {
                Ok(()) => (),
                Err(err) => return Err(err),
            }
        }

        match zip_writer.finish() {
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }

    fn add_bytes_to_zip<W: std::io::Write + std::io::Seek>(
        zip_writer: &mut ZipWriter<W>,
        content: &[u8],
        file_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let name = format!("./{}", file_name);

        match zip_writer.start_file(name, Default::default()) {
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        }

        match zip_writer.write_all(content) {
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }

    fn add_file_to_zip_from_str<W: std::io::Write + std::io::Seek>(
        zip_writer: &mut ZipWriter<W>,
        content: &str,
        file_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let name = file_name.to_string();

        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        match zip_writer.start_file(file_name, options) {
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        }

        match zip_writer.write_all(content.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }

    pub fn strip_bom() {
        /*
        This stripBom function takes an InputStream, detects and removes any Byte Order Marks (BOMs) present in the stream,
        then returns an InputStreamReader using the appropriate Charset,
        defaulting to UTF-8 if no BOM is found or if the BOM is not recognized as UTF-8,
        UTF-16LE, or UTF-16BE. If the BOM is not recognized, it throws an IOException with the message
        "The charset is not supported."
        */
    }

    pub fn load(fname: &Path) -> Result<(Project, MetaData), Box<dyn Error>> {
        let zip_file = std::fs::File::open(fname).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();

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

        let project: Project = match from_str(&p) {
            Ok(p) => p,
            Err(err) => return Err(err.into()),
        };

        let metadata: MetaData = match from_str(&m) {
            Ok(m) => m,
            Err(err) => return Err(err.into()),
        };

        Ok((project, metadata))
    }

    fn load_metadata(fname: &Path) -> Result<MetaData, Box<dyn Error>> {
        let zip_file = std::fs::File::open(fname).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();

        let mut contents = String::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let _out_path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if file.name() == METADATA_FILE {
                match file.read_to_string(&mut contents) {
                    Ok(_v) => (),
                    Err(err) => return Err(err.into()),
                };
            }
        }

        let metadata: MetaData = match from_str(&contents) {
            Ok(p) => p,
            Err(err) => return Err(err.into()),
        };

        Ok(metadata)
    }

    fn stream_embedded(_file: &Path, _embedded_path: String) -> Result<(), Box<dyn Error>> {
        todo!("Implement");
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
        let file = match std::fs::File::create(zip_file_path) {
            Ok(f) => f,
            Err(err) => return Err(err.into()),
        };

        let mut zip_writer = zip::ZipWriter::new(file);

        let contents = String::from_str("This is just the test").unwrap();
        let filename = "./tests/test.dawproject";

        match DawProject::add_file_to_zip_from_str(&mut zip_writer, &contents, filename) {
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        }

        match zip_writer.finish() {
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        }

        let zip_file = std::fs::File::open(zip_file_path).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();

        let mut contents_read = String::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let _out_path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if file.name().contains(filename) {
                match file.read_to_string(&mut contents_read) {
                    Ok(_) => println!("File {}", file.name()),
                    Err(err) => return Err(err.into()),
                };
            }
        }

        assert_eq!(contents, contents_read);

        Ok(())
    }
}
