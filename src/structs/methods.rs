use clap::arg_enum;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Methods {
        RK4,
        RKF45
    }
}
