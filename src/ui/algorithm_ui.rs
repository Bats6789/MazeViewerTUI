use ratatui::{
    layout::{Constraint, Layout, Rect}, style::{Style, Stylize}, text::Span, widgets::{Block, List}, Frame
};

use crate::app::{App, AlgorithmSetting};

pub fn algorithm_ui(f: &mut Frame, algorithm_layout: Rect, app: &mut App) {
    let layout =
        Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Min(0)]).split(algorithm_layout);

    let generator_layout = layout[0];
    let solver_layout = layout[1];

    let items: Vec<String> = app.gen_algo_lookup.iter().map(|el| el.get_name()).collect();

    let default_style = Style::default().fg(app.default_color);
    let highlight_style = Style::default().fg(app.highlight_fg).bg(app.highlight_bg);

    let mut gen_text = Span::from("Generator");
    let mut gen_style = default_style;
    let mut gen_highlight_style = highlight_style;

    let mut solve_text = Span::from("Solver");
    let mut solve_style = default_style;
    let mut solve_highlight_style = highlight_style;

    match app.algorithm_setting {
        AlgorithmSetting::Generator => {
            gen_text = gen_text.style(Style::default().underlined());
            solve_style = solve_style.dim();
            solve_highlight_style = solve_highlight_style.dim();
        }
        AlgorithmSetting::Solver => {
            solve_text = solve_text.style(Style::default().underlined());
            gen_style = gen_style.dim();
            gen_highlight_style = gen_highlight_style.dim();
        }
    }


    let generator_display = List::new(items)
        .block(Block::bordered().title(gen_text))
        .style(gen_style)
        .highlight_style(gen_highlight_style);

    let items: Vec<String> = app.solve_algo_lookup.iter().map(|el| el.get_name()).collect();

    let solver_display = List::new(items)
        .block(Block::bordered().title(solve_text))
        .style(solve_style)
        .highlight_style(solve_highlight_style);

    f.render_stateful_widget(generator_display, generator_layout, &mut app.gen_list_state);
    f.render_stateful_widget(solver_display, solver_layout, &mut app.solve_list_state);
}
