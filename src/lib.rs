#![feature(box_syntax)]
#![feature(try_blocks)]

use seed::app::{CmdHandle, Orders};

use seed::virtual_dom::IntoNodes;

use seed::{nodes, plain, App, Url};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::fetch::post;

mod fetch;
mod pages;
mod data_model;

seed::struct_urls!();

impl<'a> Urls<'a> {
    pub fn index_url(self) -> Url {
        self.base_url()
    }

    pub fn post_url(self, post_id: u32) -> Url {
        self.base_url().add_hash_path_part("post").add_hash_path_part(post_id.to_string())
    }
}

fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        base_url: url.to_hash_base_url(),
        page: Page::Wait(Page::init(&mut url, orders)),
    }
}

struct Model {
    base_url: Url,
    page: Page,
}

pub enum Msg {
    UrlChanged(seed::prelude::subs::UrlChanged),

    Render(Page),

    LoginMsg(pages::login::Msg),
    Logout
}

pub enum Page {
    Index(pages::index::Model),
    Post(pages::post::Model),
    Login(pages::login::Model),

    Wait(Option<CmdHandle>),
    NotFound,
}

impl Page {
    fn init(url: &mut Url, orders: &mut impl Orders<Msg>) -> Option<CmdHandle> {
        Some(match url.next_hash_path_part() {
            None => pages::index::init(url, orders),
            Some("post") => pages::post::init(url, orders),
            Some("login") => pages::login::init(url, orders),
            _ => {
                orders.send_msg(Msg::Render(Page::NotFound));
                return None;
            }
        })
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(seed::prelude::subs::UrlChanged(mut url)) => model.page = Page::Wait(Page::init(&mut url, orders)),
        Msg::Render(page) => model.page = page,

        Msg::LoginMsg(msg) => {
            if let Page::Login(model) = &mut model.page {
                model.update(msg, orders)
            }
        }
        Msg::Logout => {
            let url = model.base_url.clone();
            orders.perform_cmd(async move {
                #[derive(serde::Serialize, serde::Deserialize)]
                struct Empty {}
                post::<_, Empty>("/auth/logout", &Empty {}).await.unwrap();
                Msg::UrlChanged(seed::prelude::subs::UrlChanged(url))
            });
        }
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    nodes![
        match &model.page {
            Page::Index(model) => model.view(),
            Page::Post(model) => model.view(),
            Page::Login(model) => model.view(),
            Page::NotFound => {
                nodes![plain!("404")]
            }
            Page::Wait(_) => crate::pages::wait::view(),
        }
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    seed::console_error_panic_hook::set_once();
    App::start("app", init, update, view);
}
