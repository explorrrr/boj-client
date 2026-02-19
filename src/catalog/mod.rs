use serde::{Deserialize, Serialize};

/// Immutable BOJ API discovery catalog embedded in this crate.
#[derive(Debug, Clone)]
pub struct CatalogSnapshot {
    /// Source document used to curate this snapshot.
    pub source_document: &'static str,
    /// Source publication date.
    pub source_date: &'static str,
    /// Global parameter notes shared across endpoints.
    pub general_notes: &'static [&'static str],
    /// Supported output format codes.
    pub format_codes: &'static [&'static str],
    /// Supported language codes.
    pub language_codes: &'static [&'static str],
    /// Supported frequency codes for `getDataLayer`.
    pub frequency_codes: &'static [&'static str],
    /// Known DB catalog entries.
    pub databases: &'static [DatabaseEntry],
    /// Parameter catalog matrix for each endpoint.
    pub parameters: &'static [ParameterSpec],
    /// Request limits and overflow behavior notes.
    pub limits: &'static [RequestLimit],
    /// Layer selection rules.
    pub layer_rules: &'static [&'static str],
    /// BOJ STATUS/MESSAGE catalog entries.
    pub messages: &'static [MessageCodeSpec],
}

/// One DB code entry from BOJ appendix A.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseEntry {
    /// Statistics category label in Japanese.
    pub category_ja: &'static str,
    /// BOJ DB code.
    pub code: &'static str,
    /// DB name label in Japanese.
    pub name_ja: &'static str,
}

/// Endpoint requirement level for a parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointRequirement {
    /// Parameter is required.
    Required,
    /// Parameter is optional.
    Optional,
    /// Parameter is unsupported for the endpoint.
    Unsupported,
}

impl EndpointRequirement {
    /// Returns normalized requirement text.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::Unsupported => "unsupported",
        }
    }
}

/// One parameter row in the BOJ parameter matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterSpec {
    /// Parameter name in BOJ manual notation.
    pub name: &'static str,
    /// Parameter purpose summary.
    pub description_ja: &'static str,
    /// Allowed value summary.
    pub allowed_values: &'static str,
    /// Requirement level for `getDataCode`.
    pub code_api: EndpointRequirement,
    /// Requirement level for `getDataLayer`.
    pub layer_api: EndpointRequirement,
    /// Requirement level for `getMetadata`.
    pub metadata_api: EndpointRequirement,
    /// Additional notes for this parameter.
    pub notes: &'static [&'static str],
}

/// One request limit specification from BOJ manual section II.4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestLimit {
    /// API scope for this limit.
    pub api_scope: &'static str,
    /// What this limit counts.
    pub target: &'static str,
    /// Maximum value allowed by BOJ.
    pub max_value: u32,
    /// Behavior when the limit is exceeded.
    pub overflow_behavior: &'static str,
}

/// One BOJ message code row from appendix B.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageCodeSpec {
    /// BOJ STATUS value.
    pub status: u16,
    /// BOJ MESSAGEID value.
    pub message_id: &'static str,
    /// BOJ MESSAGE value.
    pub message: &'static str,
    /// Notes from the appendix table.
    pub note: &'static str,
}

static GENERAL_NOTES: &[&str] = &[
    "以下の文字および全角文字は指定不可: < > ” ! | \\ ; '",
    "パラメータ名および値は大文字小文字を区別しません。",
    "複数パラメータの並び順は順不同です。",
];

static FORMAT_CODES: &[&str] = &["JSON", "CSV"];
static LANGUAGE_CODES: &[&str] = &["JP", "EN"];
static FREQUENCY_CODES: &[&str] = &["CY", "FY", "CH", "FH", "Q", "M", "W", "D"];

static DATABASES: &[DatabaseEntry] = &[
    DatabaseEntry {
        category_ja: "金利（預金・貸出関連）",
        code: "IR01",
        name_ja: "基準割引率および基準貸付利率（従来「公定歩合」として掲載されていたもの）の推移",
    },
    DatabaseEntry {
        category_ja: "金利（預金・貸出関連）",
        code: "IR02",
        name_ja: "預金種類別店頭表示金利の平均年利率等",
    },
    DatabaseEntry {
        category_ja: "金利（預金・貸出関連）",
        code: "IR03",
        name_ja: "定期預金の預入期間別平均金利",
    },
    DatabaseEntry {
        category_ja: "金利（預金・貸出関連）",
        code: "IR04",
        name_ja: "貸出約定平均金利",
    },
    DatabaseEntry {
        category_ja: "マーケット関連",
        code: "FM01",
        name_ja: "無担保コールＯ／Ｎ物レート（毎営業日）",
    },
    DatabaseEntry {
        category_ja: "マーケット関連",
        code: "FM02",
        name_ja: "短期金融市場金利",
    },
    DatabaseEntry {
        category_ja: "マーケット関連",
        code: "FM03",
        name_ja: "短期金融市場残高",
    },
    DatabaseEntry {
        category_ja: "マーケット関連",
        code: "FM04",
        name_ja: "コール市場残高",
    },
    DatabaseEntry {
        category_ja: "マーケット関連",
        code: "FM05",
        name_ja: "公社債発行・償還および現存額",
    },
    DatabaseEntry {
        category_ja: "マーケット関連",
        code: "FM06",
        name_ja: "公社債消化状況（利付国債）",
    },
    DatabaseEntry {
        category_ja: "マーケット関連",
        code: "FM07",
        name_ja: "(参考）国債窓口販売額・窓口販売率（2004年1月まで）",
    },
    DatabaseEntry {
        category_ja: "マーケット関連",
        code: "FM08",
        name_ja: "外国為替市況",
    },
    DatabaseEntry {
        category_ja: "マーケット関連",
        code: "FM09",
        name_ja: "実効為替レート",
    },
    DatabaseEntry {
        category_ja: "決済関連",
        code: "PS01",
        name_ja: "各種決済",
    },
    DatabaseEntry {
        category_ja: "決済関連",
        code: "PS02",
        name_ja: "フェイルの発生状況",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD01",
        name_ja: "マネタリーベース",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD02",
        name_ja: "マネーストック",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD03",
        name_ja: "マネタリーサーベイ",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD04",
        name_ja: "(参考) マネーサプライ (M2+CD) 増減と信用面の対応",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD05",
        name_ja: "通貨流通高",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD06",
        name_ja: "日銀当座預金増減要因と金融調節（実績）",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD07",
        name_ja: "準備預金額",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD08",
        name_ja: "業態別の日銀当座預金残高",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD09",
        name_ja: "マネタリーベースと日本銀行の取引",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD10",
        name_ja: "預金者別預金",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD11",
        name_ja: "預金・現金・貸出金",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD12",
        name_ja: "都道府県別預金・現金・貸出金",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD13",
        name_ja: "貸出・預金動向",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "MD14",
        name_ja: "定期預金の残高および新規受入高",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "LA01",
        name_ja: "貸出先別貸出金",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "LA02",
        name_ja: "日本銀行貸出",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "LA03",
        name_ja: "その他貸出残高",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "LA04",
        name_ja: "コミットメントライン契約額、利用額",
    },
    DatabaseEntry {
        category_ja: "預金・マネー・貸出",
        code: "LA05",
        name_ja: "主要銀行貸出動向アンケート調査",
    },
    DatabaseEntry {
        category_ja: "金融機関バランスシート",
        code: "BS01",
        name_ja: "日本銀行勘定",
    },
    DatabaseEntry {
        category_ja: "金融機関バランスシート",
        code: "BS02",
        name_ja: "民間金融機関の資産・負債",
    },
    DatabaseEntry {
        category_ja: "資金循環",
        code: "FF",
        name_ja: "資金循環",
    },
    DatabaseEntry {
        category_ja: "その他の日本銀行関連",
        code: "OB01",
        name_ja: "日本銀行の対政府取引",
    },
    DatabaseEntry {
        category_ja: "その他の日本銀行関連",
        code: "OB02",
        name_ja: "日本銀行が受入れている担保の残高",
    },
    DatabaseEntry {
        category_ja: "短観",
        code: "CO",
        name_ja: "短観",
    },
    DatabaseEntry {
        category_ja: "物価",
        code: "PR01",
        name_ja: "企業物価指数",
    },
    DatabaseEntry {
        category_ja: "物価",
        code: "PR02",
        name_ja: "企業向けサービス価格指数",
    },
    DatabaseEntry {
        category_ja: "物価",
        code: "PR03",
        name_ja: "製造業部門別投入・産出物価指数",
    },
    DatabaseEntry {
        category_ja: "物価",
        code: "PR04",
        name_ja: "＜サテライト指数＞最終需要・中間需要物価指数",
    },
    DatabaseEntry {
        category_ja: "財政関連",
        code: "PF01",
        name_ja: "財政資金収支",
    },
    DatabaseEntry {
        category_ja: "財政関連",
        code: "PF02",
        name_ja: "政府債務",
    },
    DatabaseEntry {
        category_ja: "国際収支・BIS関連",
        code: "BP01",
        name_ja: "国際収支統計",
    },
    DatabaseEntry {
        category_ja: "国際収支・BIS関連",
        code: "BIS",
        name_ja: "BIS 国際資金取引統計および国際与信統計の日本分集計結果",
    },
    DatabaseEntry {
        category_ja: "国際収支・BIS関連",
        code: "DER",
        name_ja: "デリバティブ取引に関する定例市場報告",
    },
    DatabaseEntry {
        category_ja: "その他",
        code: "OT",
        name_ja: "その他",
    },
];

static PARAMETER_SPECS: &[ParameterSpec] = &[
    ParameterSpec {
        name: "FORMAT",
        description_ja: "結果ファイル形式",
        allowed_values: "JSON, CSV",
        code_api: EndpointRequirement::Optional,
        layer_api: EndpointRequirement::Optional,
        metadata_api: EndpointRequirement::Optional,
        notes: &["エラー時は指定形式にかかわらずJSONでエラー内容を出力。"],
    },
    ParameterSpec {
        name: "LANG",
        description_ja: "言語",
        allowed_values: "JP, EN",
        code_api: EndpointRequirement::Optional,
        layer_api: EndpointRequirement::Optional,
        metadata_api: EndpointRequirement::Optional,
        notes: &[],
    },
    ParameterSpec {
        name: "DB",
        description_ja: "DB名",
        allowed_values: "付録AのDBコード",
        code_api: EndpointRequirement::Required,
        layer_api: EndpointRequirement::Required,
        metadata_api: EndpointRequirement::Required,
        notes: &["DB名は付録Aを参照。"],
    },
    ParameterSpec {
        name: "CODE",
        description_ja: "系列コード",
        allowed_values: "系列コード（カンマ区切りで複数指定可、同じ期種のみ指定可）",
        code_api: EndpointRequirement::Required,
        layer_api: EndpointRequirement::Unsupported,
        metadata_api: EndpointRequirement::Unsupported,
        notes: &[
            "データコード（先頭にDB名付き）は不可。",
            "上限は1250コード。",
        ],
    },
    ParameterSpec {
        name: "LAYER",
        description_ja: "階層情報",
        allowed_values: "階層1〜5をカンマ区切り、ワイルドカード * 指定可",
        code_api: EndpointRequirement::Unsupported,
        layer_api: EndpointRequirement::Required,
        metadata_api: EndpointRequirement::Unsupported,
        notes: &["階層1は必須、階層2〜5は任意。"],
    },
    ParameterSpec {
        name: "FREQUENCY",
        description_ja: "期種",
        allowed_values: "CY, FY, CH, FH, Q, M, W, D",
        code_api: EndpointRequirement::Unsupported,
        layer_api: EndpointRequirement::Required,
        metadata_api: EndpointRequirement::Unsupported,
        notes: &["週次にはW0〜W6が存在するが指定時はWを利用。"],
    },
    ParameterSpec {
        name: "STARTDATE",
        description_ja: "開始期",
        allowed_values: "CY/FY: YYYY, CH/FH: YYYYHH, Q: YYYYQQ, M/W/D: YYYYMM",
        code_api: EndpointRequirement::Optional,
        layer_api: EndpointRequirement::Optional,
        metadata_api: EndpointRequirement::Unsupported,
        notes: &["開始期未指定時は収録開始期から出力。"],
    },
    ParameterSpec {
        name: "ENDDATE",
        description_ja: "終了期",
        allowed_values: "STARTDATEと同形式",
        code_api: EndpointRequirement::Optional,
        layer_api: EndpointRequirement::Optional,
        metadata_api: EndpointRequirement::Unsupported,
        notes: &["終了期未指定時は収録終了期まで出力。"],
    },
    ParameterSpec {
        name: "STARTPOSITION",
        description_ja: "検索開始位置",
        allowed_values: "1以上の整数",
        code_api: EndpointRequirement::Optional,
        layer_api: EndpointRequirement::Optional,
        metadata_api: EndpointRequirement::Unsupported,
        notes: &["上限超過時にNEXTPOSITIONと組み合わせて継続取得。"],
    },
];

static REQUEST_LIMITS: &[RequestLimit] = &[
    RequestLimit {
        api_scope: "getDataLayer",
        target: "検索条件で抽出される系列数（期種絞り込み前）",
        max_value: 1250,
        overflow_behavior: "上限を超える場合はエラー。出力ファイルは作成されない。",
    },
    RequestLimit {
        api_scope: "getDataCode,getDataLayer",
        target: "1回のリクエストで検索可能な系列数",
        max_value: 250,
        overflow_behavior: "上限まで出力し、続き検索用にNEXTPOSITIONを出力。",
    },
    RequestLimit {
        api_scope: "getDataCode,getDataLayer",
        target: "1回のリクエストで検索可能なデータ数（系列数×期数）",
        max_value: 60000,
        overflow_behavior: "上限まで出力し、続き検索用にNEXTPOSITIONを出力。",
    },
];

static LAYER_RULES: &[&str] = &[
    "階層1の指定は必須。",
    "階層2〜5は任意。",
    "複数階層はカンマ区切りで指定。",
    "* は当該階層を全件対象とするワイルドカード。",
];

static MESSAGE_CODES: &[MessageCodeSpec] = &[
    MessageCodeSpec {
        status: 200,
        message_id: "M181000I",
        message: "正常に終了しました。",
        note: "一部のデータが欠損値の場合も含む。",
    },
    MessageCodeSpec {
        status: 200,
        message_id: "M181030I",
        message: "正常に終了しましたが、該当データはありませんでした。",
        note: "「該当データなし」は、指定系列・時期が全て収録期間外、または指定全系列が欠損値の場合。",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181001E",
        message: "Invalid input parameters",
        note: "一部の記号（`< > ” ! | \\ ; '`）や全角文字は利用不可。系列コード先頭にDB名を付けた場合（例: `IR01’MADR1Z@D`）も本メッセージ。",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181002E",
        message: "Invalid language setting",
        note: "言語設定が正しくありません。",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181003E",
        message: "結果ファイル形式が正しくありません。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181004E",
        message: "DBが指定されていません。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181005E",
        message: "DB名が正しくありません。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181006E",
        message: "系列コードが指定されていません。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181007E",
        message: "系列コードの数が1250を超えています。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181008E",
        message: "指定した開始期が正しくありません。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181009E",
        message: "指定した終了期が正しくありません。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181010E",
        message: "時期は1850年から2050年までを数値で指定してください。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181011E",
        message: "開始期と終了期の順序を正しく指定してください。",
        note: "開始期≦終了期で指定。",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181012E",
        message: "検索開始位置が正しくありません。",
        note: "1以上の整数を指定。指定方法は II.4.(2) を参照。",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181013E",
        message: "指定した系列コードは存在しません。:*番目のコード",
        note: "`*` は指定順を表示。",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181014E",
        message: "指定した系列コードの期種が一致しません。:*番目のコード",
        note: "`*` は指定順を表示。",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181015E",
        message: "指定した開始期の設定形式が期種と一致しません。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181016E",
        message: "指定した終了期の設定形式が期種と一致しません。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181017E",
        message: "期種が指定されていません。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181018E",
        message: "期種が正しくありません。",
        note: "-",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181019E",
        message: "階層情報が指定されていません。",
        note: "階層1は必須。階層2〜5は任意。6つ以上は指定不可。",
    },
    MessageCodeSpec {
        status: 400,
        message_id: "M181020E",
        message: "階層情報設定が正しくありません。",
        note: "正しい指定方法は II.3.(3) を参照。",
    },
    MessageCodeSpec {
        status: 500,
        message_id: "M181090S",
        message: "予期しないエラーが発生しました。時間をおいてからやり直してください。",
        note: "-",
    },
    MessageCodeSpec {
        status: 503,
        message_id: "M181091S",
        message: "データベースにアクセス中にエラーになりました。時間をおいてからやり直してください。",
        note: "-",
    },
];

static CATALOG_SNAPSHOT: CatalogSnapshot = CatalogSnapshot {
    source_document: "api_manual.pdf",
    source_date: "2026-02-18",
    general_notes: GENERAL_NOTES,
    format_codes: FORMAT_CODES,
    language_codes: LANGUAGE_CODES,
    frequency_codes: FREQUENCY_CODES,
    databases: DATABASES,
    parameters: PARAMETER_SPECS,
    limits: REQUEST_LIMITS,
    layer_rules: LAYER_RULES,
    messages: MESSAGE_CODES,
};

/// Returns the immutable discovery catalog snapshot.
pub fn snapshot() -> &'static CatalogSnapshot {
    &CATALOG_SNAPSHOT
}

/// Returns all known DB entries from appendix A.
pub fn databases() -> &'static [DatabaseEntry] {
    DATABASES
}

/// Returns all parameter specs from the request matrix.
pub fn parameter_specs() -> &'static [ParameterSpec] {
    PARAMETER_SPECS
}

/// Returns all request limit definitions.
pub fn request_limits() -> &'static [RequestLimit] {
    REQUEST_LIMITS
}

/// Returns the documented layer rules.
pub fn layer_rules() -> &'static [&'static str] {
    LAYER_RULES
}

/// Returns all documented message code rows.
pub fn message_codes() -> &'static [MessageCodeSpec] {
    MESSAGE_CODES
}

/// Returns documented response format codes.
pub fn format_codes() -> &'static [&'static str] {
    FORMAT_CODES
}

/// Returns documented language codes.
pub fn language_codes() -> &'static [&'static str] {
    LANGUAGE_CODES
}

/// Returns documented frequency codes.
pub fn frequency_codes() -> &'static [&'static str] {
    FREQUENCY_CODES
}

/// Finds a DB entry by code using ASCII case-insensitive match.
pub fn find_db(code: &str) -> Option<&'static DatabaseEntry> {
    DATABASES
        .iter()
        .find(|entry| entry.code.eq_ignore_ascii_case(code))
}

/// Returns `true` when the input code exists in appendix A.
pub fn is_known_db(code: &str) -> bool {
    find_db(code).is_some()
}
