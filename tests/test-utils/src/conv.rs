use spl_token::solana_program::program_option::COption;

pub fn is_opt_eq_copt<T: PartialEq>(us: Option<T>, sol: COption<T>) -> bool {
    match (us, sol) {
        (None, COption::None) => true,
        (Some(us), COption::Some(sol)) => us == sol,
        _ => false,
    }
}
