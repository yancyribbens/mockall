// vim: tw=80
//! Integration tests for mock!{}

use mockall_derive::*;

// Semiautomatic style mocking with associated types
mod associated_types_mock {
    use super::*;

    mock! {
        MyIter {}
        trait Iterator {
            type Item=u32;

            fn next(&mut self) -> Option<u32>;
        }
    }

    #[test]
    fn t() {
        let mut mock = MockMyIter::default();
        mock.expect_next()
            .returning(|_| Some(5));
        assert_eq!(5, mock.next().unwrap());
    }
}

/// Mock a struct whose definition is inaccessible
mod external_struct {
    use super::*;

    // A struct with a definition like this:
    // struct ExternalStruct {
    //     _x: i16
    // }
    // impl ExternalStruct {
    //     fn foo(&self, _x: u32) -> u32 {
    //         42
    //     }
    // }
    // Could be mocked like this:
    mock!{
        ExternalStruct {
            fn foo(&self, x: u32) -> u32;
        }
    }

    #[test]
    fn t() {
        let mut mock = MockExternalStruct::default();
        mock.expect_foo()
            .returning(|x| x + 1);
        assert_eq!(6, mock.foo(5));
    }
}

/// Use mock! to mock a generic struct
mod external_generic_struct {
    use super::*;

    // A struct with a definition like this:
    // pub struct ExtGenericStruct<T: Clone> {
    //     _x: i16
    // }
    // impl<T: Clone> ExtGenericStruct<T> {
    //     fn foo(&self, _x: T) -> T {
    //         42
    //     }
    // }
    // Could be mocked like this:
    mock!{
        pub ExtGenericStruct<T: Clone> {
            fn foo(&self, x: T) -> T;
        }
    }

    #[test]
    fn t() {
        let mut mock = MockExtGenericStruct::<u32>::default();
        mock.expect_foo()
            .returning(|x| x.clone());
        assert_eq!(5, mock.foo(5));
    }
}

mod external_struct_with_trait {
    use super::*;

    trait Bar {
        fn bar(&self, _x: u32) -> u32;
    }

    // A struct with a definition like this:
    // struct ExternalStruct {
    //     _x: i16
    // }
    // impl ExternalStruct {
    //     fn foo(&self, _x: u32) -> u32 {
    //         42
    //     }
    // }
    // impl Bar for ExternalStruct {
    //     fn bar(&self, _x: u32) -> u32 {
    //         42
    //     }
    // }
    //
    // Could be mocked like this:
    mock!{
        ExternalStruct {
            fn foo(&self, x: u32) -> u32;
        }
        trait Bar {
            fn bar(&self, _x: u32) -> u32;
        }
    }

    #[test]
    fn t() {
        let mut mock = MockExternalStruct::default();
        mock.expect_foo()
            .returning(|x| x + 1);
        mock.expect_bar()
            .returning(|x| x - 1);
        assert_eq!(6, mock.foo(5));
        assert_eq!(4, mock.bar(5));
    }
}

mod generic_method_returning_reference {
    use super::*;

    trait Foo {
        fn foo<T: 'static>(&self, t: T) -> &u32;
    }

    mock!{
        MyStruct {}
        trait Foo {
            fn foo<T: 'static>(&self, t: T) -> &u32;
        }
    }

    #[test]
    fn t() {
        let mut mock = MockMyStruct::default();
        mock.expect_foo::<i16>().return_const(5u32);
        assert_eq!(5u32, *mock.foo(99i16));
    }
}

/// Use mock! to mock a generic struct
mod generic_struct_with_generic_trait {
    use super::*;

    trait Foo<T> {
        fn foo(&self, x: T) -> T;
    }
    mock! {
        ExternalStruct<T, Z> {}
        trait Foo<T> {
            fn foo(&self, x: T) -> T;
        }
    }

    #[test]
    fn t() {
        let mut mock = MockExternalStruct::<u32, u64>::default();
        mock.expect_foo()
            .returning(|x| x);
        assert_eq!(5u32, mock.foo(5u32));
    }
}

mod inherited_trait {
    use super::*;

    trait A {
        fn foo(&self);
    }

    trait B: A {
        fn bar(&self);
    }

    mock!{
        B {}
        trait A {
            fn foo(&self);
        }
        trait B {
            fn bar(&self);
        }
    }

    #[test]
    fn t() {
        let mut mock = MockB::default();
        mock.expect_foo().returning(|_| ());
        mock.expect_bar().returning(|_| ());
        mock.foo();
        mock.bar();
    }
}

#[allow(unused)]
mod multi_trait {
    use super::*;

    trait A {}
    trait B {}
    mock!{
        MultiTrait {}
        trait A  {}
        trait B  {}
    }

    #[test]
    fn t() {
        fn foo<T: A + B>(_t: T) {}

        let mock = MockMultiTrait::default();
        foo(mock);
    }
}

mod reference_arguments {
    use super::*;

    mock!{
        Foo<'a> {
            fn foo(&self, x: &'a u32) -> u32;
        }
    }

    #[test]
    fn t() {
        const Y: u32 = 5;
        let mut mock = MockFoo::default();
        {
            mock.expect_foo().returning(|x| *x);
        }
        {
            let r = mock.foo(&Y);
            assert_eq!(5, r);
        }
    }
}

mod reference_return {
    use super::*;

    mock! {
        Foo {
            fn foo(&self) -> &u32;
        }
    }

    #[test]
    fn t() {
        let mut mock = MockFoo::default();
        mock.expect_foo()
            .return_const(5u32);
        assert_eq!(5, *mock.foo());
    }
}

mod ref_mut_return {
    use super::*;

    mock! {
        Foo {
            fn foo(&mut self) -> &mut u32;
        }
    }

    #[test]
    fn t() {
        let mut mock = MockFoo::default();
        mock.expect_foo()
            .return_var(5u32);
        {
            let r = mock.foo();
            assert_eq!(5, *r);
            *r = 6;
        }
        assert_eq!(6, *mock.foo());
    }
}
