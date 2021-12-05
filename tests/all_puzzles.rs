use paste::paste;

macro_rules! gen_test {
    ($name:tt, $day:expr, $puzzle:expr, $expected:expr) => {
        paste! {
            #[test]
            fn [<test_ $day:lower _ $puzzle:lower _ $name:lower>]() -> anyhow::Result<()> {
                use aoc2021::{Day::*, Puzzle::*, Input, run_with_config};
                let res =
                    run_with_config(&Input {
                        input: format!("res/{}_{}", paste!{stringify!([<$day:lower>])}, stringify!($name)).into(),
                        day: $day,
                        puzzle: $puzzle,
                    })?;
                assert_eq!(
                    res,
                    $expected.to_string(),
                );
                Ok(())
            }
        }
    };
}

gen_test! {small, D01, First, 7}
gen_test! {small, D01, Second, 5}
gen_test! {main, D01, First, 1766}
gen_test! {main, D01, Second, 1797}

gen_test! {small, D02, First, 150}
gen_test! {small, D02, Second, 900}
gen_test! {main, D02, First, 1507611}
gen_test! {main, D02, Second, 1880593125}

gen_test! {small, D03, First, 198}
gen_test! {small, D03, Second, 230}
gen_test! {main, D03, First, 1071734}
gen_test! {main, D03, Second, 6124992}

gen_test! {small, D04, First, 4512}
gen_test! {main, D04, First, 60368}
gen_test! {small, D04, Second, 1924}
gen_test! {main, D04, Second, 17435}

gen_test! {small, D05, First, 5}
gen_test! {main, D05, First, 5585}
gen_test! {small, D05, Second, 12}
gen_test! {main, D05, Second, 17193}
