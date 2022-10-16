//! # All Zotero data structures and their builders
//!
//! This structs are used for serialize and deserialize zotero Item's data.
//! This structs can also be used to create or update zotero Item's data.
//!
//! ```rust
//! use zotero::data_structure::item::{Creator, CreatorBuilder, BookData, BookDataBuilder};
//!
//! let creators : Vec<Creator> = vec![
//!     CreatorBuilder::default()
//!         .creator_type("author")
//!         .first_name("John")
//!         .last_name("Doe")
//!         .build()
//!         .unwrap()
//! ];
//!
//! let new_book : BookData = BookDataBuilder::default()
//!       .title("Book title")
//!       .creators(creators)
//!       .item_type("book")
//!       .build()
//!       .unwrap();
//! ```

mod item_data;
use serde::Deserializer;
use serde_json::Value;
pub use item_data::ArtworkData;
pub use item_data::ArtworkDataBuilder;
pub use item_data::AttachmentData;
pub use item_data::AttachmentDataBuilder;
pub use item_data::AudioRecordingData;
pub use item_data::AudioRecordingDataBuilder;
pub use item_data::BillData;
pub use item_data::BillDataBuilder;
pub use item_data::BlogPostData;
pub use item_data::BlogPostDataBuilder;
pub use item_data::BookData;
pub use item_data::BookDataBuilder;
pub use item_data::BookSectionData;
pub use item_data::BookSectionDataBuilder;
pub use item_data::CaseData;
pub use item_data::CaseDataBuilder;
pub use item_data::ComputerProgramData;
pub use item_data::ComputerProgramDataBuilder;
pub use item_data::ConferencePaperData;
pub use item_data::ConferencePaperDataBuilder;
pub use item_data::DictionaryEntryData;
pub use item_data::DictionaryEntryDataBuilder;
pub use item_data::DocumentData;
pub use item_data::DocumentDataBuilder;
pub use item_data::EmailData;
pub use item_data::EmailDataBuilder;
pub use item_data::EncyclopediaArticleData;
pub use item_data::EncyclopediaArticleDataBuilder;
pub use item_data::FilmData;
pub use item_data::FilmDataBuilder;
pub use item_data::ForumPostData;
pub use item_data::ForumPostDataBuilder;
pub use item_data::HearingData;
pub use item_data::HearingDataBuilder;
pub use item_data::InstantMessageData;
pub use item_data::InstantMessageDataBuilder;
pub use item_data::InterviewData;
pub use item_data::InterviewDataBuilder;
pub use item_data::JournalArticleData;
pub use item_data::JournalArticleDataBuilder;
pub use item_data::LetterData;
pub use item_data::LetterDataBuilder;
pub use item_data::MagazineArticleData;
pub use item_data::MagazineArticleDataBuilder;
pub use item_data::ManuscriptData;
pub use item_data::ManuscriptDataBuilder;
pub use item_data::MapData;
pub use item_data::MapDataBuilder;
pub use item_data::NewspaperArticleData;
pub use item_data::NewspaperArticleDataBuilder;
pub use item_data::NoteData;
pub use item_data::NoteDataBuilder;
pub use item_data::PatentData;
pub use item_data::PatentDataBuilder;
pub use item_data::PodcastData;
pub use item_data::PodcastDataBuilder;
pub use item_data::PresentationData;
pub use item_data::PresentationDataBuilder;
pub use item_data::RadioBroadcastData;
pub use item_data::RadioBroadcastDataBuilder;
pub use item_data::ReportData;
pub use item_data::ReportDataBuilder;
pub use item_data::StatuteData;
pub use item_data::StatuteDataBuilder;
pub use item_data::ThesisData;
pub use item_data::ThesisDataBuilder;
pub use item_data::TvBroadcastData;
pub use item_data::TvBroadcastDataBuilder;
pub use item_data::VideoRecordingData;
pub use item_data::VideoRecordingDataBuilder;
pub use item_data::WebpageData;
pub use item_data::WebpageDataBuilder;

use serde::Deserialize;
use serde::Serialize;

use crate::data_structure::shared_fields::{Library, Links, Tagable, Tag};

use derive_builder::Builder;

use zotero_derive::Tagable;

#[derive(Deserialize, Serialize, Debug, Clone, Tagable)]
#[serde(rename_all = "camelCase", tag = "itemType")]
/// An enum that holds structs used to deserialize zotero item data into rust structs.
pub enum ItemType {
    /// A piece of artwork (e.g., an oil painting, photograph, or sculpture). Also use this item type for other types of images or visual items (e.g., scientific figures).
    Artwork(ArtworkData),
    /// Any form of audio recording, including music, spoken word, sound effects, archival recordings, or audio-based scientific figures.
    AudioRecording(AudioRecordingData),
    /// A proposed piece of legislation.
    Bill(BillData),
    /// An article or entry posted to a personal blog website. For online articles published as part of a larger online publication (e.g., NYT BlogsData), using Magazine Article or Newspaper Article generally yields better results
    BlogPost(BlogPostData),
    /// A book or similar published item. For government documents, technical reports, manuals, etc., use Report instead. This item type can also be adapted to fit many types of unusual items.
    Book(BookData),
    /// A section of a book. Usually chapters, but also forewords, prefaces, introductions, appendices, afterwords, comments, etc.
    BookSection(BookSectionData),
    /// A legal case, either published or unpublished.
    Case(CaseData),
    /// A piece of software or other computer program.
    ComputerProgram(ComputerProgramData),
    /// A paper presented at a conference and subsequently published in a formal conference proceedings publication (e.g., as a book, report, or issue of a journal). For conference papers that have not been published in a proceedings, use Presentation.
    ConferencePaper(ConferencePaperData),
    /// An entry published as part of a dictionary.
    DictionaryEntry(DictionaryEntryData),
    /// A generic document item. This item type has a poor selection of fields and poor support in citation styles, so it should generally be avoided.
    Document(DocumentData),
    /// A message sent via email. This type could also be used for other forms of personal communication.
    Email(EmailData),
    /// An article or chapter published as part of an encyclopedia.
    EncyclopediaArticle(EncyclopediaArticleData),
    ///  A film or motion picture. Generally, use this type for artistically-oriented films (including fictional, non-fictional, and documentary films). For other types of video items, use Video Recording.
    Film(FilmData),
    /// A post on an online discussion forum. Also use this type for items such as Facebook posts or tweets.
    ForumPost(ForumPostData),
    /// A formal hearing or meeting report by a legislative body.
    Hearing(HearingData),
    /// A message sent via an instant message or chat service. This type could also be used for other forms of personal communication.
    InstantMessage(InstantMessageData),
    /// An interview with a person, including recordings, transcripts, or other records of the interview.
    Interview(InterviewData),
    /// An article published in a scholarly journal (either print or online).
    JournalArticle(JournalArticleData),
    /// A letter sent between persons or organizations. This type could also be used for other forms of personal communication.
    Letter(LetterData),
    /// An article published in a non-scholarly, popular, or trade magazine (either print or online).
    MagazineArticle(MagazineArticleData),
    /// An unpublished manuscript. Use this type for both historical documents and modern unpublished work (e.g., unpublished manuscripts, manuscripts submitted for publication, working papers that are not widely available). Can also be used for other forms of historical or archival documents. This item type can also be adapted to fit many types of unusual items.
    Manuscript(ManuscriptData),
    /// A map. Also use this type for geographic models.
    Map(MapData),
    /// An article published in a newspaper (either print or online).
    NewspaperArticle(NewspaperArticleData),
    /// A patent awarded for an invention.
    Patent(PatentData),
    /// A podcast (an episode of an audio or video program distributed online, often via subscription).
    Podcast(PodcastData),
    /// A presentation made as part of a conference, meeting, symposium, lecture, etc. This item type refers to the presentation itself, not a written version published as part of a conference proceedings (use Conference Paper for such published versions).
    Presentation(PresentationData),
    /// An audio broadcast, such as a radio news show, an episode of a radio entertainment series, or similar. Includes broadcasts from online radio stations and audio broadcasts archived online (cf. Podcast).
    RadioBroadcast(RadioBroadcastData),
    /// A report published by an organization, institution, government department, or similar entity. Also used for working papers and preprints distributed through institutional repositories or preprint servers. This item type can also be adapted to fit many types of unusual items.
    Report(ReportData),
    /// A law or other piece of enacted legislation.
    Statute(StatuteData),
    /// A thesis submitted as part of a student applying for a degree (either published or unpublished).
    Thesis(ThesisData),
    /// An episode of a television series.
    TvBroadcast(TvBroadcastData),
    /// A video recording. Use this type for general video items that do not fit into one of the more specific video item types (e.g., Film, TV BroadcastData), such as YouTube videos or video-based scientific figures.
    VideoRecording(VideoRecordingData),
    /// An online page of a website. When possible, use one of the more specific item types above (e.g., Magazine Article, Blog Post, Report).
    Webpage(WebpageData),
    /// A standalone attachment file (e.g., a PDF, JPEG, DOCX, PPTX, XLSX, or ODT file). Standalone attachment files have limited functionality in Zotero (e.g., they cannot be properly searched or cited). Always attach files to proper Zotero items.
    Attachment(AttachmentData),
    /// A standalone note. Notes can be used for organizing and annotating in Zotero. If you cite a standalone note, Zotero will use the first 120 characters as the item title (and will treat the note as an author-less and date-less item). Citing notes is not a reliable way to add standalone commentary to a bibliography or reference list.
    Note(NoteData),
}

/// A struct used to represent or deserialize zotero items into rust struct
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub key: String,
    pub version: usize,
    pub library: Library,
    pub links: Links,
    pub meta: ItemMeta,
    pub data: ItemType,
}

#[derive(Deserialize, Serialize, Default, Clone, Debug, Builder, PartialEq)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct Creator {
    #[serde(alias = "creatorType")]
    pub creator_type: String,
    #[serde(alias = "firstName")]
    pub first_name: String,
    #[serde(alias = "lastName")]
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct ItemMeta {
    pub creator_summary: Option<String>,
    pub parsed_date: Option<String>,
    #[serde(deserialize_with = "deserialize_meta_num_children", default)]
    pub num_children: Option<SizeOrBool>,
    // The following part concerns collections
    pub num_collections: Option<usize>,
    pub num_items: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SizeOrBool {
    r#Bool(bool),
    Size(usize),
}

impl Default for SizeOrBool {
    fn default() -> SizeOrBool {
        SizeOrBool::Bool(false)
    }
}

/// A custom deserializer that deserialize parent_collection value either in bool or in Size.
fn deserialize_meta_num_children<'de, D>(deserializer: D) -> Result<Option<SizeOrBool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Value = serde::Deserialize::deserialize(deserializer)?;
    if s.is_boolean() {
        match serde_json::from_value::<bool>(s) {
            Err(_) => Ok(None),
            Ok(v) => Ok(Some(SizeOrBool::Bool(v)))
        }
    } else if s.is_number() {
        match serde_json::from_value::<usize>(s) {
            Err(_) => Ok(None),
            Ok(v) => Ok(Some(SizeOrBool::Size(v)))
        }
    } else if s.is_null() {
        Ok(None)
    } else {
        panic!("invalid type")
    }
}


#[cfg(test)]
mod test_item_deserialization {
    use super::*;
    #[test]
    fn item_deserialization() {
        let input = r#"
            {
                "key": "4X5CQGQA",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/4X5CQGQA",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/4X5CQGQA",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "4X5CQGQA",
                    "version": 2444,
                    "itemType": "bookSection",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "bookTitle": "",
                    "series": "",
                    "seriesNumber": "",
                    "volume": "",
                    "numberOfVolumes": "",
                    "edition": "",
                    "place": "",
                    "publisher": "",
                    "date": "",
                    "pages": "",
                    "language": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:23Z",
                    "dateModified": "2019-10-01T21:17:23Z"
                }
            }
        "#;

        serde_json::from_str::<Item>(input).expect("Failed to parse zotero data");
        assert!(true)
    }
    #[test]
    fn items_deserialization() {
        let input = r#"
        [
            {
                "key": "4X5CQGQA",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/4X5CQGQA",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/4X5CQGQA",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "4X5CQGQA",
                    "version": 2444,
                    "itemType": "bookSection",
                    "title": "",
                    "creators": [
                        {
                            "creatorType": "author",
                            "firstName": "John",
                            "lastName": "Doe"
                        }
                    ],
                    "abstractNote": "",
                    "bookTitle": "",
                    "series": "",
                    "seriesNumber": "",
                    "volume": "",
                    "numberOfVolumes": "",
                    "edition": "",
                    "place": "",
                    "publisher": "",
                    "date": "",
                    "pages": "",
                    "language": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:23Z",
                    "dateModified": "2019-10-01T21:17:23Z"
                }
            },
            {
                "key": "4H5SBMDE",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/4H5SBMDE",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/4H5SBMDE",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "4H5SBMDE",
                    "version": 2444,
                    "itemType": "map",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "mapType": "",
                    "scale": "",
                    "seriesTitle": "",
                    "edition": "",
                    "place": "",
                    "publisher": "",
                    "date": "",
                    "language": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:19Z",
                    "dateModified": "2019-10-01T21:17:19Z"
                }
            },
            {
                "key": "SAU7PP79",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/SAU7PP79",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/SAU7PP79",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "SAU7PP79",
                    "version": 2444,
                    "itemType": "patent",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "place": "",
                    "country": "",
                    "assignee": "",
                    "issuingAuthority": "",
                    "patentNumber": "",
                    "filingDate": "",
                    "pages": "",
                    "applicationNumber": "",
                    "priorityNumbers": "",
                    "issueDate": "",
                    "references": "",
                    "legalStatus": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:16Z",
                    "dateModified": "2019-10-01T21:17:16Z"
                }
            },
            {
                "key": "25DYFG56",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/25DYFG56",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/25DYFG56",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "25DYFG56",
                    "version": 2444,
                    "itemType": "blogPost",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "blogTitle": "",
                    "websiteType": "",
                    "date": "",
                    "url": "",
                    "accessDate": "",
                    "language": "",
                    "shortTitle": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:13Z",
                    "dateModified": "2019-10-01T21:17:13Z"
                }
            },
            {
                "key": "U4AP5MUH",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/U4AP5MUH",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/U4AP5MUH",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "U4AP5MUH",
                    "version": 2444,
                    "itemType": "podcast",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "seriesTitle": "",
                    "episodeNumber": "",
                    "audioFileType": "",
                    "runningTime": "",
                    "url": "",
                    "accessDate": "",
                    "language": "",
                    "shortTitle": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:10Z",
                    "dateModified": "2019-10-01T21:17:10Z"
                }
            },
            {
                "key": "SD5EWBBC",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/SD5EWBBC",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/SD5EWBBC",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "SD5EWBBC",
                    "version": 2444,
                    "itemType": "journalArticle",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "publicationTitle": "",
                    "volume": "",
                    "issue": "",
                    "pages": "",
                    "date": "",
                    "series": "",
                    "seriesTitle": "",
                    "seriesText": "",
                    "journalAbbreviation": "",
                    "language": "",
                    "DOI": "",
                    "ISSN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:16:58Z",
                    "dateModified": "2019-10-01T21:16:58Z"
                }
            },
            {
                "key": "9XZ2W4RU",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/9XZ2W4RU",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/9XZ2W4RU",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "9XZ2W4RU",
                    "version": 2444,
                    "itemType": "magazineArticle",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "publicationTitle": "",
                    "volume": "",
                    "issue": "",
                    "date": "",
                    "pages": "",
                    "language": "",
                    "ISSN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:16:56Z",
                    "dateModified": "2019-10-01T21:16:56Z"
                }
            },
            {
                "key": "EFZTBRBT",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/EFZTBRBT",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/EFZTBRBT",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "EFZTBRBT",
                    "version": 2444,
                    "itemType": "newspaperArticle",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "publicationTitle": "",
                    "place": "",
                    "edition": "",
                    "date": "",
                    "section": "",
                    "pages": "",
                    "language": "",
                    "shortTitle": "",
                    "ISSN": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:16:53Z",
                    "dateModified": "2019-10-01T21:16:53Z"
                }
            },
            {
                "key": "5G7FQMJH",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/5G7FQMJH",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/5G7FQMJH",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "5G7FQMJH",
                    "version": 2444,
                    "itemType": "conferencePaper",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "date": "",
                    "proceedingsTitle": "",
                    "conferenceName": "",
                    "place": "",
                    "publisher": "",
                    "volume": "",
                    "pages": "",
                    "series": "",
                    "language": "",
                    "DOI": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:16:50Z",
                    "dateModified": "2019-10-01T21:16:50Z"
                }
            },
            {
                "key": "48UAQNNN",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/48UAQNNN",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/48UAQNNN",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "48UAQNNN",
                    "version": 2444,
                    "itemType": "encyclopediaArticle",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "encyclopediaTitle": "",
                    "series": "",
                    "seriesNumber": "",
                    "volume": "",
                    "numberOfVolumes": "",
                    "edition": "",
                    "place": "",
                    "publisher": "",
                    "date": "",
                    "pages": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "language": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:16:45Z",
                    "dateModified": "2019-10-01T21:16:45Z"
                }
            },
            {
                "key": "5I47RLH3",
                "version": 2438,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/5I47RLH3",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/5I47RLH3",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "5I47RLH3",
                    "version": 2438,
                    "itemType": "videoRecording",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "videoRecordingFormat": "",
                    "seriesTitle": "",
                    "volume": "",
                    "numberOfVolumes": "",
                    "place": "",
                    "studio": "",
                    "date": "",
                    "runningTime": "",
                    "language": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:36:01Z",
                    "dateModified": "2019-10-01T13:36:01Z"
                }
            },
            {
                "key": "T9GKB3IR",
                "version": 2438,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/T9GKB3IR",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/T9GKB3IR",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "T9GKB3IR",
                    "version": 2438,
                    "itemType": "radioBroadcast",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "programTitle": "",
                    "episodeNumber": "",
                    "audioRecordingFormat": "",
                    "place": "",
                    "network": "",
                    "date": "",
                    "runningTime": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:35:16Z",
                    "dateModified": "2019-10-01T13:35:16Z"
                }
            },
            {
                "key": "QPDKPRGL",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/QPDKPRGL",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/QPDKPRGL",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "QPDKPRGL",
                    "version": 2437,
                    "itemType": "patent",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "place": "",
                    "country": "",
                    "assignee": "",
                    "issuingAuthority": "",
                    "patentNumber": "",
                    "filingDate": "",
                    "pages": "",
                    "applicationNumber": "",
                    "priorityNumbers": "",
                    "issueDate": "",
                    "references": "",
                    "legalStatus": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:35:07Z",
                    "dateModified": "2019-10-01T13:35:07Z"
                }
            },
            {
                "key": "MD62TVCE",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/MD62TVCE",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/MD62TVCE",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "MD62TVCE",
                    "version": 2437,
                    "itemType": "map",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "mapType": "",
                    "scale": "",
                    "seriesTitle": "",
                    "edition": "",
                    "place": "",
                    "publisher": "",
                    "date": "",
                    "language": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:35:01Z",
                    "dateModified": "2019-10-01T13:35:01Z"
                }
            },
            {
                "key": "L77RMQ8J",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/L77RMQ8J",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/L77RMQ8J",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "L77RMQ8J",
                    "version": 2437,
                    "itemType": "instantMessage",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "date": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:56Z",
                    "dateModified": "2019-10-01T13:34:56Z"
                }
            },
            {
                "key": "77NPSQY5",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/77NPSQY5",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/77NPSQY5",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "77NPSQY5",
                    "version": 2437,
                    "itemType": "hearing",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "committee": "",
                    "place": "",
                    "publisher": "",
                    "numberOfVolumes": "",
                    "documentNumber": "",
                    "pages": "",
                    "legislativeBody": "",
                    "session": "",
                    "history": "",
                    "date": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:50Z",
                    "dateModified": "2019-10-01T13:34:50Z"
                }
            },
            {
                "key": "3L59IWHK",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/3L59IWHK",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/3L59IWHK",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "3L59IWHK",
                    "version": 2437,
                    "itemType": "statute",
                    "nameOfAct": "",
                    "creators": [],
                    "abstractNote": "",
                    "code": "",
                    "codeNumber": "",
                    "publicLawNumber": "",
                    "dateEnacted": "",
                    "pages": "",
                    "section": "",
                    "session": "",
                    "history": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:43Z",
                    "dateModified": "2019-10-01T13:34:43Z"
                }
            },
            {
                "key": "QFRII6XJ",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/QFRII6XJ",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/QFRII6XJ",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "QFRII6XJ",
                    "version": 2437,
                    "itemType": "case",
                    "caseName": "",
                    "creators": [],
                    "abstractNote": "",
                    "reporter": "",
                    "reporterVolume": "",
                    "court": "",
                    "docketNumber": "",
                    "firstPage": "",
                    "history": "",
                    "dateDecided": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:40Z",
                    "dateModified": "2019-10-01T13:34:40Z"
                }
            },
            {
                "key": "K36LXQI5",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/1000000/items/K36LXQI5",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/K36LXQI5",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "K36LXQI5",
                    "version": 2437,
                    "itemType": "bill",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "billNumber": "",
                    "code": "",
                    "codeVolume": "",
                    "section": "",
                    "codePages": "",
                    "legislativeBody": "",
                    "session": "",
                    "history": "",
                    "date": "",
                    "language": "",
                    "url": "",
                    "accessDate": "",
                    "shortTitle": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:30Z",
                    "dateModified": "2019-10-01T13:34:30Z"
                }
            }
        ]
        "#;
        serde_json::from_str::<Vec<Item>>(input).expect("Failed to parse zotero data");
        assert!(true);
    }

    #[test]
    fn test_deserialize_creators() {
        let expected_output = Creator {
            creator_type: "author".into(),
            first_name: "John".into(),
            last_name: "Doe".into(),
        };

        let input = r#"
        {
            "creatorType": "author",
            "firstName": "John",
            "lastName": "Doe"
        }
        "#;

        let result = serde_json::from_str::<Creator>(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_item_meta_deserialization() {
        let expected_output = ItemMeta {
            creator_summary: Some("Lorem".into()),
            parsed_date: Some("25-2-2019".into()),
            num_children: Some(SizeOrBool::Size(1)),
            num_collections: Some(1 as usize),
            num_items: Some(0 as usize),
        };

        let input = r#"
            {
                "creatorSummary" : "Lorem",
                "parsedDate": "25-2-2019",
                "numChildren": 1,
                "numCollections": 1,
                "numItems": 0
            }
        "#;

        let result = serde_json::from_str::<ItemMeta>(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_item_deserialization() {
        let input = r#"
            {
                "key": "K36LXQI5",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/1000000/items/K36LXQI5",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/K36LXQI5",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "K36LXQI5",
                    "version": 2437,
                    "itemType": "bill",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "billNumber": "",
                    "code": "",
                    "codeVolume": "",
                    "section": "",
                    "codePages": "",
                    "legislativeBody": "",
                    "session": "",
                    "history": "",
                    "date": "",
                    "language": "",
                    "url": "",
                    "accessDate": "",
                    "shortTitle": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:30Z",
                    "dateModified": "2019-10-01T13:34:30Z"
                }
            }
        "#;

        serde_json::from_str::<Item>(input).expect("Failed to deserialize item");
        assert!(true);
    }
}
