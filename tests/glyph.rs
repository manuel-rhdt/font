#[test]
fn draw_letter() {
    use library::Curve::*;
    use library::Operation::*;

    let font = &::setup(None)[0];
    let glyph = font.case.draw('o').unwrap().unwrap();

    assert_eq!(&*glyph, &*vec![
        Move((274.0, 445.0)),
        Curve(Cubic((361.0, 445.0), (409.0, 371.0), (409.0, 236.0))),
        Curve(Cubic((409.0, 102.0), (361.0,  30.0), (274.0,  30.0))),
        Curve(Cubic((188.0,  30.0), (140.0, 102.0), (140.0, 236.0))),
        Curve(Cubic((140.0, 371.0), (188.0, 445.0), (274.0, 445.0))),
        Move((274.0, 491.0)),
        Curve(Cubic((159.0, 491.0), ( 45.0, 402.0), ( 45.0, 237.0))),
        Curve(Cubic(( 45.0,  71.0), (158.0, -15.0), (274.0, -15.0))),
        Curve(Cubic((391.0, -15.0), (504.0,  71.0), (504.0, 237.0))),
        Curve(Cubic((504.0, 402.0), (390.0, 491.0), (274.0, 491.0))),
    ]);
}

#[test]
fn draw_lowercase_letters() {
    let font = &::setup(None)[0];
    for code in b'a'..(b'z' + 1) {
        font.case.draw(code as char).unwrap().unwrap();
    }
}
