use crate::tbox_item::TBI;

/*
   this type englobe all type of rules for tbox items
   take a number of items and generates a new if possible
*/
pub type TbRule = fn(Vec<&TBI>) -> Option<TBI>;

// for the moment we only implement for dl_lite

//--------------------------------------------------------------------------------------------------
pub fn dl_lite_rule_one(vec: Vec<&TBI>) -> Option<TBI> {
    /*
    negation rule
    A=>notB then B=>notA
     */
    if vec.len() != 1 {
        Option::None
    } else {
        let tbi = vec[0];

        if tbi.rside().is_negated() {
            tbi.reverse_negation()
        } else {
            Option::None
        }
    }
}

pub fn dl_lite_rule_two(vec: Vec<&TBI>) -> Option<TBI> {
    /*
    chain rule
    A=>B and B=>C then A=>C
     */
    if vec.len() != 2 {
        Option::None
    } else {
        let tbi1 = vec[0];
        let tbi2 = vec[1];

        if tbi1.rside() == tbi2.lside() {
            TBI::new(tbi1.lside().clone(), tbi2.rside().clone())
        } else {
            Option::None
        }
    }
}

pub fn dl_lite_rule_three(vec: Vec<&TBI>) -> Option<TBI> {
    Option::None
}

pub fn dl_lite_rule_four(vec: Vec<&TBI>) -> Option<TBI> {
    Option::None
}

pub fn dl_lite_rule_five(vec: Vec<&TBI>) -> Option<TBI> {
    Option::None
}

pub fn dl_lite_rule_six(vec: Vec<&TBI>) -> Option<TBI> {
    Option::None
}

pub fn dl_lite_rule_seven(vec: Vec<&TBI>) -> Option<TBI> {
    Option::None
}

pub fn dl_lite_rule_eight(vec: Vec<&TBI>) -> Option<TBI> {
    Option::None
}
