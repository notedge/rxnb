use super::*;

pub struct NotebookSplit {
    link: ComponentLink<Self>,
}


impl Component for NotebookSplit {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
        <div class="notebook-cell-split">
            {"title"}
        </div>
        }
    }
}

impl NotebookSplit {
    fn btn_add_new(&self) {}
}
