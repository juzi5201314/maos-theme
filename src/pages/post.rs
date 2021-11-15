use std::collections::BTreeMap;

use seed::app::{CmdHandle, Orders};
use seed::prelude::*;
use seed::virtual_dom::Node;

use seed::{
    article, attrs, div, document, h1, h5, header, img, li, md, nodes, p, ul, Url, C,
};

use crate::data_model::{CommentModel, PostModel, SiteConfig};
use crate::{Msg, Page};

pub struct Model {
    raw_data: Data,
}

impl Model {
    pub fn view(&self) -> Vec<Node<Msg>> {
        let data = self.raw_data.clone();
        document().set_title(&data.post.title);
        nodes![
            article![
                C!["uk-article"],
                h1![
                    C!["uk-h1", "uk-article-title"],
                    data.post.title
                ],
                p![
                    C!["uk-article-meta"],
                    data.post.create_time.format("%Y-%m-%d %H:%M").to_string()
                ],
                p!(md!(&data.post.content)),

                data
                    .comments
                    .into_iter()
                    .filter(|(_, comment)| !comment.deleted)
                    .map(|(_, comment)| {
                    article![
                        C!["uk-comment"],
                        header![
                            C!["uk-comment-header"],
                            div![
                                C!["uk-grid-medium", "uk-flex-middle"],
                                attrs! {
                                    At::from("uk-grid") => AtValue::None
                                },

                                div![
                                    C!["uk-width-auto"],
                                    img![
                                        C!["uk-comment-avatar"],
                                        attrs! {
                                            At::Src => format!("https://www.gravatar.com/avatar/{:?}", md5::compute(comment.email.as_bytes()))
                                        }
                                    ]
                                ],

                                div![
                                    C!["uk-width-expand"],
                                    h5![
                                        C!["uk-comment-title", "uk-margin-remove"],
                                        comment.nickname
                                    ],
                                    ul![
                                        C!["uk-comment-meta", "uk-subnav", "uk-subnav-divider", "uk-margin-remove-top"],
                                        li!(comment.create_time.format("%Y-%m-%d %H:%M").to_string()),
                                        li!("Reply")
                                    ]
                                ]
                            ]
                        ],
                        div![
                            C!["uk-comment-body"],
                            p!(comment.content)
                        ]
                    ]
                }).collect::<Vec<Node<Msg>>>()
            ]
        ]
    }
}

pub fn init(url: &mut Url, orders: &mut impl Orders<Msg>) -> CmdHandle {
    let url = url.clone();
    orders.perform_cmd_with_handle(async move {
        if let Some(post_id) = url
            .clone()
            .next_hash_path_part()
            .map(|id| id.parse::<u32>().ok())
            .flatten()
        {
            let data = crate::fetch::get::<Data>(&format!("/post/{}/api", post_id))
                .await
                .unwrap();
            Msg::Render(Page::Post(Model {
                raw_data: data,
            }))
        } else {
            Msg::Render(Page::NotFound)
        }
    })
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Data {
    site: SiteConfig,
    logged: bool,
    post: PostModel,
    comments: BTreeMap<u32, CommentModel>,
}
