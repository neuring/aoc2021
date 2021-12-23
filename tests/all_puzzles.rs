use paste::paste;

macro_rules! gen_test {
    ($name:tt, $day:expr, $puzzle:expr, $expected:expr) => {
        paste! {
            #[test]
            fn [<test_ $day:lower _ $puzzle:lower _ $name:lower>]() -> anyhow::Result<()> {
                use aoc2021::{Day, Puzzle::*, Input, run_with_config};
                let res =
                    run_with_config(&Input {
                        input: format!("res/{}_{}.txt", paste!{stringify!([<d $day:lower>])}, stringify!($name)).into(),
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

gen_test! {small, 10, First, 26397}
gen_test! {main, 10, First, 392043}
gen_test! {small, 10, Second, 288957}
gen_test! {main, 10, Second, 1605968119}

gen_test! {small, 11, First, 1656}
gen_test! {main, 11, First, 1601}
gen_test! {small, 11, Second, 195}
gen_test! {main, 11, Second, 368}

gen_test! {small, 12, First, 10}
gen_test! {medium, 12, First, 19}
gen_test! {large, 12, First, 226}
gen_test! {main, 12, First, 3497}
gen_test! {small, 12, Second, 36}
gen_test! {medium, 12, Second, 103}
gen_test! {large, 12, Second, 3509}
gen_test! {main, 12, Second, 93686}

gen_test! {small, 13, First, 17}
gen_test! {main, 13, First, 942}
gen_test! {small, 13, Second, "\
#####
#...#
#...#
#...#
#####"
}
gen_test! {main, 13, Second, "\
..##.####..##..#..#..##..###..###..###.
...#....#.#..#.#..#.#..#.#..#.#..#.#..#
...#...#..#....#..#.#..#.#..#.#..#.###.
...#..#...#.##.#..#.####.###..###..#..#
#..#.#....#..#.#..#.#..#.#....#.#..#..#
.##..####..###..##..#..#.#....#..#.###."
}

gen_test! {small, 14, First, 1588}
gen_test! {main, 14, First, 2874}
gen_test! {small, 14, Second, 2188189693529u64}
gen_test! {main, 14, Second, 5208377027195u64}

gen_test! {small, 15, First, 40}
gen_test! {main, 15, First, 441}
gen_test! {small, 15, Second, 315}
gen_test! {main, 15, Second, 2849}

gen_test! {example_1, 16, First, 16}
gen_test! {example_2, 16, First, 12}
gen_test! {example_3, 16, First, 23}
gen_test! {example_4, 16, First, 31}
gen_test! {main, 16, First, 1007}

gen_test! {eval_1, 16, Second, 3}
gen_test! {eval_2, 16, Second, 54}
gen_test! {eval_3, 16, Second, 7}
gen_test! {eval_4, 16, Second, 9}
gen_test! {eval_5, 16, Second, 1}
gen_test! {eval_6, 16, Second, 0}
gen_test! {eval_7, 16, Second, 0}
gen_test! {eval_8, 16, Second, 1}
gen_test! {main, 16, Second, 834151779165u64}

gen_test! {small, 17, First, 45}
gen_test! {main, 17, First, 12090}
gen_test! {small, 17, Second, 112}
gen_test! {main, 17, Second, 5059}

gen_test! {small, 18, First, 4140}
gen_test! {main, 18, First, 4173}
gen_test! {small, 18, Second, 3993}
gen_test! {main, 18, Second, 4706}

gen_test! {small, 19, First, 79}
//gen_test! {main, 19, First, 425}
gen_test! {small, 19, Second, 3621}
//gen_test! {main, 19, Second, 13354}

gen_test! {small, 20, First, 35}
gen_test! {main, 20, First, 5765}
gen_test! {small, 20, Second, 3351}
gen_test! {main, 20, Second, 18509}

gen_test! {small, 21, First, 739785}
gen_test! {main, 21, First, 798147}
gen_test! {small, 21, Second, 444356092776315u64}
gen_test! {main, 21, Second, 809953813657517u64}

gen_test! {small, 22, First, 590784}
gen_test! {main, 22, First, 596989}
gen_test! {medium, 22, Second, 2758514936282235i64}
// gen_test! {main, 22, Second, 1160011199157381i64} Fairly slow...

gen_test! {small, 23, First, 12521}
//gen_test! {main, 23, First, 11516}
gen_test! {small_second, 23, Second, 44169}
gen_test! {main_second, 23, Second, 40272}
