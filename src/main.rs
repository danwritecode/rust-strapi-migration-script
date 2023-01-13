mod strapi_models;
use slugify::slugify;
use std::collections::HashMap;
use strapi_models::strapi_models::{ LegacyArticle, CreatePost, CreatePostData, CreatePostPostType, CreatePostAuthor, CreatePostResponse, CreatePostTag };
use reqwest::{ Error, Client, multipart };
use reqwest::multipart::Part;
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let mut legacy_articles: Vec<LegacyArticle> = vec![];

    match get_articles().await {
        Ok(a) => legacy_articles = a,
        Err(e) => println!("Something went wrong getting legacy articles: {}", e)
    }

    for article in &legacy_articles {
        let post = map_article(article);
        let cover_photo = get_cover_photo(&article.cover_photo.url).await;

        post_article_and_cover_photo(post, cover_photo.unwrap()).await;
    }
}

async fn get_cover_photo(url: &String) -> Result<Bytes, Error> {
    let req_url = format!("{}{}", "", url);
    let response:Bytes = reqwest::get(req_url)
        .await?
        .bytes()
        .await?;
    Ok(response)
}

async fn post_article_and_cover_photo(article:CreatePost, image: Bytes) -> Result<CreatePostResponse, Error> {
    // let request_url = "http://localhost:1337/api/posts?populate=deep";
    let request_url = "";

    let image_vec = Vec::from(image);
    let part = multipart::Part::bytes(image_vec)
        .file_name(format!("{}.png", slugify!(article.data.title.as_str())))
        .mime_str("image/png")?;

    let article_json = serde_json::to_string(&article.data).unwrap();
    let part_ref_data = multipart::Part::text(article_json);

    let form = multipart::Form::new()
        .part("data", part_ref_data)
        .part("files.postType[0].coverPhoto", part);

    let response = Client::new()
        .post(request_url)
        // .header("Authorization", "")
        .header("Authorization", "")
        .multipart(form)
        .send().await?;

    println!("HTTP Status: {} | Created Post: {}", response.status(), &article.data.title);

    let create_post_resp = response.json().await?;
    Ok(create_post_resp)
}

async fn get_articles() -> Result<Vec<LegacyArticle>, Error> {
    let response = reqwest::get("").await?;
    let articles: Vec<LegacyArticle> = response.json().await?;
    
    Ok(articles)
}

fn map_article(article: &LegacyArticle) -> CreatePost {
    let post = CreatePost {
        data: CreatePostData {
            title: article.title.clone(),
            tags: vec![CreatePostTag {
                id: get_category_tag_id(article.category.name.clone())
            }],
            published_at: get_iso_date(article.published_at.clone()),
            post_type: vec![
                CreatePostPostType {
                   	component: "post-types.article".to_string(),
				    content: article.content.clone(),
				    time_to_read: article.time_to_read,
				    summary: article.summary.clone(),
				    card_type: "standard".to_string(), 
				    author: CreatePostAuthor {
				        id: 1
				    }
                }
            ]
        }
    };
    return post;
}

fn get_category_tag_id(name:String) -> i32 {
    let tags = HashMap::from([
        ("Featured Articles", 1),
        ("Clinical News", 2),
        ("Partner News", 3),
        ("AscellaHealth Innovations", 4),
        ("Featured Consultant", 5),
    ]);

    let id = tags.get(&name.as_str()).unwrap();
    return *id;
}

fn get_iso_date(date:String) -> String {

    let published_iso = date
        .split("T")
        .take(1)
        .collect::<Vec<&str>>();

    return published_iso.first().unwrap().to_string();
}

#[deprecated]
#[allow(dead_code)]
async fn post_article(article:CreatePost) -> Result<CreatePostResponse, Error> {
    let request_url = "http://localhost:1337/api/posts";

    let response = Client::new()
        .post(request_url)
        .header("Authorization", "")
        .json(&article)
        .send().await?;

    let create_post_resp = response.json().await?;

    Ok(create_post_resp)
}

#[deprecated]
#[allow(dead_code)]
async fn post_cover_photo(image:Bytes, post_id: i32) -> Result<(), Error> {
    let request_url = "http://localhost:1337/api/upload";

    let image_vec = Vec::from(image);
    let part = multipart::Part::bytes(image_vec)
        .file_name("testing_image_upload_v1.png");

    let part_ref = multipart::Part::text("api::post.post");
    let part_ref_id = multipart::Part::text(post_id.to_string());
    let part_ref_field = multipart::Part::text("test");

    let form = multipart::Form::new()
        .part("files", part)
        .part("ref", part_ref)
        .part("refId", part_ref_id)
        .part("field", part_ref_field);


    let response = Client::new()
        .post(request_url)
        .header("Authorization", "")
        .multipart(form)
        .send()
        .await?;

    Ok(())
}
