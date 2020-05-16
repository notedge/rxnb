use super::*;

pub struct NotebookTabs {
    link: ComponentLink<Self>,
}


impl Component for NotebookTabs {
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
        <nav class="notebook-tab">{"title"}</nav>
        }
    }
}
