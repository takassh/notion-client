use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{emoji::Emoji, file::File, parent::Parent, rich_text::RichText, user::User};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Block {
    pub object: String,
    pub id: String,
    pub parent: Parent,
    #[serde(flatten)]
    pub block_type: BlockType,
    pub created_time: DateTime<Utc>,
    pub created_by: User,
    pub last_edited_time: DateTime<Utc>,
    pub last_edited_by: User,
    pub archived: bool,
    pub has_children: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlockType {
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
    Template {
        template: TemplateValue,
    },
    ToDo {
        to_do: ToDoValue,
    },
    ToggleBlocks {
        toggle_blocks: ToggleBlocksValue,
    },
    Video {
        video: VideoValue,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BookmarkValue {
    caption: Vec<RichText>,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BreadcrumpValue {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BulletedListItemValue {
    pub rich_text: Vec<RichText>,
    pub color: TextColor,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CalloutValue {
    rich_text: Vec<RichText>,
    icon: Icon,
    color: TextColor,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildDatabaseValue {
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildPageValue {
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CodeValue {
    caption: Vec<RichText>,
    rich_text: Vec<RichText>,
    language: Language,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ColumnListValue {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ColumnValue {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DividerValue {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct EmbedValue {
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct EquationValue {
    expression: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FileValue {
    caption: Vec<RichText>,
    #[serde(flatten)]
    file_type: File,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct HeadingsValue {
    rich_text: Vec<RichText>,
    color: TextColor,
    is_toggleable: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ImageValue {
    #[serde(flatten)]
    file_type: File,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkPreviewValue {
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct NumberedListItemValue {
    pub rich_text: Vec<RichText>,
    pub color: TextColor,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ParagraphValue {
    pub rich_text: Vec<RichText>,
    pub color: TextColor,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct PdfValue {
    caption: Vec<RichText>,
    #[serde(flatten)]
    file_type: File,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct QuoteValue {
    pub rich_text: Vec<RichText>,
    pub color: TextColor,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SyncedBlockValue {
    pub synced_from: SyncedFrom,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SyncedFrom {
    BlockId { block_id: String },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableValue {
    table_width: u32,
    has_column_header: bool,
    has_row_header: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableRowsValue {
    cells: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableOfContentsValue {
    color: TextColor,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TemplateValue {
    rich_text: Vec<RichText>,
    children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ToDoValue {
    rich_text: Vec<RichText>,
    checked: bool,
    color: TextColor,
    children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ToggleBlocksValue {
    rich_text: Vec<RichText>,
    color: TextColor,
    children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VideoValue {
    #[serde(flatten)]
    file_type: File,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
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
