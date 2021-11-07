use seed::virtual_dom::Node;
use seed::{plain, Url, nodes};
use crate::{ViewModel, Msg};
use seed::app::{Orders, CmdHandle};


pub struct Model {

}

impl ViewModel for Model {
    fn view(&self) -> Vec<Node<Msg>> {
        nodes![
            plain!("post")
        ]
    }
}

pub fn init(url: &mut Url, orders: &mut impl Orders<Msg>) -> CmdHandle {
    let mut url = url.clone();
    orders.perform_cmd_with_handle(async move {
        if let Some(post_id) = url.next_hash_path_part().map(|id| id.parse::<u32>().ok()).flatten() {
            seed::log!(post_id);
            //let data = crate::fetch::get::<Data>("/api").await.unwrap();
            Msg::Render(Some(box Model {}))
        } else {
            Msg::Render(None)
        }
    })
}
