#[macro_export]
macro_rules! html {
    ($tag:literal) => {
        $crate::server::html::Html::new($tag)
    };
}
#[macro_export]
macro_rules! div {
    () => {
        $crate::server::html::Html::new("div")
    };
}

#[macro_export]
macro_rules! p {
    () => {
        $crate::server::html::Html::new("p")
    };
}

#[macro_export]
macro_rules! h1 {
    () => {
        $crate::server::html::Html::new("h1")
    };
}

#[macro_export]
macro_rules! button {
    () => {
        $crate::server::html::Html::new("button")
    };
}
