pub mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}
    pub mod inside {
        pub fn inner_function() {}
        fn secret_function() {}
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
