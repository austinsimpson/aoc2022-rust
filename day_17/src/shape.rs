pub struct Shape {
    data: [[bool; 4]; 4],
}

pub const SHAPES: [Shape; 5] = [
    Shape {
        data: [
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    },
    Shape {
        data: [
            [false, true, false, false],
            [true, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
    },
    Shape {
        data: [
            [false, false, true, false],
            [false, false, true, false],
            [true, true, true, false],
            [false, false, false, false],
        ],
    },
    Shape {
        data: [
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
        ],
    },
    Shape {
        data: [
            [true, true, false, false],
            [true, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    },
];
