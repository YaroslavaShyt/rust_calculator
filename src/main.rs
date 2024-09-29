use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, PlatformError, Widget, WidgetExt, WindowDesc};
use druid::{EventCtx, Selector};

mod calculator;
use calculator::Calculator;

#[derive(Clone, Data, Lens)]
struct CalcState {
    input: String,
    output: String,
    memory: String,
}

const CALCULATE: Selector = Selector::new("calculate");

fn build_ui() -> impl Widget<CalcState> {
    let input_box = TextBox::new()
        .with_placeholder("Enter expression")
        .fix_width(300.0)
        .lens(CalcState::input);

    let output_label =
        Label::new(|data: &CalcState, _env: &Env| format!("Result: {}", data.output))
            .with_text_size(20.0);

    let memory_label =
        Label::new(|data: &CalcState, _env: &Env| format!("Memory: {}", data.memory))
            .with_text_size(16.0)
            .padding(10.0);

    let buttons = build_buttons();

    let calculate_button =
        Button::new("=").on_click(|ctx: &mut EventCtx, data: &mut CalcState, _env: &Env| {
            ctx.submit_command(CALCULATE);
        });

    let clear_button =
        Button::new("C").on_click(|_ctx: &mut EventCtx, data: &mut CalcState, _env: &Env| {
            data.input.clear();
        });

    let memory_save_button =
        Button::new("M+").on_click(|_ctx: &mut EventCtx, data: &mut CalcState, _env: &Env| {
            data.memory = data.output.clone();
        });

    let memory_recall_button =
        Button::new("MR").on_click(|_ctx: &mut EventCtx, data: &mut CalcState, _env: &Env| {
            data.input.push_str(&data.memory);
        });

    let mut col = Flex::column();
    col.add_child(input_box.padding(25.0));
    col.add_child(memory_label);
    col.add_child(buttons);
    col.add_child(
        Flex::row()
            .with_child(calculate_button)
            .with_child(clear_button)
            .with_child(memory_save_button)
            .with_child(memory_recall_button)
            .padding(25.0),
    );
    col.add_child(output_label.padding(25.0));

    col
}

fn build_buttons() -> impl Widget<CalcState> {
    let mut button_layout = Flex::column();

    let buttons = vec![
        vec!["0", "1", "2", "/"],
        vec!["3", "4", "5", "*"],
        vec!["6", "7", "8", "-"],
        vec!["9", ".", "+", ""],
    ];

    for row in buttons {
        let mut row_layout = Flex::row();
        for &label in &row {
            if !label.is_empty() {
                let button = Button::new(label).on_click(
                    move |_ctx: &mut EventCtx, data: &mut CalcState, _env: &Env| {
                        data.input.push_str(label);
                    },
                );
                row_layout.add_child(button.padding(5.0));
            }
        }
        button_layout.add_child(row_layout);
    }

    button_layout
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui())
        .title("Rust Calculator")
        .window_size((200.0, 460.0));

    let initial_state = CalcState {
        input: String::new(),
        output: String::from("0"),
        memory: String::new(), 
    };

    AppLauncher::with_window(main_window)
        .delegate(AppDelegate)
        .launch(initial_state)?;

    Ok(())
}

struct AppDelegate;

impl druid::AppDelegate<CalcState> for AppDelegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut CalcState,
        _env: &druid::Env,
    ) -> druid::Handled {
        if cmd.is(CALCULATE) {
            match Calculator::parse(&data.input) {
                Ok(tokens) => {
                    let expr = Calculator::expression(tokens);
                    if let Some(result) = Calculator::evaluate(expr) {
                        data.output = format!("{}", result);
                    } else {
                        data.output = String::from("Error");
                    }
                }
                Err(_) => {
                    data.output = String::from("Error");
                }
            }
            return druid::Handled::Yes;
        }
        druid::Handled::No
    }
}
