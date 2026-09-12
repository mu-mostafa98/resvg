// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tests for building a tree programmatically (without XML input).

use tiny_skia_path::{PathBuilder, Rect};
use usvg::{Color, Fill, Group, Node, Paint, Path, Size, Transform, Tree};

#[test]
fn build_tree_programmatically() {
    // A 50x50 red rectangle at the origin.
    let rect = Rect::from_xywh(0.0, 0.0, 50.0, 50.0).unwrap();
    let data = PathBuilder::from_rect(rect);

    let fill = Fill::new(Paint::Color(Color::new_rgb(255, 0, 0)));
    let path = Path::new(
        String::new(),
        true,
        Some(fill),
        None,
        Default::default(),
        Default::default(),
        std::sync::Arc::new(data),
        Transform::default(),
    )
    .unwrap();

    let mut root = Group::empty();
    root.push_child(Node::Path(Box::new(path)));

    let mut tree = Tree::new(Size::from_wh(100.0, 100.0).unwrap(), root);
    tree.finalize();

    assert_eq!(tree.size(), Size::from_wh(100.0, 100.0).unwrap());

    let bbox = tree.root().bounding_box();
    assert!((bbox.width() - 50.0).abs() < 0.01);
    assert!((bbox.height() - 50.0).abs() < 0.01);

    assert_eq!(tree.root().children().len(), 1);
}

#[test]
fn build_tree_with_nested_group() {
    let rect = Rect::from_xywh(0.0, 0.0, 10.0, 10.0).unwrap();
    let data = PathBuilder::from_rect(rect);

    // A child group translated to (10, 20) containing a path.
    // Absolute transforms are propagated top-down by the builder, so the path
    // carries the accumulated transform (10, 20).
    let group_abs = Transform::from_translate(10.0, 20.0);
    let path = Path::new(
        String::new(),
        true,
        Some(Fill::new(Paint::Color(Color::black()))),
        None,
        Default::default(),
        Default::default(),
        std::sync::Arc::new(data),
        group_abs,
    )
    .unwrap();

    let mut child = Group::empty();
    child.transform = group_abs;
    child.abs_transform = group_abs;
    child.push_child(Node::Path(Box::new(path)));

    let mut root = Group::empty();
    root.push_child(Node::Group(Box::new(child)));

    let mut tree = Tree::new(Size::from_wh(100.0, 100.0).unwrap(), root);
    tree.finalize();

    // The group's object bbox is 10x10; its abs bbox is shifted to (10, 20).
    let abs = tree.root().abs_bounding_box();
    assert!((abs.x() - 10.0).abs() < 0.01);
    assert!((abs.y() - 20.0).abs() < 0.01);
    assert!((abs.width() - 10.0).abs() < 0.01);
    assert!((abs.height() - 10.0).abs() < 0.01);
}
