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
use crate::kb::knowledge_base::{Implier, Item, LeveledItem, TBoxItem, TbRule};
use crate::kb::types::{DLType, CR};

/*
   I'm changing the rule philosophy, now they (if they can) take the first
   n tbis they need from the vector
*/

/*
   this type englobe all type of rules for tbox items
   take a number of items and generates a new if possible
*/

//-----------------------------------------------------------------------------------------------
// here I will put the rules for aboxes

// if (a,b):r then a:Er and b:Er^⁻
pub fn dl_lite_abox_rule_one(
    abis: &[&AbiqDllite],
    _tbis: &[&TbiDllite],
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
    abis: &[&AbiqDllite],
    tbis: &[&TbiDllite],
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
    abis: &[&AbiqDllite],
    tbis: &[&TbiDllite],
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

//================================================================================================//
// FROM THIS POINT ON I'M IMPLEMENTING THE CLOSURE RULES AND MAKING A DIFFERENCE BETWEEN
// POSITIVE AND NEGATIVE CLOSURE

/*
   how to add a new rule:
       - add the rule definition below the last one,
       - add a pointer to it in this function just below
*/

pub fn dl_lite_closure_enter_point(
    vec: &[&TbiDllite],
    deduction_tree: bool,
    rule_number: usize,
) -> Option<Vec<TbiDllite>> {
    // type declaration
    type T = TbiDllite;

    // add a pointer here for your new rule
    let rule_one: TbRule<T> = dl_lite_closure_negative_one;
    let rule_two: TbRule<T> = dl_lite_closure_negative_two;
    let rule_three: TbRule<T> = dl_lite_closure_negative_three;
    let rule_four: TbRule<T> = dl_lite_closure_negative_four;
    let rule_five: TbRule<T> = dl_lite_closure_negative_five;
    let rule_six: TbRule<T> = dl_lite_closure_positive_six;
    let rule_seven: TbRule<T> = dl_lite_closure_positive_seven;
    let rule_eight: TbRule<T> = dl_lite_closure_positive_eight;
    let rule_nine: TbRule<T> = dl_lite_closure_positive_nine;
    let rule_ten: TbRule<T> = dl_lite_closure_positive_ten;

    // add the pointer to the array
    let rules = [
        &rule_one,
        &rule_two,
        &rule_three,
        &rule_four,
        &rule_five,
        &rule_six,
        &rule_seven,
        &rule_eight,
        &rule_nine,
        &rule_ten,
    ];

    // update the number of rules
    let number_of_rules_present = 10;

    // ALL RULE FOR DLLITE HAVE ARITY 2, THUS I'M USING THAT !!!!

    if 1 <= rule_number && rule_number <= number_of_rules_present {
        match vec.len() {
            0 | 1 => None,
            _ => rules[rule_number](vec, deduction_tree),
        }
    } else {
        None
    }
}

// RULES DO NOT MAKE THE VERIFICATION OF CORRECT ARITY, THIS TEST IS DONE BY THE
// RULES ENTER POINT JUST ABOVE !!

pub fn output_new_tbis_for_rules(
    new_tbi_op: Option<TbiDllite>,
    deduction_tree: bool,
    rule_number: CR,
    tbi1: &TbiDllite,
    tbi2: &TbiDllite,
) -> Option<Vec<TbiDllite>> {
    match new_tbi_op {
        Some(mut new_tbi) => {
            if deduction_tree {
                new_tbi.add_to_implied_by((rule_number, vec![tbi1.clone(), tbi2.clone()]));
            }

            Some(vec![new_tbi])
        }
        None => None,
    }
}

pub fn dl_lite_closure_decompact_vec<'a>(
    vec: &'a [&TbiDllite],
) -> (&'a TbiDllite, &'a TbiDllite, usize) {
    let tbi1 = vec[0];
    let tbi2 = vec[1];
    let level = usize::max(tbi1.level(), tbi2.level()) + 1;

    (tbi1, tbi2, level)
}

// X < Y AND (Y < -Z or Z < -Y) THEN X < -Z
// this rule generalizes rule 'NEGATIVE FOUR'
pub fn dl_lite_closure_negative_one(
    vec: &[&TbiDllite],
    deduction_tree: bool,
) -> Option<Vec<TbiDllite>> {
    // extract the needed values

    // decompact and get the level
    let (tbi1, tbi2, level) = dl_lite_closure_decompact_vec(&vec);

    // can decompact first without problem
    let x = tbi1.lside();
    let y = tbi1.rside();

    // second tbi we do not kwon which pattern to match
    let lside2 = tbi2.lside();
    let rside2 = tbi2.rside();

    // one of the two patters match
    if (y == lside2 && rside2.is_purely_negated())
        || (rside2.is_purely_negated() && rside2.is_negation(y))
    {
        // initialize the new tbi value
        let new_tbi_op: Option<TbiDllite>;

        // pick the matching pattern
        if y == lside2 && rside2.is_purely_negated() {
            new_tbi_op = TbiDllite::new(x.clone(), rside2.clone(), level);
        } else {
            new_tbi_op = TbiDllite::new(x.clone(), lside2.clone().negate(), level);
        }

        // be sure that the tbi was built
        output_new_tbis_for_rules(new_tbi_op, deduction_tree, CR::First, tbi1, tbi2)
    } else {
        None
    }
}

// r < s AND ( E.s < -X OR X < -E.s) THEN E.r < -X
pub fn dl_lite_closure_negative_two(
    vec: &[&TbiDllite],
    deduction_tree: bool,
) -> Option<Vec<TbiDllite>> {
    let (tbi1, tbi2, level) = dl_lite_closure_decompact_vec(&vec);

    if DLType::all_roles(tbi1.lside().t(), tbi1.rside().t()) && tbi2.rside().is_purely_negated() {
        if tbi2.lside().t() == DLType::ExistsConcept
            && ItemDllite::child(Some(tbi2.lside()), 1).unwrap()[0] == tbi1.rside()
        {
            let new_item = tbi1.lside().clone().exists().unwrap(); // the clone instruction is necessary because exists
                                                                   // consumes the item
            let new_tbi_op = TbiDllite::new(new_item, tbi2.rside().clone(), level);

            output_new_tbis_for_rules(new_tbi_op, deduction_tree, CR::Second, tbi1, tbi2)
        } else if ItemDllite::child(Some(tbi2.rside()), 1).unwrap()[0].t() == DLType::ExistsConcept
            && ItemDllite::child(Some(tbi2.rside()), 2).unwrap()[0] == tbi1.rside()
        {
            let new_item = tbi1.lside().clone().exists().unwrap(); // the clone instruction is necessary because exists
                                                                   // consumes the item
            let new_tbi_op = TbiDllite::new(new_item, tbi2.lside().clone().negate(), level);

            output_new_tbis_for_rules(new_tbi_op, deduction_tree, CR::Second, tbi1, tbi2)
        } else {
            None
        }
    } else {
        None
    }
}

// r < s AND ( E.s^- < -X OR X < -E.s^-) THEN E.r^- < -X
pub fn dl_lite_closure_negative_three(
    vec: &[&TbiDllite],
    deduction_tree: bool,
) -> Option<Vec<TbiDllite>> {
    let (tbi1, tbi2, level) = dl_lite_closure_decompact_vec(&vec);

    if DLType::all_roles(tbi1.lside().t(), tbi1.rside().t()) && tbi2.rside().is_purely_negated() {
        let second_child_lside2 = ItemDllite::child(Some(tbi2.lside()), 2);

        if second_child_lside2.is_some() && second_child_lside2.unwrap()[0] == tbi1.rside() {
            let new_item = tbi1.lside().clone().inverse().unwrap().exists().unwrap();

            let new_tbi_op = TbiDllite::new(new_item, tbi2.rside().clone(), level);

            return output_new_tbis_for_rules(new_tbi_op, deduction_tree, CR::Third, tbi1, tbi2);
        }

        let third_child_rside2 = ItemDllite::child(Some(tbi2.rside()), 3);

        if third_child_rside2.is_some() && third_child_rside2.unwrap()[0] == tbi1.rside() {
            let new_item = tbi1.lside().clone().inverse().unwrap().exists().unwrap();

            let new_tbi_op = TbiDllite::new(new_item, tbi2.lside().clone().negate(), level);

            return output_new_tbis_for_rules(new_tbi_op, deduction_tree, CR::Third, tbi1, tbi2);
        }

        None
    } else {
        None
    }
}

// r < s AND ( s < -q OR q < -s ) THEN r < -q
pub fn dl_lite_closure_negative_four(
    vec: &[&TbiDllite],
    deduction_tree: bool,
) -> Option<Vec<TbiDllite>> {
    // IN OUR SETTING THIS RULE AND THE RULE 'NEGATIVE ONE' ARE NOT DIFFERENT,
    // I'M GOING TO POINT TO RULE 'NEGATIVE ONE' HERE

    dl_lite_closure_negative_one(vec, deduction_tree)
}

// ( E.r < -E.r OR E.r^- < -E.r^- OR r < -r ) THEN ( E.r < -E.r AND E.r^- < -E.r^- AND r < -r )
pub fn dl_lite_closure_negative_five(
    vec: &[&TbiDllite],
    deduction_tree: bool,
) -> Option<Vec<TbiDllite>> {
    /*
       you see ? here the necessary arity is only one, but is preferable to keep
       global arity equal to two and make the distinction here
    */

    let tbi = vec[0];
    let level = tbi.level();

    // inner function for clarity
    // closures are cool !
    let f = |t1: Option<TbiDllite>, t2: Option<TbiDllite>| match (t1, t2) {
        (Some(mut tbi1), Some(mut tbi2)) => {
            if deduction_tree {
                tbi1.add_to_implied_by((CR::Fifth, vec![tbi.clone()]));
                tbi2.add_to_implied_by((CR::Fifth, vec![tbi.clone()]));
            }

            Some(vec![tbi1, tbi2])
        }
        (_, _) => None,
    };

    if DLType::all_roles(tbi.lside().t(), tbi.rside().t()) && tbi.lside().is_negation(tbi.rside()) && !tbi.lside().is_purely_negated() {
        let r_inv = tbi.lside().clone().inverse().unwrap();

        let tbi1_op = TbiDllite::new(r_inv.clone().exists().unwrap(), r_inv.exists().unwrap().negate(), level);
        let tbi2_op = TbiDllite::new(
            tbi.lside().clone().exists().unwrap(),
            tbi.lside().clone().exists().unwrap().negate(),
            level,
        );

        f(tbi1_op, tbi2_op)
    } else if tbi.lside().t() == DLType::ExistsConcept && tbi.lside().is_negation(tbi.rside()) {
        // this block should take care of the first two possibilities

        let real_r = ItemDllite::child(Some(tbi.lside()), 1).unwrap()[0];

        let tbi1_op = TbiDllite::new(real_r.clone(), real_r.clone().negate(), level);
        let tbi2_op = TbiDllite::new(
            real_r.clone().inverse().unwrap().exists().unwrap(),
            real_r.clone().inverse().unwrap().exists().unwrap().negate(),
            level,
        );

        f(tbi1_op, tbi2_op)
    } else {
        None
    }
}

// ( X < Y AND Y < Z ) THEN  X < Z
// THIS RULE GENERALIZES RULE 'NEGATIVE ONE' (THUS ALSO 'NEGATIVE FOUR')
// THIS RULE GENERALIZES RULE 'POSITIVE NINE'
pub fn dl_lite_closure_positive_six(
    vec: &[&TbiDllite],
    deduction_tree: bool,
) -> Option<Vec<TbiDllite>> {
    let (tbi1, tbi2, level) = dl_lite_closure_decompact_vec(&vec);

    if tbi1.rside() == tbi2.lside() {
        let new_tbi_op = TbiDllite::new(tbi1.lside().clone(), tbi2.rside().clone(), level);

        output_new_tbis_for_rules(new_tbi_op, deduction_tree, CR::Sixth, tbi1, tbi2)
    } else {
        None
    }
}

// ( r < s AND  X < E.r) THEN X < E.s
pub fn dl_lite_closure_positive_seven(
    vec: &[&TbiDllite],
    deduction_tree: bool,
) -> Option<Vec<TbiDllite>> {
    let (tbi1, tbi2, level) = dl_lite_closure_decompact_vec(&vec);

    if DLType::all_roles(tbi1.lside().t(), tbi1.lside().t()) && !tbi1.rside().is_purely_negated()
        && tbi2.rside().t() == DLType::ExistsConcept
        && ItemDllite::child(Some(tbi2.rside()), 1).unwrap()[0] == tbi1.lside()
    {
        let new_tbi_op = TbiDllite::new(
            tbi2.lside().clone(),
            tbi1.rside().clone().exists().unwrap(),
            level,
        );

        output_new_tbis_for_rules(new_tbi_op, deduction_tree, CR::Seventh, tbi1, tbi2)
    } else {
        None
    }
}

// ( r < s AND X < E.r^- ) THEN X < E.s^-
pub fn dl_lite_closure_positive_eight(
    vec: &[&TbiDllite],
    deduction_tree: bool,
) -> Option<Vec<TbiDllite>> {
    let (tbi1, tbi2, level) = dl_lite_closure_decompact_vec(&vec);

    if DLType::all_roles(tbi1.lside().t(), tbi1.rside().t()) && !tbi1.rside().is_purely_negated()
        && tbi2.rside().t() == DLType::ExistsConcept
        && ItemDllite::child(Some(tbi2.rside()), 1).unwrap()[0].is_inverse(tbi1.lside())
    {
        let new_item = tbi1.rside().clone().inverse().unwrap().exists().unwrap();
        let new_tbi_op = TbiDllite::new(tbi2.lside().clone(), new_item, level);

        output_new_tbis_for_rules(new_tbi_op, deduction_tree, CR::Eight, tbi1, tbi2)
    } else {
        None
    }
}

// ( r < s AND s < q ) THEN r < q
pub fn dl_lite_closure_positive_nine(
    vec: &[&TbiDllite],
    deduction_tree: bool,
) -> Option<Vec<TbiDllite>> {
    // THIS RULE IS GENERALIZED BY RULE 'POSITIVE SIX' THUS
    // IT WILL POINT TO IT
    dl_lite_closure_positive_six(vec, deduction_tree)
}

// ( r < s OR r^- < s^- ) THEN ( r < s AND r^- < s^- )
pub fn dl_lite_closure_positive_ten(
    vec: &[&TbiDllite],
    deduction_tree: bool,
) -> Option<Vec<TbiDllite>> {
    let tbi = vec[0];
    let level = tbi.level();

    if DLType::all_roles(tbi.lside().t(), tbi.rside().t()) && !tbi.rside().is_purely_negated() {
        let r = tbi.lside();
        let s = tbi.rside();

        let new_tbi_op = TbiDllite::new(
            r.clone().inverse().unwrap(),
            s.clone().inverse().unwrap(),
            level,
        );

        match new_tbi_op {
            Some(mut new_tbi) => {
                if deduction_tree {
                    new_tbi.add_to_implied_by((CR::Tenth, vec![tbi.clone()]));
                }

                Some(vec![new_tbi])
            }
            None => None,
        }
    } else {
        None
    }
}
