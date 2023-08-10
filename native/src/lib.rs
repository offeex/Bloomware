pub mod modules;
pub mod logging;

#[macro_export]
macro_rules! kocklin_object {
    (
        $(
            [$package:ident] $name:ident {
                $(
                    fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) -> $ret:ty;
                )*
            }
        )*
    ) => {
        paste::paste! {
            use jni::JNIEnv;
            use jni::objects::JClass;
            $(
                $(
                    #[no_mangle]
                    pub extern "system" fn [<Java_ $package _ $name _ $method>](env: JNIEnv, class: JClass, $($arg: $arg_ty),*) -> $ret {
                        let result = $name::$method(env, class, $($arg),*);
                        result
                    }
                )*

                struct $name;

                trait [<$name Trait>] {
                    $(
                        fn $method(_env: JNIEnv, _class: JClass, $($arg: $arg_ty),*) -> $ret;
                    )*
                }
            )*
        }
    };
}
