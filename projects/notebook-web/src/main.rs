#![recursion_limit = "1024"]

use yew::{
    Component,
    ComponentLink,
    html,
    Html, prelude::*, services::reader::{FileData, ReaderService, ReaderTask}, ShouldRender,
};

use rxnb::cell::NotebookCell;

mod cell;

pub enum Event {
    Input(String),
    Length(ChangeData),
    Mode(ChangeData),
    Files(ChangeData),
    Loaded(FileData),
}

#[derive(Debug)]
pub struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <>
        <header>
          <div id="header-logo"><a href="/logo.svg"> </a></div>
            </header>
        <main class="notebook-horizontal">
            <aside class="notebook-left-pane">
            <span>{"icon"}</span>
            </aside>
            <div class="notebook-right-pane">
                <nav class="notebook-tab">{"title"}</nav>
                <article class="notebook-body">
                    <NotebookCell/>
                    <NotebookCell/>
                    <NotebookCell/>
                    <NotebookCell/>
                    <NotebookCell/>
                </article>
            </div>
        </main>
        // <footer> </footer>
        </>
        }
    }
}

impl Model {}

fn main() {
    yew::start_app::<Model>();
}
