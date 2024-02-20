use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{emoji::Emoji, file::File, parent::Parent, rich_text::RichText, user::User};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Block {
    pub object: Option<String>,
    pub id: Option<String>,
    pub parent: Option<Parent>,
    #[serde(flatten)]
    pub block_type: BlockType,
    pub created_time: Option<DateTime<Utc>>,
    pub created_by: Option<User>,
    pub last_edited_time: Option<DateTime<Utc>>,
    pub last_edited_by: Option<User>,
    pub archived: Option<bool>,
    pub has_children: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlockType {
    #[default]
    None,
    Bookmark {
        bookmark: BookmarkValue,
    },
    Breadcrumb {
        breadcrump: BreadcrumpValue,
    },
    BulletedListItem {
        bulleted_list_item: BulletedListItemValue,
    },
    Callout {
        callout: CalloutValue,
    },
    ChildDatabase {
        child_database: ChildDatabaseValue,
    },
    ChildPage {
        child_page: ChildPageValue,
    },
    Code {
        code: CodeValue,
    },
    ColumnList {
        column_list: ColumnListValue,
    },
    Column {
        column: ColumnValue,
    },
    Divider {
        divider: DividerValue,
    },
    Embed {
        embed: EmbedValue,
    },
    Equation {
        equation: EquationValue,
    },
    File {
        file: FileValue,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        heading_1: HeadingsValue,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        heading_2: HeadingsValue,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        heading_3: HeadingsValue,
    },
    Image {
        image: ImageValue,
    },
    LinkPreview {
        link_preview: LinkPreviewValue,
    },
    NumberedListItem {
        numbered_list_item: NumberedListItemValue,
    },
    Paragraph {
        paragraph: ParagraphValue,
    },
    Pdf {
        pdf: PdfValue,
    },
    Quote {
        quote: QuoteValue,
    },
    SyncedBlock {
        synced_block: SyncedBlockValue,
    },
    Table {
        table: TableValue,
    },
    TableOfContents {
        table_of_contents: TableOfContentsValue,
    },
    TableRow {
        table_row: TableRowsValue,
    },
    Template {
        template: TemplateValue,
    },
    ToDo {
        to_do: ToDoValue,
    },
    Toggle {
        toggle: ToggleValue,
    },
    Video {
        video: VideoValue,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BookmarkValue {
    pub caption: Vec<RichText>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BreadcrumpValue {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BulletedListItemValue {
    pub rich_text: Vec<RichText>,
    pub color: TextColor,
    pub children: Option<Vec<Block>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CalloutValue {
    pub rich_text: Vec<RichText>,
    pub icon: Icon,
    pub color: TextColor,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildDatabaseValue {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildPageValue {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CodeValue {
    pub caption: Vec<RichText>,
    pub rich_text: Vec<RichText>,
    pub language: Language,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ColumnListValue {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ColumnValue {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DividerValue {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct EmbedValue {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct EquationValue {
    pub expression: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FileValue {
    pub caption: Vec<RichText>,
    #[serde(flatten)]
    pub file_type: File,
    pub name: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct HeadingsValue {
    pub rich_text: Vec<RichText>,
    pub color: Option<TextColor>,
    pub is_toggleable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ImageValue {
    #[serde(flatten)]
    pub file_type: File,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkPreviewValue {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct NumberedListItemValue {
    pub rich_text: Vec<RichText>,
    pub color: TextColor,
    pub children: Option<Vec<Block>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct ParagraphValue {
    pub rich_text: Vec<RichText>,
    pub color: Option<TextColor>,
    pub children: Option<Vec<Block>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct PdfValue {
    pub caption: Vec<RichText>,
    #[serde(flatten)]
    pub file_type: File,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct QuoteValue {
    pub rich_text: Vec<RichText>,
    pub color: TextColor,
    pub children: Option<Vec<Block>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SyncedBlockValue {
    pub synced_from: SyncedFrom,
    pub children: Option<Vec<Block>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SyncedFrom {
    BlockId { block_id: String },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableValue {
    pub table_width: u32,
    pub has_column_header: bool,
    pub has_row_header: bool,
    pub children: Option<Vec<Block>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableRowsValue {
    pub cells: Option<Vec<RichText>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableOfContentsValue {
    pub color: TextColor,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TemplateValue {
    pub rich_text: Vec<RichText>,
    pub children: Option<Vec<Block>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct ToDoValue {
    pub rich_text: Vec<RichText>,
    pub checked: Option<bool>,
    pub color: Option<TextColor>,
    pub children: Option<Vec<Block>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ToggleValue {
    pub rich_text: Vec<RichText>,
    pub color: TextColor,
    pub children: Option<Vec<Block>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VideoValue {
    #[serde(flatten)]
    pub file_type: File,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TextColor {
    Blue,
    BlueBackground,
    Brown,
    BrownBackground,
    Default,
    Gray,
    GrayBackground,
    Green,
    GreenBackground,
    Orange,
    OrangeBackground,
    Yellow,
    YellowBackground,
    Pink,
    PinkBackground,
    Purple,
    PurpleBackground,
    Red,
    RedBackground,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Icon {
    File(File),
    Emoji(Emoji),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Abap,
    Arduino,
    Bash,
    Basic,
    C,
    Clojure,
    Coffeescript,
    #[serde(rename = "c++")]
    CPlusPlus,
    #[serde(rename = "c#")]
    CSharp,
    Css,
    Dart,
    Diff,
    Docker,
    Elixir,
    Elm,
    Erlang,
    Flow,
    Fortran,
    #[serde(rename = "f#")]
    FSharp,
    Gherkin,
    Glsl,
    Go,
    Graphql,
    Groovy,
    Haskell,
    Html,
    Java,
    Javascript,
    Json,
    Julia,
    Kotlin,
    Latex,
    Less,
    Lisp,
    Livescript,
    Lua,
    Makefile,
    Markdown,
    Markup,
    Matlab,
    Mermaid,
    Nix,
    #[serde(rename = "objective-c")]
    ObjectiveC,
    Ocaml,
    Pascal,
    Perl,
    Php,
    #[serde(rename = "plain text")]
    PlainText,
    Powershell,
    Prolog,
    Protobuf,
    Python,
    R,
    Reason,
    Ruby,
    Rust,
    Sass,
    Scala,
    Scheme,
    Scss,
    Shell,
    Sql,
    Swift,
    Typescript,
    #[serde(rename = "vb.net")]
    VbNet,
    Verilog,
    Vhdl,
    #[serde(rename = "visual basic")]
    VisualBasic,
    Webassembly,
    Xml,
    Yaml,
    #[serde(rename = "java/c/c++/c#")]
    JavaOrCOrCPlusPlusOrCSharp,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
struct EmptyObject {}
