use futures::{
    future::{FusedFuture, FutureExt},
    prelude::*,
};
use std::pin::Pin;

use dioxus::prelude::*;

use crate::{
    stats::Stats,
    words::{make_words, Segment},
};

pub fn get_textarea() -> Option<web_sys::HtmlTextAreaElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id("input")
        .and_then(|el| wasm_bindgen::JsCast::dyn_into(el).ok())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum InputAction {
    Set { refresh: bool, s: String },
    Append { refresh: bool, s: String },
    Refresh,
}

pub async fn input_service(
    mut rx: UnboundedReceiver<InputAction>,
    words: UseRef<Vec<Segment>>,
    stats: UseRef<Stats>,
) {
    let mknever = || Box::pin(future::pending());
    let mktimeout = |ms| {
        Box::pin({
            async move {
                gloo_timers::future::TimeoutFuture::new(ms).await;
                Some(InputAction::Refresh)
            }
            .fuse()
        })
    };
    let mut fut: Pin<Box<dyn FusedFuture<Output = Option<InputAction>>>> = Box::pin(mknever());
    let mut lastval = String::default();
    let textbox = get_textarea().expect("Could not get input text area");
    while let Some(msg) = futures::select! {
      m1 = rx.next() => m1,
      m2 = fut => m2,
    } {
        let (refresh, curr) = match msg {
            InputAction::Set { refresh, s } => (refresh, Some(s)),
            InputAction::Append { refresh, s } => (
                refresh,
                Some(String::from_iter(
                    [textbox.value().as_str(), s.as_str()].into_iter(),
                )),
            ),
            InputAction::Refresh => (true, None),
        };

        if let Some(s) = curr {
            lastval = s;
            textbox.set_value(&lastval);
        }
        if refresh {
            fut = mknever();
            let (newwords, newstats) = make_words(&lastval);
            words.set(newwords);
            stats.set(newstats);
            continue;
        }
        let inputlen = lastval.len();
        let timeoutms = if inputlen < 500 {
            100
        } else if inputlen < 2000 {
            250
        } else {
            500
        };
        fut = mktimeout(timeoutms);
    }
}
