pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]).unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50f32), ..Default::default() },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
