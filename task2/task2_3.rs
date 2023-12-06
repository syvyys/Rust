// task 3
use std::fmt::Debug;
use std::cmp::{min, max};
use std::ops::{Add, Div, Sub};

#[derive(Debug, PartialEq)]
struct Rectangle<T: Sized> {
    x: T,
    y: T,
    width: T,
    height: T
}

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T
}

trait Rect<T: Default + Sized> : Default {
    fn new(x: T, y : T, width: T, height: T) -> Self;
    fn adjust(&mut self, dx1: T, dy1: T, dx2: T, dy2: T);
    fn left(&self) -> T;
    fn right(&self) -> T;
    fn top(&self) -> T;
    fn bottom(&self) -> T;
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn height(&self) -> T;
    fn width(&self) -> T;
    fn bottom_left(&self) -> Point<T>;
    fn bottom_right(&self) -> Point<T>;
    fn top_left(&self) -> Point<T>;
    fn top_right(&self) -> Point<T>;
    fn center(&self) -> Point<T>;
    fn contains_point(&self, point: &Point<T>) -> bool;
    fn contains_rect(&self, point: &Rectangle<T>) -> bool;
    fn is_intersected(&self, rect: &Rectangle<T>) -> bool;
    fn intersected(&self, rect: &Rectangle<T>) -> Rectangle<T>;
    fn united(&self, rect: &Rectangle<T>) -> Rectangle<T>;
    fn transposed(&self) -> Rectangle<T>;
}

impl<T: Default + Sized> Default for Rectangle<T> {
    fn default() -> Self {
        return Rectangle {
            x: T::default(),
            y: T::default(),
            width: T::default(),
            height: T::default()
        };
    }
}

impl<T: Default + Debug + Copy + Ord + Add<Output = T> + Sub<Output = T> + Div<Output = T>> Rect<T> for Rectangle<T> {
    fn new(x: T, y: T, width: T, height: T) -> Self {
        return Rectangle { x, y, width, height };
    }

    fn adjust(&mut self, dx1: T, dy1: T, dx2: T, dy2: T) {
        self.x = self.x + dx1;
        self.y = self.y + dy1;
        self.width = self.width + dx2;
        self.height = self.height + dy2;
    }

    fn left(&self) -> T {
        return self.x;
    }

    fn right(&self) -> T {
        return self.x + self.width;
    }

    fn top(&self) -> T {
        return self.y + self.height;
    }

    fn bottom(&self) -> T {
        return self.y;
    }

    fn x(&self) -> T {
        return self.x;
    }

    fn y(&self) -> T {
        return self.y;
    }

    fn height(&self) -> T {
        return self.height;
    }

    fn width(&self) -> T {
        return self.width;
    }

    fn bottom_left(&self) -> Point<T> {
        return Point {
            x: self.left(),
            y: self.bottom()
        };
    }

    fn bottom_right(&self) -> Point<T> {
        return Point {
            x: self.right(),
            y: self.bottom()
        };
    }

    fn top_left(&self) -> Point<T> {
        return Point {
            x: self.left(),
            y: self.top()
        };
    }

    fn top_right(&self) -> Point<T> {
        return Point {
            x: self.right(),
            y: self.top()
        };
    }

    fn center(&self) -> Point<T> {
        todo!()
        // return Point {
        //     x: self.x + self.width / 2,
        //     y: self.y + self.height / 2
        // };
    }

    fn contains_point(&self, point: &Point<T>) -> bool {
        return self.x <= point.x && point.x <= self.x + self.width
            && self.y <= point.y && point.y <= self.y + self.height;
    }

    fn contains_rect(&self, rect: &Rectangle<T>) -> bool {
        let has_top_left: bool = self.contains_point(&rect.top_left());
        let has_top_right: bool = self.contains_point(&rect.top_right());
        let has_bottom_left: bool = self.contains_point(&rect.bottom_left());
        let has_bottom_right: bool = self.contains_point(&rect.bottom_right());
        return has_top_left && has_top_right && has_bottom_left && has_bottom_right;
    }

    fn is_intersected(&self, rect: &Rectangle<T>) -> bool {
        // Is fully inside
        if self.contains_rect(rect) || rect.contains_rect(self) {
            return true;
        }

        let has_top_left: bool = self.contains_point(&rect.top_left());
        let has_top_right: bool = self.contains_point(&rect.top_right());
        let has_bottom_left: bool = self.contains_point(&rect.bottom_left());
        let has_bottom_right: bool = self.contains_point(&rect.bottom_right());

        // Is fully outside
        if !has_top_left && !has_top_right && !has_bottom_right && !has_bottom_left {
            return false;
        }
        return true;
    }

    fn intersected(&self, rect: &Rectangle<T>) -> Rectangle<T> {
        if !self.is_intersected(rect) {
            return Rectangle::default();
        }

        let top = min(self.top(), rect.top());
        let left = max(self.left(), rect.left());
        let right = min(self.right(), rect.right());
        let bottom = max(self.bottom(), rect.bottom());
        return Rectangle {
            x: left,
            y: bottom,
            width: right - left,
            height: top - bottom
        };
    }

    fn united(&self, rect: &Rectangle<T>) -> Rectangle<T> {
        let top = max(self.top(), rect.top());
        let left = min(self.left(), rect.left());
        let right = max(self.right(), rect.right());
        let bottom = min(self.bottom(), rect.bottom());

        return Rectangle {
            x: left,
            y: bottom,
            width: right - left,
            height: top - bottom
        };
    }

    fn transposed(&self) -> Rectangle<T> {
        return Rectangle {
            x: self.x,
            y: self.y,
            width: self.height,
            height: self.width
        };
    }
}

#[test]
fn test_parameters() {
    let rect: Rectangle<i32> = Rectangle {x: 10, y: 20, width: 50, height: 45};

    assert_eq!(rect.x(), 10);
    assert_eq!(rect.y(), 20);
    assert_eq!(rect.width(), 50);
    assert_eq!(rect.height(), 45);
}

#[test]
fn test_borders() {
    let rect: Rectangle<i32> = Rectangle {x: 10, y: 20, width: 50, height: 45};

    assert_eq!(rect.left(), 10);
    assert_eq!(rect.right(), 60);
    assert_eq!(rect.top(), 65);
    assert_eq!(rect.bottom(), 20);

    assert_eq!(rect.top_left(), Point {x: 10, y: 65});
    assert_eq!(rect.top_right(), Point {x: 60, y: 65});
    assert_eq!(rect.bottom_left(), Point {x: 10, y: 20});
    assert_eq!(rect.bottom_right(), Point {x: 60, y: 20});
}

#[test]
fn test_adjust() {
    let mut rect: Rectangle<i32> = Rectangle {x: 10, y: 20, width: 50, height: 45};

    assert_eq!(Rectangle::new(10, 20, 50, 45), rect);
    rect.adjust(-10, -10, 10, 5);
    assert_eq!(Rectangle::new(0, 10, 60, 50), rect);
    rect.adjust(0, 0, 0, 0);
    assert_eq!(Rectangle::new(0, 10, 60, 50), rect);
}

#[test]
fn test_intersections() {
    let rect: Rectangle<i32> = Rectangle {x: 0, y: 10, width: 60, height: 50};

    assert_eq!(rect.contains_point(&Point {x: 10, y: 20}), true);
    assert_eq!(rect.contains_point(&Point {x: 0, y: 10}), true);
    assert_eq!(rect.contains_point(&Point {x: 100, y: -10}), false);
    assert_eq!(rect.contains_point(&Point {x: 0, y: 0}), false);

    assert_eq!(rect.contains_rect(&Rectangle {x: 30, y: 30, width: 10, height: 10}),
               true); // fully inside
    assert_eq!(rect.contains_rect(&Rectangle {x: 500, y: 20, width: 10, height: 10}),
               false); // fully outside
    assert_eq!(rect.contains_rect(&Rectangle {x: 30, y: 30, width: 100, height: 100}),
               false); // half inside
    assert_eq!(rect.contains_rect(&Rectangle {x: 0, y: 10, width: 60, height: 50}),
               true); // same
    assert_eq!(rect.contains_rect(&Rectangle {x: 0, y: 10, width: 60, height: 10}),
               true); // same side

    assert_eq!(rect.is_intersected(&Rectangle {x: 30, y: 30, width: 10, height: 10}),
               false); // fully inside
    assert_eq!(rect.is_intersected(&Rectangle {x: 500, y: 20, width: 10, height: 10}),
               false); // fully outside
    assert_eq!(rect.is_intersected(&Rectangle {x: 30, y: 30, width: 100, height: 100}),
               true); // half inside
    assert_eq!(rect.is_intersected(&Rectangle {x: 0, y: 10, width: 60, height: 50}),
               false); // same
    assert_eq!(rect.is_intersected(&Rectangle {x: 0, y: 10, width: 60, height: 10}),
               false); // same side inside
    assert_eq!(rect.is_intersected(&Rectangle {x: 60, y: 10, width: 10, height: 10}),
               true); // same side inside
}

#[test]
fn test_operations() {
    let rect: Rectangle<i32> = Rectangle {x: 10, y: 20, width: 50, height: 45};

    assert_eq!(rect.transposed(), Rectangle {x: 10, y: 20, width: 45, height: 50});

    assert_eq!(rect.intersected(&Rectangle {x: 10, y: 20, width: 50, height: 45}),
               Rectangle {x: 10, y: 20, width: 50, height: 45}); // same
    assert_eq!(rect.intersected(&Rectangle {x: 20, y: 30, width: 10, height: 10}),
               Rectangle {x: 20, y: 30, width: 10, height: 10}); // rect fully inside
    assert_eq!(rect.intersected(&Rectangle {x: 0, y: 0, width: 100, height: 100}),
               Rectangle {x: 10, y: 20, width: 50, height: 45}); // rect fully inside
    assert_eq!(rect.intersected(&Rectangle {x: 40, y: 40, width: 50, height: 50}),
               Rectangle {x: 40, y: 40, width: 20, height: 25}); // half inside
    assert_eq!(rect.intersected(&Rectangle {x: 100, y: 20, width: 50, height: 45}),
               Rectangle {x: 0, y: 0, width: 0, height: 0}); // fully outside

    assert_eq!(rect.united(&Rectangle {x: 10, y: 20, width: 50, height: 45}),
               Rectangle {x: 10, y: 20, width: 50, height: 45}); // same
    assert_eq!(rect.united(&Rectangle {x: 20, y: 30, width: 10, height: 10}),
               Rectangle {x: 10, y: 20, width: 50, height: 45}); // rect fully inside
    assert_eq!(rect.united(&Rectangle {x: 0, y: 0, width: 100, height: 100}),
               Rectangle {x: 0, y: 0, width: 100, height: 100}); // rect fully inside
    assert_eq!(rect.united(&Rectangle {x: 40, y: 40, width: 50, height: 50}),
               Rectangle {x: 10, y: 20, width: 80, height: 70}); // half inside
    assert_eq!(rect.united(&Rectangle {x: 100, y: 20, width: 50, height: 50}),
               Rectangle {x: 10, y: 20, width: 140, height: 50}); // fully outside
}

fn main() {
    let rect: Rectangle<i32> = Rectangle {x: 10, y: 15, width: 5, height: 8};
    println!("{:?}", rect);
}
