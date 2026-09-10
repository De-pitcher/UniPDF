use crate::error::{ConversionError, Result};
use lopdf::{Document, Object, ObjectId};
use std::collections::BTreeMap;
use std::path::Path;

/// Merge multiple PDF files into a single destination PDF
pub fn merge_pdfs(input_paths: &[&Path], output_path: &Path) -> Result<()> {
    if input_paths.is_empty() {
        return Err(ConversionError::PdfGeneration("No PDF files provided to merge".into()));
    }

    let mut max_id = 1;
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut document = Document::with_version("1.5");

    for (doc_index, path) in input_paths.iter().enumerate() {
        let mut doc = Document::load(path).map_err(|e| {
            ConversionError::FileRead {
                path: path.to_path_buf(),
                source: std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{:?}", e)),
            }
        })?;

        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;

        documents_pages.extend(
            doc.get_pages()
                .into_iter()
                .map(|(_, object_id)| (doc_index, object_id)),
        );
        documents_objects.extend(doc.objects);
    }

    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Process all collected objects
    for (object_id, object) in documents_objects {
        match object.type_name().unwrap_or("") {
            "Catalog" => {
                if catalog_object.is_none() {
                    catalog_object = Some((object_id, object));
                }
            }
            "Pages" => {
                if pages_object.is_none() {
                    pages_object = Some((object_id, object));
                }
            }
            "Page" => {
                document.objects.insert(object_id, object);
            }
            "Outlines" | "Outline" => {}
            _ => {
                document.objects.insert(object_id, object);
            }
        }
    }

    let (pages_object_id, mut pages_dict) = if let Some((id, Object::Dictionary(dict))) = pages_object {
        (id, dict)
    } else {
        let id = document.new_object_id();
        let mut dict = lopdf::Dictionary::new();
        dict.set("Type", Object::Name(b"Pages".to_vec()));
        (id, dict)
    };

    let page_ids: Vec<Object> = documents_pages
        .into_iter()
        .map(|(_, id)| Object::Reference(id))
        .collect();

    pages_dict.set("Count", Object::Integer(page_ids.len() as i64));
    pages_dict.set("Kids", Object::Array(page_ids));
    document.objects.insert(pages_object_id, Object::Dictionary(pages_dict));

    let catalog_object_id = if let Some((id, Object::Dictionary(mut dict))) = catalog_object {
        dict.set("Pages", Object::Reference(pages_object_id));
        document.objects.insert(id, Object::Dictionary(dict));
        id
    } else {
        let id = document.new_object_id();
        let mut dict = lopdf::Dictionary::new();
        dict.set("Type", Object::Name(b"Catalog".to_vec()));
        dict.set("Pages", Object::Reference(pages_object_id));
        document.objects.insert(id, Object::Dictionary(dict));
        id
    };

    document.trailer.set("Root", Object::Reference(catalog_object_id));
    document.max_id = document.objects.len() as u32;
    document.renumber_objects();
    document.compress();

    let mut file = std::fs::File::create(output_path).map_err(|e| ConversionError::FileWrite {
        path: output_path.to_path_buf(),
        source: e,
    })?;

    document.save_to(&mut file).map_err(|e| {
        ConversionError::PdfGeneration(format!("Failed to save merged PDF: {:?}", e))
    })?;

    Ok(())
}
