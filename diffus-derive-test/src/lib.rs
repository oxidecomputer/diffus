#[cfg(test)]
#[allow(unused_variables)]
#[allow(dead_code)]
mod test {
    use diffus_derive::{
        Diffus,
    };

    use diffus::{
        self,
        Diffable,
    };

    #[derive(Diffus)]
    enum NestedTest {
        T { test: Test },
    }

    #[derive(Debug, Diffus, PartialEq, Eq)]
    enum Test {
        A,
        B(String),
        Bd(String, u32),
        C { x: u32 },
        Cd { x: u32, y: String },
    }

    mod visibility_test {
        /*
         * To verify that the visibility of the Edited version of the object
         * is inherited correctly and doesn't cause problems.
         */
        use diffus_derive::Diffus;

        #[derive(Diffus)]
        pub struct VisTestStructUnit;

        #[derive(Diffus)]
        pub struct VisTestStructTuple(u32);

        #[derive(Diffus)]
        pub struct VisTestStruct { x: u32 }

        #[derive(Diffus)]
        pub enum VisTestEnum {
            A,
            B(u32),
            C { x: u32 },
        }
    }

    #[test]
    fn enm_nested_test() {
        let left = NestedTest::T {
            test: Test::C { x: 32 },
        };
        let right = NestedTest::T {
            test: Test::C { x: 43 },
        };

        let diff = left.diff(&right);

        if let diffus::edit::enm::Edit::AssociatedChanged(EditedNestedTest::T { test }) = diff.change().unwrap() {
            if let diffus::edit::enm::Edit::AssociatedChanged(EditedTest::C { x }) = test.change().unwrap() {
                assert_eq!(
                    x.change(),
                    Some(&(&32, &43))
                );
            }else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    }


    #[test]
    fn enm_associated_not_change_tuple_variant() {
        let left = Test::Bd(
            "Bilbo Baggins".to_owned(),
            42,
        );
        let right = Test::Bd(
            "Bilbo Baggins".to_owned(),
            42,
        );

        assert!(left.diff(&right).is_copy());
    }

    #[test]
    fn enm_associated_not_change() {
        let left = Test::Cd {
            x: 42,
            y: "Bilbo Baggins".to_owned(),
        };
        let right = Test::Cd {
            x: 42,
            y: "Bilbo Baggins".to_owned(),
        };

        assert!(left.diff(&right).is_copy());
    }

    #[test]
    fn enm_associated_change() {
        let left = Test::Cd {
            x: 42,
            y: "Bilbo Baggins".to_owned(),
        };
        let right = Test::Cd {
            x: 42,
            y: "Frodo Baggins".to_owned(),
        };
        if let diffus::edit::Edit::Change(diffus::edit::enm::Edit::AssociatedChanged(EditedTest::Cd { x, y })) = left.diff(&right) {
            assert!(x.is_copy());
            assert!(y.is_change());
        } else {
            unreachable!()
        }
    }

    #[test]
    fn enm_variant_change() {
        let left = Test::Cd {
            x: 42,
            y: "Bilbo Baggins".to_owned(),
        };
        let right = Test::B("Frodo Baggins".to_owned());
        if let diffus::edit::Edit::Change(diffus::edit::enm::Edit::VariantChanged(l, r)) = left.diff(&right) {
            assert_eq!(&left, l);
            assert_eq!(&right, r);
        } else {
            unreachable!()
        }
    }

    #[derive(Diffus, Debug)]
    struct Inner {
        x: String,
        y: u32,
    }

    #[derive(Diffus)]
    struct Unit;

    #[derive(Diffus, Debug)]
    struct Unnamed(u32, String);

    #[derive(Diffus, Debug)]
    struct Outer {
        inner: Inner,
        lit: i32,
    }

    #[test]
    fn nested() {
        let left = Outer {
            inner: Inner {
                x: "x".to_owned(),
                y: 13,
            },
            lit: 3,
        };
        let right = Outer {
            inner: Inner {
                x: "x".to_owned(),
                y: 37,
            },
            lit: 3,
        };

        let diff = left.diff(&right);

        assert_eq!(
            diff.change().unwrap()
                .inner.change().unwrap()
                .y.change().unwrap(),
            &(&13, &37)
        );

    }
}
