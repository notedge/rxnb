use std::iter::FromIterator;
use std::rc::Rc;

use yew::prelude::*;
use yew::services::ConsoleService;
use yew::web_sys::KeyboardEvent;

pub enum Event {
    Input(InputData),
    Length(ChangeData),
    Mode(ChangeData),
    Update(ChangeData),
    Press(KeyboardEvent),
}

pub enum CellState {
    /// Gray Cell
    /// this cell doesn't evaluate even once
    Unevaluated,
    /// Red Cell
    /// Syntax error, runtime error or cancel by user
    Canceled,
    /// Orange Cell
    /// This cell is evaluating
    Evaluating,
    /// Yellow Cell
    /// The dependencies are evaluating, this cell is waiting for update
    Pending,
    /// Blue Cell
    /// This cell is up to date but some dependencies are canceled(red)
    Fine,
    /// Green Cell
    /// Everything just ok
    OK,
}


pub struct NotebookCell {
    link: ComponentLink<Self>,
    id: usize,
    state: CellState,
    kind: String,
    input: String,
    input_render: String,
    out: Html,
}

impl Component for NotebookCell {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            id: 0,
            state: CellState::Unevaluated,
            kind: "".to_string(),
            input: "".to_string(),
            input_render: "".to_string(),
            out: Html::from(200),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Input(s) => {
                self.input = s.value;
                false
            }
            Event::Length(_) => { false }
            Event::Mode(_) => { false }
            Event::Update(_) => { false }
            Event::Press(key) => {
                self.out = Html::from_iter(self.input.lines().map(|e| html! { <div>{e}</div>}));
                ConsoleService::info(&key.code());
                match key {
                    key if key.code() == "Enter" && key.shift_key() == true => {
                        // evaluate and select next cell
                        true
                    },
                    key if key.code() == "Enter" && key.ctrl_key() == true => {
                        true
                    },
                    _ => false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let key = self.link.callback(Event::Press);
        let txt = self.link.callback(Event::Input);

        html! {
            <div>
                <h1>{"in"}</h1>
            <pre
                class="input-cell"
                contenteditable="true"
                spellcheck="false"
                placeholder="notedown mode"
                onkeypress=key
                oninput=txt
            >
            </pre>
            <h2>{"id: "} {&self.id}</h2>
            <h2>{"kind: "} {&self.kind}</h2>

            <h1>{"out"}</h1>
            {self.out.to_owned()}
            </div>
        }
    }
}

impl NotebookCell {
    fn get_input(&mut self) {}

    fn update_out(&mut self) {}
}
