use itertools::Itertools;
use slidy::{
    algorithm::{algorithm::Algorithm, as_slice::AsAlgorithmSlice},
    puzzle::{
        color_scheme::{Black, ColorScheme, Scheme},
        coloring::Rainbow,
        label::label::SplitSquareFringe,
        puzzle::Puzzle,
        render::{Renderer, RendererBuilder, Text},
        scrambler::{RandomState, Scrambler},
        sliding_puzzle::SlidingPuzzle,
    },
    solver::pdb4443::solver::Solver,
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    let solver = Solver::new();
    let renderer = RendererBuilder::with_dyn_scheme(Box::new(Scheme::new(
        SplitSquareFringe,
        Rainbow::default(),
    )))
    .text(Text::with_scheme(Box::new(Black) as Box<dyn ColorScheme>).font_size(15.0))
    .tile_size(30.0)
    .build();

    let main_table = get_element_by_id("main-table");
    let extras_table = get_element_by_id("extras-table");

    for i in 1..6 {
        let row = make_scramble_row(&i.to_string(), &solver, &renderer);
        main_table.append_child(&row).unwrap();
    }

    for i in 1..3 {
        let row = make_scramble_row(&format!("E{i}"), &solver, &renderer);
        extras_table.append_child(&row).unwrap();
    }
}

fn create_element(tag: &str) -> HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.create_element(tag).unwrap();
    element.dyn_into::<HtmlElement>().unwrap()
}

fn get_element_by_id(id: &str) -> HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.get_element_by_id(id).unwrap();
    element.dyn_into::<HtmlElement>().unwrap()
}

fn make_scramble_row(scramble_name: &str, solver: &Solver, renderer: &Renderer) -> HtmlElement {
    let mut state = Puzzle::default();
    RandomState.scramble(&mut state);

    let solution = solver.solve(&state).unwrap();
    let scramble = solution.inverse();

    let scramble_string = scramble
        .as_slice()
        .moves()
        .map(|m| {
            let mut s = m.to_string();
            if m.amount() == 1 {
                s.push_str(" ");
            }
            s
        })
        .chunks(10)
        .into_iter()
        .map(|mut chunk| chunk.join(" "))
        .join("\n");

    // Double check that the scramble generates the correct state
    {
        let mut check_state = Puzzle::default();
        let alg = scramble_string.parse::<Algorithm>().unwrap();
        check_state.apply_alg(&alg);
        assert_eq!(check_state, state, "scramble check failed");
    }

    let svg = renderer.render(&state).unwrap();

    let tr = create_element("tr");

    let col_num = create_element("td");
    col_num.set_class_name("col-num");
    col_num.set_inner_text(scramble_name);
    tr.append_child(&col_num).unwrap();

    let col_scramble = create_element("td");
    col_scramble.set_class_name("col-scramble");
    col_scramble.set_inner_text(&scramble_string);
    tr.append_child(&col_scramble).unwrap();

    let col_puzzle = create_element("td");
    col_puzzle.set_class_name("col-puzzle");
    col_puzzle.set_inner_html(&svg.to_string());
    tr.append_child(&col_puzzle).unwrap();

    tr
}
