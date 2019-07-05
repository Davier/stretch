#[test]
fn rounding_flex_basis_flex_grow_row_width_of_100() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]).unwrap();
    let node1 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]).unwrap();
    let node2 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]).unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 33f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 34f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 33f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 33f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 67f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 0f32);
}