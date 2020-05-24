use std::iter::FromIterator;
use std::ops::Div;
use std::rc::Rc;

use monaco::api::CodeEditorOptions;
use monaco::sys::editor::IDimension;
use monaco::yew::{CodeEditor, CodeEditorLink};
use yew::prelude::*;
use yew::Renderable;
use yew::services::ConsoleService;
use yew::web_sys::KeyboardEvent;

use crate::configs::LanguageConfig;
use crate::widgets::icons;

pub enum Event {
    Input(InputData),
    Length(ChangeData),
    Mode(ChangeData),
    Update(ChangeData),
    Press(KeyboardEvent),
    RunCell,
    ChangePinState,
    RenderOutput(String),
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
    editor: Html,
    editor_link: CodeEditorLink,
    /// run ID
    id: usize,
    expand: bool,
    pin: bool,
    ///
    state: CellState,
    ///
    ///
    language: LanguageConfig,
    out: Html,
}

impl Component for NotebookCell {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            id: 0,
            expand: false,
            pin: false,
            state: CellState::Unevaluated,
            language: LanguageConfig::default(),
            editor: Default::default(),
            editor_link: CodeEditorLink::default(),
            out: Html::from(200),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Input(_s) => {
                // self.input = s.value;
                false
            }
            Event::Length(_) => { false }
            Event::Mode(_) => { false }
            Event::Update(_) => { false }
            Event::Press(_key) => {
                // self.out = Html::from_iter(self.input.lines().map(|e| html! { <div>{e}</div>}));
                // ConsoleService::info(&key.code());
                // match key {
                //     key if key.code() == "Enter" && key.shift_key() == true => {
                //         // evaluate and select next cell
                //         true
                //     },
                //     key if key.code() == "Enter" && key.ctrl_key() == true => {
                //         true
                //     },
                //     _ => false
                // }
                false
            }
            Event::RunCell => {
                match self.editor_link.get_value() {
                    Some(s) => { self.link.callback(Event::RenderOutput).emit(s) },
                    None => (),
                }
                false
            }
            Event::ChangePinState => {
                self.pin = !self.pin;
                true
            }
            Event::RenderOutput(s) => {
                ConsoleService::info(&s);
                self.out = Html::from(s);
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        //let key = self.link.callback(Event::Press);
        //let txt = self.link.callback(Event::Input);
        let options = CodeEditorOptions {
            dimension: Some(IDimension::new(800, 300)),
            theme: None,
            model: None,
            language: None,
            value: Some("200".to_string()),
        };
        html! {
        <div class="notebook-cell" data-node-id=self.id>
            {self.left_panel()}
            <div class="cell-right-panel">
            <CodeEditor
                link=self.editor_link.clone()
                options=Rc::new(options)
            />
            {self.out.to_owned()}
            {self.cell_toolbar()}
            </div>
            <div class="cell-right-empty"/>
        </div>
        }
    }
    fn rendered(&mut self, _: bool) {}
}

impl NotebookCell {
    fn left_panel(&self) -> Html {
        html! {
            <div class="cell-left-panel">
                // {self.cell_run_button()}
            </div>
        }
    }

    fn cell_toolbar(&self) -> Html {
        html! {
            <div class="cell-toolbar">
                {self.run_button()}
                {self.cell_share_button()}
                {self.remove_button()}
                {self.cell_more_button()}
            </div>
        }
    }

    //fn get_input(&mut self) {}

    //fn update_out(&mut self) {}
}

impl NotebookCell {
    #[inline]
    fn cell_more_button(&self) -> Html {
        cell_with_tooltip(icons::tab_icon(16), "[unimplemented]Click to show more operations")
    }
    #[inline]
    fn cell_share_button(&self) -> Html {
        cell_with_tooltip(icons::link_icon(16), "[unimplemented]Click to share this cell")
    }

    fn run_button(&self) -> Html {
        let click_callback = self.link.callback(|_| Event::RunCell);
        html! {
        <button class="cell-run" onclick=click_callback>
            <div class="tooltip">
                {icons::run_icon(16)}
                <div class="tooltip-text">
                    {"Click to run cell"}
                </div>
            </div>
        </button>
        }
    }
    #[inline]
    fn remove_button(&self) -> Html {
        cell_with_tooltip(icons::remove_icon(16), "[unimplemented]Click to remove this cell")
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
