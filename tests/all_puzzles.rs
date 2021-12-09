use paste::paste;

macro_rules! gen_test {
    ($name:tt, $day:expr, $puzzle:expr, $expected:expr) => {
        paste! {
            #[test]
            fn [<test_ $day:lower _ $puzzle:lower _ $name:lower>]() -> anyhow::Result<()> {
                use aoc2021::{Day, Puzzle::*, Input, run_with_config};
                let res =
                    run_with_config(&Input {
                        input: format!("res/{}_{}", paste!{stringify!([<d $day:lower>])}, stringify!($name)).into(),
                        day: Day::new($day),
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

gen_test! {small, 01, First, 7}
gen_test! {small, 01, Second, 5}
gen_test! {main, 01, First, 1766}
gen_test! {main, 01, Second, 1797}

gen_test! {small, 02, First, 150}
gen_test! {small, 02, Second, 900}
gen_test! {main, 02, First, 1507611}
gen_test! {main, 02, Second, 1880593125}

gen_test! {small, 03, First, 198}
gen_test! {small, 03, Second, 230}
gen_test! {main, 03, First, 1071734}
gen_test! {main, 03, Second, 6124992}

gen_test! {small, 04, First, 4512}
gen_test! {main, 04, First, 60368}
gen_test! {small, 04, Second, 1924}
gen_test! {main, 04, Second, 17435}

gen_test! {small, 05, First, 5}
gen_test! {main, 05, First, 5585}
gen_test! {small, 05, Second, 12}
gen_test! {main, 05, Second, 17193}

gen_test! {small, 06, First, 5934}
gen_test! {main, 06, First, 345387}
gen_test! {small, 06, Second, 26984457539usize}
gen_test! {main, 06, Second, 1574445493136usize}

gen_test! {small, 07, First, 37}
gen_test! {main, 07, First, 355989}
gen_test! {small, 07, Second, 168}
gen_test! {main, 07, Second, 102245489}

gen_test! {small, 08, First, 26}
gen_test! {main, 08, First, 479}
gen_test! {small, 08, Second, 61229}
gen_test! {main, 08, Second, 1041746}

gen_test! {small, 09, First, 15}
gen_test! {main, 09, First, 588}
gen_test! {small, 09, Second, 1134}
gen_test! {main, 09, Second, 964712}
