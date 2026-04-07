mod vector;

#[cfg(test)]
mod tests {
    use crate::vector::vector2::Vector2;

    #[test]
    fn test_add() {
        let v1 = Vector2::default();
        let v2 = Vector2::new(3.0f32, 4.0);

        let v3 = v1 + v2;
        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 4.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector2 { x: 10i64, y: 10i64 };
        let v2 = Vector2 { x: 5i64, y: 2i64 };

        let v3 = v1 - v2;
        assert_eq!(v3.x, 5);
        assert_eq!(v3.y, 8);
    }

    #[test]
    fn test_mul() {
        let v1 = Vector2 { x: 2, y: 5 };
        let v2 = Vector2 { x: 3, y: 4 };

        let v3 = v1 * v2;
        assert_eq!(v3.x, 6);
        assert_eq!(v3.y, 20);
    }

    #[test]
    fn test_div() {
        let v1 = Vector2 { x: 6, y: 10 };
        let v2 = Vector2 { x: 2, y: 5 };

        let v3 = v1 / v2;
        assert_eq!(v3.x, 3);
        assert_eq!(v3.y, 2);
    }

    #[test]
    fn test_rem() {
        let v1 = Vector2 { x: 7, y: 11 };
        let v2 = Vector2 { x: 2, y: 5 };

        let v3 = v1 % v2;
        assert_eq!(v3.x, 1);
        assert_eq!(v3.y, 1);
    }
}
