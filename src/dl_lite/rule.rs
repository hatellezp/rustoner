/*
© - 2021 – UMONS
Horacio Alejandro Tellez Perez

LICENSE GPLV3+:
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses/.
*/

use crate::dl_lite::abox_item::AbiDllite;
use crate::dl_lite::abox_item_quantum::AbiqDllite;
use crate::dl_lite::node::ItemDllite;
use crate::dl_lite::tbox_item::TbiDllite;
use crate::kb::knowledge_base::{Implier, Item, TBoxItem};
use crate::kb::types::{DLType, CR};

/*
   I'm changing the rule philosophy, now they (if they can) take the first
   n tbis they need from the vector
*/

/*
   this type englobe all type of rules for tbox items
   take a number of items and generates a new if possible
*/

// for the moment we only implement for dl_lite

// TODO: the rules must implement the level and impliers stuff

//--------------------------------------------------------------------------------------------------
// rule zero has deduction tree
pub fn dl_lite_rule_zero(vec: Vec<&TbiDllite>, deduction_tree: bool) -> Option<Vec<TbiDllite>> {
    /*
    bottom is included in everything
    everything is included in Top
    X => Y implies X => Top, Y => Top, Bottom => X, Y => Bottom
     */
    if vec.is_empty() {
        Option::None
    } else {
        let tbi = vec[0]; // we only take the first

        if DLType::all_concepts(tbi.lside().t(), tbi.rside().t()) {
            let bottom_op = ItemDllite::new(Option::None, DLType::Bottom);
            let top_op = ItemDllite::new(Option::None, DLType::Top);

            match (bottom_op, top_op) {
                (Some(bottom), Some(top)) => {
                    // here we add the new level
                    let level_to_give = tbi.level() + 1;

                    // the bottom tbis
                    let bottom1 =
                        TbiDllite::new((&bottom).clone(), tbi.lside().clone(), level_to_give);
                    let bottom2 = TbiDllite::new(bottom, tbi.rside().clone(), level_to_give);

                    // the top tbis
                    let top1 = TbiDllite::new(tbi.lside().clone(), (&top).clone(), level_to_give);
                    let top2 = TbiDllite::new(tbi.rside().clone(), top, level_to_give);

                    match (bottom1, bottom2, top1, top2) {
                        (Some(mut b1), Some(mut b2), Some(mut t1), Some(mut t2)) => {
                            if deduction_tree {
                                let impliers = vec![tbi.clone()];

                                b1.add_to_implied_by((CR::Zero, impliers.clone()));
                                b2.add_to_implied_by((CR::Zero, impliers.clone()));
                                t1.add_to_implied_by((CR::Zero, impliers.clone()));
                                t2.add_to_implied_by((CR::Zero, impliers));
                            }

                            Some(vec![b1, b2, t1, t2])
                        }
                        (_, _, _, _) => Option::None,
                    }
                }
                (_, _) => Option::None,
            }
        } else {
            Option::None
        }
    }
}

// one has deduction tree
pub fn dl_lite_rule_one(vec: Vec<&TbiDllite>, deduction_tree: bool) -> Option<Vec<TbiDllite>> {
    /*
    negation rule
    A=>notB then B=>notA
     */
    if vec.is_empty() {
        Option::None
    } else {
        let tbi = vec[0];

        if tbi.rside().is_negated() && tbi.rside().t() != DLType::Top {
            // no need to be changing top
            let add_level = true; // change after if necessary

            let mut tbi_reversed = tbi.reverse_negation(add_level).unwrap();

            if deduction_tree {
                tbi_reversed.add_to_implied_by((CR::First, vec![tbi.clone()]));
            }

            Some(vec![tbi_reversed])
        } else {
            Option::None
        }
    }
}

// two implement deduction tree
pub fn dl_lite_rule_two(vec: Vec<&TbiDllite>, deduction_tree: bool) -> Option<Vec<TbiDllite>> {
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
            let mut new_tbi = TbiDllite::new(
                tbi1.lside().clone(),
                tbi2.rside().clone(),
                biggest_level + 1,
            )
            .unwrap();

            if deduction_tree {
                new_tbi.add_to_implied_by((CR::Second, vec![tbi1.clone(), tbi2.clone()]));
            }

            Some(vec![new_tbi])
        } else if tbi2.rside() == tbi1.lside() {
            let mut new_tbi = TbiDllite::new(
                tbi2.lside().clone(),
                tbi1.rside().clone(),
                biggest_level + 1,
            )
            .unwrap();

            if deduction_tree {
                new_tbi.add_to_implied_by((CR::Second, vec![tbi1.clone(), tbi2.clone()]));
            }

            Some(vec![new_tbi])
        } else {
            Option::None
        }
    }
}

// third rule: r1=>r2 and B=>notExists_r2 then B=>notExists_r1 and Exists_r1=>notB
pub fn dl_lite_rule_three(vec: Vec<&TbiDllite>, deduction_tree: bool) -> Option<Vec<TbiDllite>> {
    // maybe matches are super cool but here will be more complicated, Boxes are complicated
    if vec.len() < 2 {
        Option::None
    } else {
        let tbi1 = vec[0];
        let tbi2 = vec[1];

        let get_max = true;
        let big_level = TbiDllite::get_extrema_level(vec, 2, get_max);

        // verifies you have roles
        if !DLType::all_roles(tbi1.lside().t(), tbi1.rside().t()) {
            Option::None
        } else {
            // two time because is second child we want

            // I will use recursive child
            let tbi2_rside_second_child = ItemDllite::child(Some(tbi2.lside()), 2);

            if (&tbi2_rside_second_child).is_some() {
                // this big ass condition verifies that rside of tbi2 is of the correct form
                if tbi2.rside().t() == DLType::NegatedConcept
                    // TODO: this works apparently well
                    && (ItemDllite::child(Some(tbi2.rside()), 1).unwrap()[0].t() == DLType::ExistsConcept)
                // && (ItemDllite::child_old(Some(tbi2.rside())).unwrap().t() == DLType::ExistsConcept)
                {
                    if tbi2_rside_second_child.unwrap().get(0).unwrap() == &tbi1.rside() {
                        let exists_r1 = tbi1.lside().clone().exists().unwrap();
                        let not_exists_r1 = (&exists_r1).clone();

                        // add levels
                        let mut new_tbi1 =
                            TbiDllite::new(tbi2.lside().clone(), not_exists_r1, big_level + 1)
                                .unwrap();
                        let mut new_tbi2 =
                            TbiDllite::new(exists_r1, tbi2.lside().clone().negate(), big_level + 1)
                                .unwrap();

                        // it is here we put level and deduction tree
                        if deduction_tree {
                            let v = vec![tbi1.clone(), tbi2.clone()];

                            new_tbi1.add_to_implied_by((CR::Third, v.clone()));
                            new_tbi2.add_to_implied_by((CR::Third, v));
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
pub fn dl_lite_rule_four(vec: Vec<&TbiDllite>, deduction_tree: bool) -> Option<Vec<TbiDllite>> {
    // maybe matches are super cool but here will be more complicated, Boxes are complicated
    if vec.len() < 2 {
        Option::None
    } else {
        let tbi1 = vec[0];
        let tbi2 = vec[1];

        let get_max = true;
        let big_level = TbiDllite::get_extrema_level(vec, 2, get_max);

        // verifies you have roles
        if !DLType::all_roles(tbi1.lside().t(), tbi1.rside().t()) {
            Option::None
        } else {
            // two time because is second child we want
            let tbi2_rside_third_child = ItemDllite::child(Some(tbi2.lside()), 3);

            if (&tbi2_rside_third_child).is_some() {
                let tbi2_rside = tbi2.rside();
                // this big ass condition verfies that rside to tbi2 is of the correct form
                if tbi2_rside.t() == DLType::NegatedConcept
                    && ItemDllite::child(Some(tbi2_rside), 1)
                        .unwrap()
                        .get(0)
                        .unwrap()
                        .t()
                        == DLType::ExistsConcept
                    && ItemDllite::child(Some(tbi2_rside), 2)
                        .unwrap()
                        .get(0)
                        .unwrap()
                        .t()
                        == DLType::InverseRole
                {
                    if tbi2_rside_third_child.unwrap().get(0).unwrap() == &tbi1.rside() {
                        let mut new_rside = tbi1.lside().clone().inverse().unwrap();
                        new_rside = new_rside.exists().unwrap();
                        new_rside = new_rside.negate();

                        // added level
                        let mut new_tbi =
                            TbiDllite::new(tbi2.lside().clone(), new_rside, big_level + 1).unwrap();

                        // added impliers
                        if deduction_tree {
                            new_tbi
                                .add_to_implied_by((CR::Fourth, vec![tbi1.clone(), tbi2.clone()]));
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
pub fn dl_lite_rule_five(vec: Vec<&TbiDllite>, deduction_tree: bool) -> Option<Vec<TbiDllite>> {
    if vec.is_empty() {
        Option::None
    } else {
        let tbi = vec[0];
        let tbi_rside_child = ItemDllite::child(Some(&tbi.rside()), 1);

        let big_level = tbi.level();

        if tbi.lside().t() == DLType::ExistsConcept {
            if let Some(some_child) = tbi_rside_child {
                if tbi.lside() == some_child[0] {
                    let role = ItemDllite::child(Some(tbi.lside()), 1).unwrap()[0].clone();

                    let not_role = (&role).clone().negate();
                    let inv_role = (&role).clone().inverse().unwrap();
                    let exists = (&inv_role).clone().exists().unwrap();
                    let not_exists = inv_role.exists().unwrap().negate();

                    let mut new_tbi1 = TbiDllite::new(role, not_role, big_level + 1).unwrap();
                    let mut new_tbi2 = TbiDllite::new(exists, not_exists, big_level + 1).unwrap();

                    if deduction_tree {
                        let v = vec![tbi.clone()];
                        new_tbi1.add_to_implied_by((CR::Fifth, v.clone()));
                        new_tbi2.add_to_implied_by((CR::Fifth, v));
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

// TODO: verify that fifth and sixth rules are really different, for the moment I'm not implementing the sixth rule
// sixth rule: Exists_r_inv=>notExists_r_inv then r=>not_r and Exists_r=>notExists_r
pub fn dl_lite_rule_six(_vec: Vec<&TbiDllite>, _deduction_tree: bool) -> Option<Vec<TbiDllite>> {
    Option::None
}

// seventh rule: r=>not_r then Exists_r=>notExists_r and Exists_r_inv=>notExists_r_inv
pub fn dl_lite_rule_seven(vec: Vec<&TbiDllite>, deduction_tree: bool) -> Option<Vec<TbiDllite>> {
    if vec.is_empty() {
        Option::None
    } else {
        let tbi = vec[0];

        let big_level = tbi.level();

        match (tbi.lside().t(), tbi.rside().t()) {
            (DLType::BaseRole, DLType::NegatedRole) => {
                let r = tbi.lside().clone();
                let maybe_not_r = tbi.rside();

                if &r
                    == *ItemDllite::child(Some(maybe_not_r), 1)
                        .unwrap()
                        .get(0)
                        .unwrap()
                {
                    let exists_r = (&r).clone().exists().unwrap();
                    let not_exists_r = (&exists_r).clone().negate();

                    let r_inv = (&r).clone().inverse().unwrap();
                    let exists_r_inv = r_inv.exists().unwrap();
                    let not_exists_r_inv = (&exists_r_inv).clone().negate();

                    // added level
                    let mut new_tbi1 =
                        TbiDllite::new(exists_r, not_exists_r, big_level + 1).unwrap();
                    let mut new_tbi2 =
                        TbiDllite::new(exists_r_inv, not_exists_r_inv, big_level + 1).unwrap();

                    // added impliers
                    if deduction_tree {
                        let v = vec![tbi.clone()];

                        new_tbi1.add_to_implied_by((CR::Seventh, v.clone()));
                        new_tbi2.add_to_implied_by((CR::Seventh, v));
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
pub fn dl_lite_rule_eight(vec: Vec<&TbiDllite>, deduction_tree: bool) -> Option<Vec<TbiDllite>> {
    if vec.is_empty() {
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
                let mut new_tbi1 = TbiDllite::new(r1_inv, r2_inv, big_level + 1).unwrap();
                let mut new_tbi2 = TbiDllite::new(exists_r1, exists_r2, big_level + 1).unwrap();

                // added deduction tree
                if deduction_tree {
                    let v = vec![tbi.clone()];

                    new_tbi1.add_to_implied_by((CR::Eight, v.clone()));
                    new_tbi2.add_to_implied_by((CR::Eight, v));
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
pub fn dl_lite_abox_rule_one(
    abis: Vec<&AbiqDllite>,
    _tbis: Vec<&TbiDllite>,
    deduction_tree: bool,
) -> Option<Vec<AbiqDllite>> {
    if abis.is_empty() {
        Option::None
    } else {
        let abi = abis[0].abi();
        let big_level = abis[0].level();

        match abi {
            AbiDllite::CA(_, _) => Option::None,
            AbiDllite::RA(r, a, b) => {
                let r_is_neg = r.is_negated();
                let mut v: Vec<AbiqDllite> = Vec::new();

                match r_is_neg {
                    true => Some(v),
                    false => {
                        // first create a: Er
                        let er = r.clone().exists().unwrap(); // this should work, r is a base role
                        let a_er = AbiDllite::new_ca(er, a.clone(), true).unwrap(); // rules are always applied for completion

                        // secondly create b:Er^-
                        let erinv = r.clone().inverse().unwrap().exists().unwrap();
                        let b_erinv = AbiDllite::new_ca(erinv, b.clone(), true).unwrap();

                        let mut a_er_q =
                            AbiqDllite::new(a_er, Option::None, Option::None, big_level + 1);
                        let mut b_erinv_q =
                            AbiqDllite::new(b_erinv, Option::None, Option::None, big_level + 1);

                        if deduction_tree {
                            a_er_q.add_to_implied_by((CR::First, vec![], vec![abis[0].clone()]));
                            b_erinv_q.add_to_implied_by((CR::First, vec![], vec![abis[0].clone()]));
                        }

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
pub fn dl_lite_abox_rule_two(
    abis: Vec<&AbiqDllite>,
    tbis: Vec<&TbiDllite>,
    deduction_tree: bool,
) -> Option<Vec<AbiqDllite>> {
    if abis.is_empty() || tbis.is_empty() {
        Option::None
    } else {
        let abi = abis[0].abi();
        let tbi = tbis[0];
        let big_level = abis[0].level();

        match abi {
            AbiDllite::CA(_, _) => Option::None,
            AbiDllite::RA(r, a, b) => {
                if r == tbi.lside() {
                    let new_ra = AbiDllite::new_ra(tbi.rside().clone(), a.clone(), b.clone(), true);

                    let mut new_ra_q =
                        AbiqDllite::new(new_ra.unwrap(), Option::None, Option::None, big_level + 1);

                    if deduction_tree {
                        new_ra_q.add_to_implied_by((
                            CR::Second,
                            vec![tbi.clone()],
                            vec![abis[0].clone()],
                        ));
                    }

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
pub fn dl_lite_abox_rule_three(
    abis: Vec<&AbiqDllite>,
    tbis: Vec<&TbiDllite>,
    deduction_tree: bool,
) -> Option<Vec<AbiqDllite>> {
    if abis.is_empty() || tbis.is_empty() {
        Option::None
    } else {
        let abi = abis[0].abi();
        let tbi = tbis[0];
        let big_level = abis[0].level();

        match abi {
            AbiDllite::RA(_, _, _) => Option::None,
            AbiDllite::CA(c, a) => {
                if c == tbi.lside() {
                    let new_ca = AbiDllite::new_ca(tbi.rside().clone(), a.clone(), true);

                    let mut new_ca_q =
                        AbiqDllite::new(new_ca.unwrap(), Option::None, Option::None, big_level + 1);

                    if deduction_tree {
                        new_ca_q.add_to_implied_by((
                            CR::Third,
                            vec![tbi.clone()],
                            vec![abis[0].clone()],
                        ));
                    }

                    let v = vec![new_ca_q];
                    Some(v)
                } else {
                    Option::None
                }
            }
        }
    }
}
