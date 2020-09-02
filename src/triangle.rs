pub(crate) struct Triangle {
    side_a: i32,
    side_b: i32,
    side_c: i32,
}

impl Triangle {
    // constructor that takes three arguments
    // pub crate makes this visible only within this crate.
    pub(crate) fn new(s1: i32, s2: i32, s3: i32) -> Triangle {
        Triangle {
            side_a: s1,
            side_b: s2,
            side_c: s3,
        }
    }

    // Setters for triangle
    pub(crate) fn get_side_one(&self) -> i32 {
        self.side_a
    }

    pub(crate) fn get_side_two(&self) -> i32 {
        self.side_b
    }
    pub(crate) fn get_side_three(&self) -> i32 {
        self.side_c
    }
    // getters for triangle
    pub(crate) fn set_side_one(&mut self, side: i32) {
        self.side_a = side
    }
    pub(crate) fn set_side_two(&mut self, side: i32) {
        self.side_b = side
    }
    pub(crate) fn set_side_three(&mut self, side: i32) {
        self.side_c = side
    }

    // checks if all sides are equal
    pub(crate) fn is_equilateral(&self) -> bool {
        self.side_a == self.side_b && self.side_b == self.side_c
    }

    // checks if no sides are equal
    pub(crate) fn is_scalene(&self) -> bool {
        self.side_a != self.side_b && self.side_b != self.side_c && self.side_a != self.side_c
    }

    // checks if two sides are equal
    pub(crate) fn is_isosceles(&self) -> bool {
        self.side_a == self.side_b || self.side_b == self.side_c || self.side_a == self.side_c
    }

    // checks if is a right triangle
    pub(crate) fn is_right(&self) -> bool {
        // builds an array and sorts it
        let mut array_sides: [i32; 3] = [self.side_a, self.side_b, self.side_c];
        array_sides.sort();
        // if the sum of the squares of the first two sides equals the square of the third
        (array_sides[0] ^ 2 + array_sides[1] ^ 2) == (array_sides[2] ^ 2)
    }
}
