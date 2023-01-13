use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyArticle {
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Content")]
    pub content: String,
    pub author: Author,
    #[serde(rename = "TimeToRead")]
    pub time_to_read: i64,
    #[serde(rename = "Summary")]
    pub summary: String,
    #[serde(rename = "published_at")]
    pub published_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub category: Category,
    #[serde(rename = "Slug")]
    pub slug: String,
    #[serde(rename = "CoverPhoto")]
    pub cover_photo: CoverPhoto,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Company")]
    pub company: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "published_at")]
    pub published_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "LinkToProfile")]
    pub link_to_profile: Value,
    #[serde(rename = "Avatar")]
    pub avatar: Avatar,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub id: i64,
    pub name: String,
    pub alternative_text: String,
    pub caption: String,
    pub width: i64,
    pub height: i64,
    pub hash: String,
    pub ext: String,
    pub mime: String,
    pub size: f64,
    pub url: String,
    pub preview_url: Value,
    pub provider: String,
    #[serde(rename = "provider_metadata")]
    pub provider_metadata: Value,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "published_at")]
    pub published_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "Order")]
    pub order: i64,
    #[serde(rename = "Home_Visible")]
    pub home_visible: bool,
    #[serde(rename = "Card_Count")]
    pub card_count: i64,
    #[serde(rename = "Featured_Card")]
    pub featured_card: bool,
    #[serde(rename = "Column_Count")]
    pub column_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverPhoto {
    pub id: i64,
    pub name: String,
    pub alternative_text: String,
    pub caption: String,
    pub width: i64,
    pub height: i64,
    pub hash: String,
    pub ext: String,
    pub mime: String,
    pub size: f64,
    pub url: String,
    pub preview_url: Value,
    pub provider: String,
    #[serde(rename = "provider_metadata")]
    pub provider_metadata: Value,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}



// New Post
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
    pub data: CreatePostData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostData {
    pub title: String,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    pub tags: Vec<CreatePostTag>,
    pub post_type: Vec<CreatePostPostType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostTag {
    pub id: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostPostType {
    #[serde(rename = "__component")]
    pub component: String,
    pub content: String,
    pub time_to_read: i64,
    pub summary: String,
    pub card_type: String,
    pub author: CreatePostAuthor
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostAuthor {
    pub id: i64,
}


// RESPONSE
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostResponse {
    pub data: CreatePostResponseData
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostResponseData {
    pub id: i32,
    pub attributes: CreatePostResponseDataAttributes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostResponseDataAttributes {
    pub created_at: String,
    pub updated_at: String,
    pub published_at: String,
    pub title: String,
    pub slug: String,
}
