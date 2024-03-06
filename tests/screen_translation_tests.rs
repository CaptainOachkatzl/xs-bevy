use xs_games_rs::{
    two_dimendions::grid::{Position, RectSize, ScreenTranslation},
    ScreenView,
};

#[test]
fn out_of_bounds() {
    let translation = create_test_translation();
    assert!(translation.get_grid_position(40., 200.).is_none()); // y outer bounds
    assert!(translation.get_grid_position(9., 70.).is_none()); // x inner bounds
}

#[test]
fn minimal_edge_case() {
    let translation = create_test_translation();
    assert_eq!(translation.get_grid_position(10., 50.).unwrap(), Position::new(0, 0));
}

#[test]
fn maximum_edge_case() {
    let translation = create_test_translation();
    assert_eq!(translation.get_grid_position(109., 149.).unwrap(), Position::new(9, 9));
}

#[test]
fn negative_values() {
    let translation = create_test_translation_negative();
    assert_eq!(translation.get_grid_position(0., 0.).unwrap(), Position::new(5, 5));
    assert_eq!(translation.block_center_to_screen_position(0, 0), (-45., -45.));
}

fn create_test_translation() -> ScreenTranslation {
    let screen_view = ScreenView {
        offset_x: 10.,
        offset_y: 50.,
        width: 100.,
        height: 100.,
    };
    let logical_size = RectSize { width: 10, height: 10 };
    ScreenTranslation::new(screen_view, logical_size)
}

fn create_test_translation_negative() -> ScreenTranslation {
    let screen_view = ScreenView {
        offset_x: -50.,
        offset_y: -50.,
        width: 100.,
        height: 100.,
    };
    let logical_size = RectSize { width: 10, height: 10 };
    ScreenTranslation::new(screen_view, logical_size)
}
