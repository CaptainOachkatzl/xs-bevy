use xs_games_rs::two_dimendions::grid::{translation::screen_translation::ScreenTranslation, Field, Position, Size2D};

#[test]
fn out_of_bounds() {
    let translation = create_test_translation();
    assert!(translation.get_logical_position(40, 200).is_none()); // y outer bounds
    assert!(translation.get_logical_position(9, 70).is_none()); // x inner bounds
}

#[test]
fn minimal_edge_case() {
    let translation = create_test_translation();
    assert_eq!(translation.get_logical_position(10, 50).unwrap(), Position::new(0, 0));
}

#[test]
fn maximum_edge_case() {
    let translation = create_test_translation();
    assert_eq!(translation.get_logical_position(109, 149).unwrap(), Position::new(9, 9));
}

fn create_test_translation() -> ScreenTranslation {
    let screen_view = Field {
        offset: Position::new(10, 50),
        size: Size2D { height: 100, width: 100 },
    };
    let logical_size = Size2D { width: 10, height: 10 };
    ScreenTranslation::new(screen_view, logical_size)
}
