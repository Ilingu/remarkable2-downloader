use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

pub type RmkFile = (String, String, Vec<u8>);

pub type RmkDocuments = Vec<RmkDocument>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RmkDocument {
    #[serde(rename = "Bookmarked")]
    pub bookmarked: bool,
    #[serde(rename = "CurrentPage")]
    pub current_page: Option<i64>,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "ModifiedClient")]
    pub modified_client: String,
    #[serde(rename = "Parent")]
    pub parent: String,
    #[serde(rename = "Type")]
    pub doc_type: DocType,
    #[serde(rename = "VissibleName")]
    pub vissible_name: String,
    pub c_pages: Option<CPages>,
    pub cover_page_number: Option<i64>,
    pub custom_zoom_center_x: Option<i64>,
    pub custom_zoom_center_y: Option<i64>,
    pub custom_zoom_orientation: Option<String>,
    pub custom_zoom_page_height: Option<i64>,
    pub custom_zoom_page_width: Option<i64>,
    pub custom_zoom_scale: Option<i64>,
    pub document_metadata: Option<DocumentMetadata>,
    pub extra_metadata: Option<ExtraMetadata>,
    pub file_type: Option<String>,
    pub font_name: Option<String>,
    pub format_version: Option<i64>,
    pub line_height: Option<i64>,
    pub margins: Option<i64>,
    pub orientation: Option<String>,
    pub page_count: Option<i64>,
    #[serde(default)]
    pub page_tags: Vec<Value>,
    pub size_in_bytes: Option<String>,
    pub tags: Vec<Value>,
    pub text_alignment: Option<String>,
    pub text_scale: Option<i64>,
    pub zoom_mode: Option<String>,
    pub original_page_count: Option<i64>,
    pub pages: Option<Vec<String>>,
    pub redirection_page_map: Option<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DocType {
    CollectionType,
    DocumentType,
    #[default]
    Unknown,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CPages {
    pub last_opened: Option<LastOpened>,
    pub original: Option<Original>,
    pub pages: Option<Vec<Page>>,
    pub uuids: Option<Vec<Uuid>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastOpened {
    pub timestamp: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original {
    pub timestamp: Option<String>,
    pub value: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    pub idx: Option<Idx>,
    pub redir: Option<Redir>,
    pub scroll_time: Option<ScrollTime>,
    pub vertical_scroll: Option<VerticalScroll>,
    pub template: Option<Template>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Idx {
    pub timestamp: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Redir {
    pub timestamp: Option<String>,
    pub value: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScrollTime {
    pub timestamp: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerticalScroll {
    pub timestamp: Option<String>,
    pub value: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub timestamp: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uuid {
    pub first: Option<String>,
    pub second: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentMetadata {
    #[serde(default)]
    pub authors: Vec<String>,
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraMetadata {
    #[serde(rename = "LastBallpointv2Color")]
    pub last_ballpointv2color: Option<String>,
    #[serde(rename = "LastBallpointv2Size")]
    pub last_ballpointv2size: Option<String>,
    #[serde(rename = "LastCalligraphyColor")]
    pub last_calligraphy_color: Option<String>,
    #[serde(rename = "LastCalligraphySize")]
    pub last_calligraphy_size: Option<String>,
    #[serde(rename = "LastEraseSectionColor")]
    pub last_erase_section_color: Option<String>,
    #[serde(rename = "LastEraseSectionSize")]
    pub last_erase_section_size: Option<String>,
    #[serde(rename = "LastEraserColor")]
    pub last_eraser_color: Option<String>,
    #[serde(rename = "LastEraserSize")]
    pub last_eraser_size: Option<String>,
    #[serde(rename = "LastEraserTool")]
    pub last_eraser_tool: Option<String>,
    #[serde(rename = "LastFinelinerv2Color")]
    pub last_finelinerv2color: Option<String>,
    #[serde(rename = "LastFinelinerv2Size")]
    pub last_finelinerv2size: Option<String>,
    #[serde(rename = "LastHighlighterv2Color")]
    pub last_highlighterv2color: Option<String>,
    #[serde(rename = "LastHighlighterv2Size")]
    pub last_highlighterv2size: Option<String>,
    #[serde(rename = "LastMarkerv2Color")]
    pub last_markerv2color: Option<String>,
    #[serde(rename = "LastMarkerv2Size")]
    pub last_markerv2size: Option<String>,
    #[serde(rename = "LastPaintbrushv2Color")]
    pub last_paintbrushv2color: Option<String>,
    #[serde(rename = "LastPaintbrushv2Size")]
    pub last_paintbrushv2size: Option<String>,
    #[serde(rename = "LastPen")]
    pub last_pen: Option<String>,
    #[serde(rename = "LastPencilv2Color")]
    pub last_pencilv2color: Option<String>,
    #[serde(rename = "LastPencilv2Size")]
    pub last_pencilv2size: Option<String>,
    #[serde(rename = "LastSelectionToolColor")]
    pub last_selection_tool_color: Option<String>,
    #[serde(rename = "LastSelectionToolSize")]
    pub last_selection_tool_size: Option<String>,
    #[serde(rename = "LastSharpPencilv2Color")]
    pub last_sharp_pencilv2color: Option<String>,
    #[serde(rename = "LastSharpPencilv2Size")]
    pub last_sharp_pencilv2size: Option<String>,
    #[serde(rename = "LastTool")]
    pub last_tool: Option<String>,
}
