use quick_xml::Reader;
use quick_xml::events::Event;
use std::path::PathBuf;

pub enum SHIQType {
    Bottom,
    Top,
    BaseConcept,
    BaseRole,
    InverseRole,
    NegatedConcept,
    IntersectionConcept,
    UnionConcept,
    ExistsConcept,
    ForAllConcept,
    LessThanConcept,
    GreaterThanConcept,
}

pub enum SHIQClass {
    Concept,
    Role,
}

pub enum XmlTag {
    // RDF Schema Features
    Class,
    subClassOf,
    rdfsSubClassOf,
    rdfProperty,
    rdfsSubPropertyOf,
    rdfsDomain,
    rdfsRange,
    Individual,
    // (In)Equality
    equivalentClass,
    equivalentProperty,
    sameAs,
    differentFrom,
    AllDifferent,
    distinctMembers,
    // Property Characteristics
    ObjectProperty,
    DatatypeProperty,
    inverseOf,
    TransitiveProperty,
    FunctionalProperty,
    InverseFunctionalProperty,
    // Property Restrictions
    Restriction,
    onProperty,
    allValuesFrom,
    someValuesFrom,
    // Restricted Cardinality
    minCardinality,
    maxCardinality,
    cardinality,
    // Header Information
    Ontology,
    imports,
    // Class intersection
    intersectionOf,
    // Datatypes
    xsdDatatypes,
    // Versioning
    versionInfo,
    priorVersion,
    backwardCompatibleWith,
    incompatibleWith,
    DeprecatedClass,
    DeprecatedProperty,
    // Annotation Properties,
    rdfsLabel,
    rdfsComment,
    rdfsSeeAlso,
    rdfsIsDefinedBy,
    AnnotationProperty,
    OntologyProperty,
}

// test function to see how quick-xml works
pub fn read_from_filename(p: &PathBuf) {
    let mut reader_op = Reader::from_file(p);

    // let mut txt = Vec::new();
    let mut buf = Vec::new();

    match reader_op {
        Err(e) => { println!("couldn't read the file: {}", &e); },
        Ok(mut reader) => {
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => {
                        match e.name() {
                            _ => {
                                let ename = e.name();
                                let ename = std::str::from_utf8(ename).unwrap();

                                let attr = e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>();
                                let attr_name = e.attributes().map(|a| a.unwrap().key).collect::<Vec<_>>();

                                let mut attr_name_string = Vec::new();
                                let mut attr_string = Vec::new();

                                for a in attr_name {
                                    let s = String::from(std::str::from_utf8(&a).unwrap());
                                    attr_name_string.push(s);
                                }

                                for a in attr {
                                    let s = String::from(std::str::from_utf8(&a).unwrap());
                                    attr_string.push(s);
                                }

                                println!(" -- opening tag: {}", ename);
                                println!(" -- with attributes");
                                for i in 0..(attr_name_string.len()) {
                                    println!("    {}: {}", attr_name_string.get(i).unwrap(), attr_string.get(i).unwrap());
                                }

                            },
                        }
                    },
                    Ok(Event::End(ref e)) => {
                        let name = std::str::from_utf8(e.name()).unwrap();
                        println!(" -- closing tag: {}", name);
                    },
                    Ok(Event::Empty(ref e)) => {
                        let tag = std::str::from_utf8(e.name()).unwrap();
                        let attr = e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>();
                        let attr_name = e.attributes().map(|a| a.unwrap().key).collect::<Vec<_>>();

                        let mut attr_name_string = Vec::new();
                        let mut attr_string = Vec::new();

                        for a in attr_name {
                            let s = String::from(std::str::from_utf8(&a).unwrap());
                            attr_name_string.push(s);
                        }

                        for a in attr {
                            let s = String::from(std::str::from_utf8(&a).unwrap());
                            attr_string.push(s);
                        }

                        println!(" -- opening tag: {}", tag);
                        println!(" -- with attributes");
                        for i in 0..(attr_name_string.len()) {
                            println!("    {}: {}", attr_name_string.get(i).unwrap(), attr_string.get(i).unwrap());
                        }

                    },
                    Ok(Event::Text(e)) => {
                        let ee = std::str::from_utf8(&e).unwrap().trim();
                        if !(ee == "") {
                            println!(" -- it is text this time: {}", ee);
                        }
                    },
                    Ok(Event::Comment(ref e)) => {},
                    Ok(Event::CData(ref e)) => {},
                    Ok(Event::Decl(ref e)) => {},
                    Ok(Event::PI(ref e)) => {},
                    Ok(Event::DocType(ref e)) => {},
                    Ok(Event::Eof) => {
                        println!(" -- end of the line (for you pal)");
                        break;
                    },
                    Err(e) => println!(" -- error here in event reader: {}", &e),
                }
            }
        }
    }
}









