use crate::widgets::icons;

use super::*;

pub enum Event {
    CreateNewCell(usize)
}

pub struct NotebookSplit {
    link: ComponentLink<Self>,
}


impl Component for NotebookSplit {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Event::CreateNewCell(_) => { false }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
        <div class="notebook-cell-split">
            <div class="drag-marker-before"></div>

            {self.btn_add_new()}
            <div class="drag-marker-after"></div>
        </div>
        }
    }
}

impl NotebookSplit {
    fn btn_add_new(&self) -> Html {
        let click_callback = self.link.callback(|_| Event::CreateNewCell(0));
        html! {
        <button class="cell-run" onclick=click_callback>
            <div class="tooltip">
                {icons::add_icon(16)}
                <div class="tooltip-text">
                    {"[unimplemented]Click to create new cell"}
                </div>
            </div>
        </button>
        }
    }
}
