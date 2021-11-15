

use seed::app::{CmdHandle, Orders};
use seed::prelude::*;

use seed::{attrs, div, document, empty, input, nodes, window, Url, C};

use crate::data_model::SiteConfig;
use crate::{Msg as MainMsg, Page};

pub enum Msg {
    Submit(String),
    Error(String),
}

pub struct Model {
    raw_data: Data,
    url: Url,
    danger_alert: Option<String>,
}

impl Model {
    pub fn view(&self) -> Vec<Node<MainMsg>> {
        let data = self.raw_data.clone();
        document().set_title(&format!("Login - {}", &data.site.title));
        nodes![
            if let Some(alert) = &self.danger_alert {
                div![C!["uk-alert", "uk-alert-danger"], alert]
            } else {
                empty!()
            },
            input![
                C!["uk-input", "uk-form-blank", "uk-form-width-medium"],
                attrs! {
                    At::Type => "password"
                },
                input_ev(Ev::Change, |s| MainMsg::LoginMsg(Msg::Submit(s)))
            ]
        ]
    }

    pub fn update(&mut self, msg: Msg, orders: &mut impl Orders<MainMsg>) {
        match msg {
            Msg::Error(err) => {
                self.danger_alert = Some(err);
            }
            Msg::Submit(pwd) => {
                if pwd.is_empty() {
                    self.danger_alert = Some(pwd);
                } else {
                    let url = self.url.clone();

                    orders.perform_cmd(async move {
                        #[derive(serde::Serialize)]
                        struct LoginData {
                            password: String,
                        }
                        #[derive(serde::Deserialize)]
                        struct Empty {}

                        match crate::fetch::post::<LoginData, Empty>(
                            "/auth",
                            &LoginData { password: pwd },
                        )
                        .await
                        {
                            Ok(_) => {
                                window()
                                    .location()
                                    .replace(&url.clone().set_hash("").to_string())
                                    .unwrap();
                                None
                            }
                            Err(err) => Some(MainMsg::LoginMsg(Msg::Error(match err {
                                FetchError::StatusError(status) => {
                                    let code = status.code;
                                    match code {
                                        401 | 403 => "wrong password".to_owned(),
                                        other => format!("unexpected status: {}", other),
                                    }
                                }
                                err => format!("fetch error: {:?}", err),
                            }))),
                        }
                    });
                }
            }
        }
    }
}

pub fn init(url: &mut Url, orders: &mut impl Orders<MainMsg>) -> CmdHandle {
    let url = url.clone();
    orders.perform_cmd_with_handle(async move {
        let data = crate::fetch::get::<Data>("/auth/api").await.unwrap();

        if data.logged {
            window()
                .location()
                .replace(&url.clone().set_hash("").to_string())
                .unwrap();
            return None;
        }

        Some(MainMsg::Render(Page::Login(Model {
            url,
            raw_data: data,
            danger_alert: None,
        })))
    })
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Data {
    site: SiteConfig,
    logged: bool,
}
