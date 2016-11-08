pub fn preprocess_input(input: &String) -> String {
    input.split_whitespace().collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use jd_grammar;
    use wtree;

    #[test]
    fn preprocess() {
        assert_eq!(preprocess_input("3 H + 4*(300 H + 200jg)".to_string()),
                "3H+4*(300H+200jg)");
    }

    #[test]
    fn single_step() {
        let r = jd_grammar::parse_Step("3E");
        let w = r.unwrap();
        assert_eq!(w.distance, 3.0);
        assert_eq!(w.time, (3 * 6 * 60) as f32);
    }

    #[test]
    fn single_step_workout() {
        let r = jd_grammar::parse_Workout_main("3L");
        let w = r.unwrap();
        assert_eq!(w.nodes.len(), 1);
    }


    fn multi_step_workout() {
        let r = jd_grammar::parse_Workout_main("4.8E+6.4M+1.6T+1.6M+3.2E");
    }

    #[test]
    fn multi_step_workout() {
        let r = jd_grammar::parse_Workout_main("3M+30minT");
        assert_eq!(r.unwrap().nodes.len(), 2);
        // TODO more checks
    }

    #[test]
    fn repeats() {
        let r = jd_grammar::parse_Workout_main("2I + 3*(1min H + 200 jg)");
        let w = r.unwrap();
        assert_eq!(w.nodes.len(), 2);
    }
}


// 3*(1E + 2*(1T + 1H))
// 2*(4*(3minT))
