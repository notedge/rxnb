use crate::widgets::icons;

use super::*;

pub struct NotebookLeftPanel {
    link: ComponentLink<Self>,
}

impl Component for NotebookLeftPanel {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
        <aside class="notebook-left-panel">
            {self.side_toc_button()}
            {self.side_dir_button()}
            {self.side_search_button()}
            {self.side_user_button()}
        </aside>
        }
    }
}

impl NotebookLeftPanel {
    fn side_toc_button(&self) -> Html {
        cell_with_tooltip(icons::toc_icon(28), "[unimplemented] Show table of content")
    }
    fn side_dir_button(&self) -> Html {
        cell_with_tooltip(icons::workspace_icon(28), "[unimplemented] Show workspace structure")
    }
    fn side_search_button(&self) -> Html {
        cell_with_tooltip(icons::search_icon(28), "[unimplemented] Search in workspace or notebook")
    }
    fn side_user_button(&self) -> Html {
        cell_with_tooltip(icons::user_icon(28), "[unimplemented] Login to some storage")
    }
}

pub fn cell_with_tooltip(icon: Html, text: &str) -> Html {
    html! {
    <button class="cell-run">
        <div class="tooltip">
            {icon}
            <div class="tooltip-text">{text}</div>
        </div>
    </button>
    }
}
