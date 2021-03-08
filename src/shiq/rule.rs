use crate::shiq::abox_item::ABI_SHIQ;
use crate::shiq::node::Node_SHIQ;
use crate::shiq::tbox_item::TBI_SHIQ;
use crate::kb::knowledge_base::{ABox, ABoxItem, Item, TBox, TBoxItem};
use crate::kb::types::DLType;
use crate::shiq::abox_item_quantum::ABIQ_SHIQ;

/*
   I'm changing the rule philosophy, now they (if they can) take the first
   n tbis they need from the vector
*/

/*
   this type englobe all type of rules for tbox items
   take a number of items and generates a new if possible
*/

// for the moment we only implement for shiq

// TODO: the rules must implement the level and impliers stuff

//--------------------------------------------------------------------------------------------------
// rule zero has deduction tree
pub fn shiq_rule_zero(vec: Vec<&TBI_SHIQ>, deduction_tree: bool) -> Option<Vec<TBI_SHIQ>> {
    /*
    bottom is included in everything
    everything is included in Top
    X => Y implies X => Top, Y => Top, Bottom => X, Y => Bottom
     */
    if vec.len() < 1 {
        Option::None
    } else {
        let tbi = vec[0]; // we only take the first

        if DLType::all_concepts(tbi.lside().t(), tbi.rside().t()) {
            let bottom = Node_SHIQ::new(Option::None, DLType::Bottom);
            let top = Node_SHIQ::new(Option::None, DLType::Top);

            if bottom.is_none() || top.is_none() {
                Option::None
            } else {
                let bottom = bottom.unwrap();
                let top = top.unwrap();

                // here we add the new level
                let level_to_give = tbi.level() + 1;

                // the bottom tbis
                let bottom1 = TBI_SHIQ::new((&bottom).clone(), tbi.lside().clone(), level_to_give);
                let bottom2 = TBI_SHIQ::new(bottom, tbi.rside().clone(), level_to_give);

                // the top tbis
                let top1 = TBI_SHIQ::new(tbi.lside().clone(), (&top).clone(), level_to_give);
                let top2 = TBI_SHIQ::new(tbi.rside().clone(), top, level_to_give);

                if bottom1.is_none() || bottom2.is_none() || top1.is_none() || top2.is_none() {
                    Option::None
                } else {
                    let mut bottom1 = bottom1.unwrap();
                    let mut bottom2 = bottom2.unwrap();
                    let mut top1 = top1.unwrap();
                    let mut top2 = top2.unwrap();

                    // later I must find a way to avoid needlessly clones
                    if deduction_tree {
                        let impliers = vec![tbi.clone()];

                        bottom1.add_to_implied_by(impliers.clone());
                        bottom2.add_to_implied_by(impliers.clone());
                        top1.add_to_implied_by(impliers.clone());
                        top2.add_to_implied_by(impliers.clone());
                    }

                    Some(vec![bottom1, bottom2, top1, top2])
                }
            }
        } else {
            Option::None
        }
    }
}

// one has deduction tree
pub fn shiq_rule_one(vec: Vec<&TBI_SHIQ>, deduction_tree: bool) -> Option<Vec<TBI_SHIQ>> {
    /*
    negation rule
    A=>notB then B=>notA
     */
    if vec.len() < 1 {
        Option::None
    } else {
        let tbi = vec[0];

        if tbi.rside().is_negated() && tbi.rside().t() != DLType::Top {
            // no need to be changing top
            let add_level = true; // change after if necessary

            let mut tbi_reversed = tbi.reverse_negation(add_level).unwrap();

            if deduction_tree {
                tbi_reversed.add_to_implied_by(vec![tbi.clone()]);
            }

            Some(vec![tbi_reversed])
        } else {
            Option::None
        }
    }
}

// two implement deduction tree
pub fn shiq_rule_two(vec: Vec<&TBI_SHIQ>, deduction_tree: bool) -> Option<Vec<TBI_SHIQ>> {
    /*
    chain rule
    A=>B and B=>C then A=>C
     */
    if vec.len() < 2 {
        Option::None
    } else {
        let tbi1 = vec[0];
        let tbi2 = vec[1];

        // the level is added here
        let biggest_level = if tbi1.level() >= tbi2.level() {
            tbi1.level()
        } else {
            tbi2.level()
        };

        if tbi1.rside() == tbi2.lside() {
            let mut new_tbi = TBI_SHIQ::new(
                tbi1.lside().clone(),
                tbi2.rside().clone(),
                biggest_level + 1,
            )
            .unwrap();

            if deduction_tree {
                new_tbi.add_to_implied_by(vec![tbi1.clone(), tbi2.clone()]);
            }

            Some(vec![new_tbi])
        } else if tbi2.rside() == tbi1.lside() {
            let mut new_tbi = TBI_SHIQ::new(
                tbi2.lside().clone(),
                tbi1.rside().clone(),
                biggest_level + 1,
            )
            .unwrap();

            if deduction_tree {
                new_tbi.add_to_implied_by(vec![tbi1.clone(), tbi2.clone()]);
            }

            Some(vec![new_tbi])
        } else {
            Option::None
        }
    }
}

// third rule: r1=>r2 and B=>notExists_r2 then B=>notExists_r1 and Exists_r1=>notB
pub fn shiq_rule_three(vec: Vec<&TBI_SHIQ>, deduction_tree: bool) -> Option<Vec<TBI_SHIQ>> {
    // maybe matches are super cool but here will be more complicated, Boxes are complicated
    if vec.len() < 2 {
        Option::None
    } else {
        let tbi1 = vec[0];
        let tbi2 = vec[1];

        let get_max = true;
        let big_level = TBI_SHIQ::get_extrema_level(vec, 2, get_max);

        // verifies you have roles
        if !DLType::all_roles(tbi1.lside().t(), tbi1.rside().t()) {
            Option::None
        } else {
            // two time because is second child we want

            // I will use recursive child
            let tbi2_rside_second_child = Node_SHIQ::child(Some(tbi2.lside()), 2);

            if (&tbi2_rside_second_child).is_some() {
                // this big ass condition verfies that rside of tbi2 is of the correct form
                if tbi2.rside().t() == DLType::NegatedConcept
                    && (Node_SHIQ::child_old(Some(tbi2.rside())).unwrap().t() == DLType::ExistsConcept)
                {
                    if tbi2_rside_second_child.unwrap().get(0).unwrap() == &tbi1.rside() {
                        let exists_r1 = tbi1.lside().clone().exists().unwrap();
                        let not_exists_r1 = (&exists_r1).clone();

                        // add levels
                        let mut new_tbi1 =
                            TBI_SHIQ::new(tbi2.lside().clone(), not_exists_r1, big_level + 1).unwrap();
                        let mut new_tbi2 =
                            TBI_SHIQ::new(exists_r1, tbi2.lside().clone().negate(), big_level + 1)
                                .unwrap();

                        // it is here we put level and deduction tree
                        if deduction_tree {
                            let v = vec![tbi1.clone(), tbi2.clone()];

                            new_tbi1.add_to_implied_by(v.clone());
                            new_tbi2.add_to_implied_by(v);
                        }

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
pub fn shiq_rule_four(vec: Vec<&TBI_SHIQ>, deduction_tree: bool) -> Option<Vec<TBI_SHIQ>> {
    // maybe matches are super cool but here will be more complicated, Boxes are complicated
    if vec.len() < 2 {
        Option::None
    } else {
        let tbi1 = vec[0];
        let tbi2 = vec[1];

        let get_max = true;
        let big_level = TBI_SHIQ::get_extrema_level(vec, 2, get_max);

        // verifies you have roles
        if !DLType::all_roles(tbi1.lside().t(), tbi1.rside().t()) {
            Option::None
        } else {
            // two time because is second child we want
            let tbi2_rside_third_child = Node_SHIQ::child(Some(tbi2.lside()), 3);

            if (&tbi2_rside_third_child).is_some() {
                let tbi2_rside = tbi2.rside();
                // this big ass condition verfies that rside to tbi2 is of the correct form
                if tbi2_rside.t() == DLType::NegatedConcept
                    && Node_SHIQ::child(Some(tbi2_rside), 1).unwrap().get(0).unwrap().t() == DLType::ExistsConcept
                    && Node_SHIQ::child(Some(tbi2_rside), 2).unwrap().get(0).unwrap().t() == DLType::InverseRole
                {
                    if tbi2_rside_third_child.unwrap().get(0).unwrap() == &tbi1.rside() {
                        let mut new_rside = tbi1.lside().clone().inverse().unwrap();
                        new_rside = new_rside.exists().unwrap();
                        new_rside = new_rside.negate();

                        // added level
                        let mut new_tbi =
                            TBI_SHIQ::new(tbi2.lside().clone(), new_rside, big_level + 1).unwrap();

                        // added impliers
                        if deduction_tree {
                            new_tbi.add_to_implied_by(vec![tbi1.clone(), tbi2.clone()]);
                        }

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
pub fn shiq_rule_five(vec: Vec<&TBI_SHIQ>, deduction_tree: bool) -> Option<Vec<TBI_SHIQ>> {
    if vec.len() < 1 {
        Option::None
    } else {
        let tbi = vec[0];
        let tbi_rside_child = Node_SHIQ::child(Some(&tbi.rside()), 1);

        let big_level = tbi.level();

        if tbi.lside().t() == DLType::ExistsConcept && tbi_rside_child.is_some() {
            if &tbi.lside() == tbi_rside_child.unwrap().get(0).unwrap() {
                let role = Node_SHIQ::child(Some(tbi.lside()), 1).unwrap().get(0).unwrap().clone().clone();

                let not_role = (&role).clone().negate();
                let inv_role = (&role).clone().inverse().unwrap();
                let exists = (&inv_role).clone().exists().unwrap();
                let not_exists = inv_role.exists().unwrap().negate();

                let mut new_tbi1 = TBI_SHIQ::new(role, not_role, big_level + 1).unwrap();
                let mut new_tbi2 = TBI_SHIQ::new(exists, not_exists, big_level + 1).unwrap();

                if deduction_tree {
                    let v = vec![tbi.clone()];
                    new_tbi1.add_to_implied_by(v.clone());
                    new_tbi2.add_to_implied_by(v);
                }

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
pub fn shiq_rule_six(_vec: Vec<&TBI_SHIQ>, _deduction_tree: bool) -> Option<Vec<TBI_SHIQ>> {
    Option::None
}

// seventh rule: r=>not_r then Exists_r=>notExists_r and Exists_r_inv=>notExists_r_inv
pub fn shiq_rule_seven(vec: Vec<&TBI_SHIQ>, deduction_tree: bool) -> Option<Vec<TBI_SHIQ>> {
    if vec.len() < 1 {
        Option::None
    } else {
        let tbi = vec[0];

        let big_level = tbi.level();

        match (tbi.lside().t(), tbi.rside().t()) {
            (DLType::BaseRole, DLType::NegatedRole) => {
                let r = tbi.lside().clone();
                let maybe_not_r = tbi.rside();

                if &r == *Node_SHIQ::child(Some(maybe_not_r), 1).unwrap().get(0).unwrap() {
                    let exists_r = (&r).clone().exists().unwrap();
                    let not_exists_r = (&exists_r).clone().negate();

                    let r_inv = (&r).clone().inverse().unwrap();
                    let exists_r_inv = r_inv.exists().unwrap();
                    let not_exists_r_inv = (&exists_r_inv).clone().negate();

                    // added level
                    let mut new_tbi1 = TBI_SHIQ::new(exists_r, not_exists_r, big_level + 1).unwrap();
                    let mut new_tbi2 =
                        TBI_SHIQ::new(exists_r_inv, not_exists_r_inv, big_level + 1).unwrap();

                    // added impliers
                    if deduction_tree {
                        let v = vec![tbi.clone()];

                        new_tbi1.add_to_implied_by(v.clone());
                        new_tbi2.add_to_implied_by(v);
                    }

                    Some(vec![new_tbi1, new_tbi2])
                } else {
                    Option::None
                }
            }
            (_, _) => Option::None,
        }
    }
}

// eight rule: r1=>r2 then r1_inv=>r2_inv and Exists_r1=>Exists_r2
pub fn shiq_rule_eight(vec: Vec<&TBI_SHIQ>, deduction_tree: bool) -> Option<Vec<TBI_SHIQ>> {
    if vec.len() < 1 {
        Option::None
    } else {
        let tbi = vec[0];
        let big_level = tbi.level();

        match (tbi.lside().t(), tbi.rside().t()) {
            (DLType::BaseRole, DLType::BaseRole) => {
                let r1 = tbi.lside().clone();
                let r2 = tbi.rside().clone();

                let r1_inv = (&r1).clone().inverse().unwrap();
                let r2_inv = (&r2).clone().inverse().unwrap();

                let exists_r1 = r1.exists().unwrap();
                let exists_r2 = r2.exists().unwrap();

                // added levels
                let mut new_tbi1 = TBI_SHIQ::new(r1_inv, r2_inv, big_level + 1).unwrap();
                let mut new_tbi2 = TBI_SHIQ::new(exists_r1, exists_r2, big_level + 1).unwrap();

                // added deduction tree
                if deduction_tree {
                    let v = vec![tbi.clone()];

                    new_tbi1.add_to_implied_by(v.clone());
                    new_tbi2.add_to_implied_by(v);
                }

                Some(vec![new_tbi1, new_tbi2])
            }
            (_, _) => Option::None,
        }
    }
}

//-----------------------------------------------------------------------------------------------
// here I will put the rules for aboxes

// if (a,b):r then a:Er and b:Er^⁻
pub fn shiq_abox_rule_one(abis: Vec<&ABIQ_SHIQ>, _tbis: Vec<&TBI_SHIQ>) -> Option<Vec<ABIQ_SHIQ>> {
    if abis.len() < 1 {
        Option::None
    } else {
        let abi = abis[0].abi();

        match abi {
            ABI_SHIQ::CA(_, _) => Option::None,
            ABI_SHIQ::RA(r, a, b) => {
                let r_is_neg = r.is_negated();
                let mut v: Vec<ABIQ_SHIQ> = Vec::new();

                match r_is_neg {
                    true => Some(v),
                    false => {
                        // first create a: Er
                        let er = r.clone().exists().unwrap(); // this should work, r is a base role
                        let a_er = ABI_SHIQ::new_ca(er, a.clone(), true).unwrap(); // rules are always applied for completion

                        // secondly create b:Er^-
                        let erinv = r.clone().inverse().unwrap().exists().unwrap();
                        let b_erinv = ABI_SHIQ::new_ca(erinv, b.clone(), true).unwrap();

                        let a_er_q = ABIQ_SHIQ::new(a_er, Option::None, Option::None);
                        let b_erinv_q = ABIQ_SHIQ::new(b_erinv, Option::None, Option::None);

                        v.push(a_er_q);
                        v.push(b_erinv_q);

                        Some(v)
                    }
                }
            }
        }
    }
}

// if (a,b):r and r < s then (a,b):s
pub fn shiq_abox_rule_two(abis: Vec<&ABIQ_SHIQ>, tbis: Vec<&TBI_SHIQ>) -> Option<Vec<ABIQ_SHIQ>> {
    if abis.len() < 1 || tbis.len() < 1 {
        Option::None
    } else {
        let abi = abis[0].abi();
        let tbi = tbis[0];

        match abi {
            ABI_SHIQ::CA(_, _) => Option::None,
            ABI_SHIQ::RA(r, a, b) => {
                if r == tbi.lside() {
                    let new_ra = ABI_SHIQ::new_ra(tbi.rside().clone(), a.clone(), b.clone(), true);

                    let new_ra_q = ABIQ_SHIQ::new(new_ra.unwrap(), Option::None, Option::None);

                    let v = vec![new_ra_q];

                    Some(v)
                } else {
                    Option::None
                }
            }
        }
    }
}

// if a:c and c < d then a:d
pub fn shiq_abox_rule_three(abis: Vec<&ABIQ_SHIQ>, tbis: Vec<&TBI_SHIQ>) -> Option<Vec<ABIQ_SHIQ>> {
    if abis.len() < 1 || tbis.len() < 1 {
        Option::None
    } else {
        let abi = abis[0].abi();
        let tbi = tbis[0];

        match abi {
            ABI_SHIQ::RA(_, _, _) => Option::None,
            ABI_SHIQ::CA(c, a) => {
                if c == tbi.lside() {
                    let new_ca = ABI_SHIQ::new_ca(tbi.rside().clone(), a.clone(), true);

                    let new_ca_q = ABIQ_SHIQ::new(new_ca.unwrap(), Option::None, Option::None);

                    let v = vec![new_ca_q];

                    Some(v)
                } else {
                    Option::None
                }
            }
        }
    }
}