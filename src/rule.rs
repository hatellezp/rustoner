use crate::node::Node;
use crate::tbox_item::TBI;
use crate::types::DLType;

/*
   this type englobe all type of rules for tbox items
   take a number of items and generates a new if possible
*/
pub type TbRule = fn(Vec<&TBI>) -> Option<Vec<TBI>>;

// for the moment we only implement for dl_lite

//--------------------------------------------------------------------------------------------------
pub fn dl_lite_rule_one(vec: Vec<&TBI>) -> Option<Vec<TBI>> {
    /*
    negation rule
    A=>notB then B=>notA
     */
    if vec.len() != 1 {
        Option::None
    } else {
        let tbi = vec[0];

        if tbi.rside().is_negated() {
            Some(vec![tbi.reverse_negation().unwrap()])
        } else {
            Option::None
        }
    }
}

pub fn dl_lite_rule_two(vec: Vec<&TBI>) -> Option<Vec<TBI>> {
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
            Some(vec![
                TBI::new(tbi1.lside().clone(), tbi2.rside().clone()).unwrap()
            ])
        } else {
            Option::None
        }
    }
}

// third rule: r1=>r2 and B=>notExists_r2 then B=>notExists_r1 and Exists_r1=>notB
pub fn dl_lite_rule_three(vec: Vec<&TBI>) -> Option<Vec<TBI>> {
    // maybe matches are super cool but here will be more complicated, Boxes are complicated
    if vec.len() != 2 {
        Option::None
    } else {
        let tbi1 = vec[0];
        let tbi2 = vec[1];

        // verifies you have roles
        if !DLType::all_roles(tbi1.lside().t(), tbi1.rside().t()) {
            Option::None
        } else {
            // two time because is second child we want

            // I will use recursive child
            let tbi2_rside_second_child = Node::child_r(Some(tbi2.lside()), 2);
            // this old call works but I'm using the recursive version
            // let tbi2_rside_second_child = Node::child(Node::child(Some(tbi2.lside())));

            if (&tbi2_rside_second_child).is_some() {
                // this big ass condition verfies that rside of tbi2 is of the correct form
                if tbi2.rside().t() == DLType::NegatedConcept
                    && (Node::child(Some(tbi2.rside())).unwrap().t() == DLType::ExistsConcept)
                {
                    if tbi2_rside_second_child.unwrap() == tbi1.rside() {
                        let exists_r1 = tbi1.lside().clone().exists().unwrap();
                        let not_exists_r1 = (&exists_r1).clone();

                        let new_tbi1 = TBI::new(tbi2.lside().clone(), not_exists_r1).unwrap();
                        let new_tbi2 = TBI::new(exists_r1, tbi2.lside().clone().negate()).unwrap();

                        Some(vec![new_tbi1, new_tbi2])
                    } else {
                        Option::None
                    }
                } else {
                    Option::None
                }
            } else {
                Option::None
            }
        }
    }
}

// fourth rule: r1=>r2 and B=>notExists_r2_inv then B=>notExists_r1_inv
pub fn dl_lite_rule_four(vec: Vec<&TBI>) -> Option<Vec<TBI>> {
// maybe matches are super cool but here will be more complicated, Boxes are complicated
    if vec.len() != 2 {
        Option::None
    } else {
        let tbi1 = vec[0];
        let tbi2 = vec[1];

        // verifies you have roles
        if !DLType::all_roles(tbi1.lside().t(), tbi1.rside().t()) {
            Option::None
        } else {
            // two time because is second child we want
            let tbi2_rside_third_child = Node::child_r(Some(tbi2.lside()), 3);

            if (&tbi2_rside_third_child).is_some() {
                let tbi2_rside = tbi2.rside();
                // this big ass condition verfies that rside to tbi2 is of the correct form
                if tbi2_rside.t() == DLType::NegatedConcept
                    && Node::child_r(Some(tbi2_rside), 1).unwrap().t() == DLType::ExistsConcept
                    && Node::child_r(Some(tbi2_rside), 2).unwrap().t() == DLType::InverseRole
                {
                    if tbi2_rside_third_child.unwrap() == tbi1.rside() {
                        let mut new_rside = tbi1.lside().clone().inverse().unwrap();
                        new_rside = new_rside.exists().unwrap();
                        new_rside = new_rside.negate();

                        let new_tbi = TBI::new(tbi2.lside().clone(), new_rside).unwrap();

                        Some(vec![new_tbi])
                    } else {
                        Option::None
                    }
                } else {
                    Option::None
                }
            } else {
                Option::None
            }
        }
    }
}

// fifth rule: Exists_r=>notExists_r then r=>not_r and Exists_r_inv=>notExists_r_inv
pub fn dl_lite_rule_five(vec: Vec<&TBI>) -> Option<Vec<TBI>> {
    if vec.len() != 1 {
        Option::None
    } else {
        let tbi = vec[0];
        let tbi_rside_child = Node::child_r(Some(&tbi.rside()), 1);

        if tbi.lside().t() == DLType::ExistsConcept && tbi_rside_child.is_some(){
            if tbi.lside() == tbi_rside_child.unwrap() {
                let role = Node::child_r(Some(tbi.lside()), 1).unwrap().clone();

                let not_role = (&role).clone().negate();
                let inv_role = (&role).clone().inverse().unwrap();
                let exists = (&inv_role).clone().exists().unwrap();
                let not_exists = inv_role.exists().unwrap().negate();

                let new_tbi1 = TBI::new(role, not_role).unwrap();
                let new_tbi2 = TBI::new(exists, not_exists).unwrap();

                Some(vec![new_tbi1, new_tbi2])
            } else {
                Option::None
            }
        } else {
            Option::None
        }
    }
}

// TODO: verify that fifth and sixth rules are really different, for the moment I'm not implementing the sixth rule
// sixth rule: Exists_r_inv=>notExists_r_inv then r=>not_r and Exists_r=>notExists_r
pub fn dl_lite_rule_six(vec: Vec<&TBI>) -> Option<Vec<TBI>> {
    Option::None
}

// seventh rule: r=>not_r then Exists_r=>notExists_r and Exists_r_inv=>notExists_r_inv
pub fn dl_lite_rule_seven(vec: Vec<&TBI>) -> Option<Vec<TBI>> {
    if vec.len() != 1 {
        Option::None
    } else {
        let tbi = vec[0];

        match (tbi.lside().t(), tbi.rside().t()) {
            (DLType::BaseRole, DLType::NegatedRole) => {
                let r = tbi.lside().clone();
                let maybe_not_r = tbi.rside();

                if &r == Node::child_r(Some(maybe_not_r), 1).unwrap() {
                    let exists_r = (&r).clone().exists().unwrap();
                    let not_exists_r = (&exists_r).clone().negate();

                    let r_inv = (&r).clone().inverse().unwrap();
                    let exists_r_inv = r_inv.exists().unwrap();
                    let not_exists_r_inv = (&exists_r_inv).clone().negate();

                    let new_tbi1 = TBI::new(exists_r, not_exists_r).unwrap();
                    let new_tbi2 = TBI::new(exists_r_inv, not_exists_r_inv).unwrap();

                    Some(vec![new_tbi1, new_tbi2])
                } else {
                    Option::None
                }
            },
            (_, _) => Option::None,
        }
    }
}

// eight rule: r1=>r2 then r1_inv=>r2_inv and Exists_r1=>Exists_r2
pub fn dl_lite_rule_eight(vec: Vec<&TBI>) -> Option<Vec<TBI>> {
    if vec.len() != 1 {
        Option::None
    } else {
        let tbi = vec[0];
        match (tbi.lside().t(), tbi.rside().t()) {
            (DLType::BaseRole, DLType::BaseRole) => {
                let r1 = tbi.lside().clone();
                let r2 = tbi.rside().clone();

                let r1_inv = (&r1).clone().inverse().unwrap();
                let r2_inv = (&r2).clone().inverse().unwrap();

                let exists_r1 = r1.exists().unwrap();
                let exists_r2 = r2.exists().unwrap();

                let new_tbi1 = TBI::new(r1_inv, r2_inv).unwrap();
                let new_tbi2 = TBI::new(exists_r1, exists_r2).unwrap();

                Some(vec![new_tbi1, new_tbi2])
            },
            (_, _) => Option::None,
        }
    }
}
