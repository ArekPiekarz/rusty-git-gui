use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap)]
pub struct Color(pub &'static str);