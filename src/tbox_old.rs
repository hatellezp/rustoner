use std::fmt;

use std::collections::HashSet;
use arrayvec::ArrayVec;

use std::collections::VecDeque;
use crate::base::{Context, CI, DLType};

/*
a TBox Item has a pointer to the global context, that should be the same for all the framework,
also two CI objects (not pointers to them) modeling lside <= rside
 */
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct TBoxItem<'a> {
    context: &'a Context,
    lside: CI<'a>,
    rside: CI<'a>,
}

impl<'a> fmt::Display for TBoxItem<'a> {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}<={}", self.lside, self.rside)
    }
}

impl<'a> TBoxItem<'a> {
    pub fn new(context: &'a Context, lside: CI<'a>, rside: CI<'a>) -> Option<TBoxItem<'a>> {
        // first verify that context is a valid context for lside and rside
        if !lside.is_valid(context) && !rside.is_valid(context) {
            Option::None
        } else if !DLType::same_type(lside.t(), rside.t()) {
            Option::None
        } else {
            match lside.t() {
                DLType::BaseRole | DLType::InverseRole => {
                    match rside.t() {
                        DLType::BaseRole | DLType::InverseRole | DLType::NegatedRole => Some(TBoxItem { context, lside, rside }),
                        _ => Option::None,
                    }
                },
                DLType::BaseConcept | DLType::ExistsConcept => {
                    match rside.t() {
                        DLType::BaseConcept | DLType::ExistsConcept | DLType::NegatedConcept => Some(TBoxItem { context, lside, rside}),
                        _ => Option::None,
                    }
                },
                _ => Option::None,
            }
        }
    }

    pub fn is_contradiction(&self) -> bool { (*self).lside.is_negation(&(*self).rside) }

    // Nope, decompact consumes self, be aware
    pub fn decompact(self) -> (&'a Context, CI<'a>, CI<'a>) {
        (self.context, self.lside, self.rside)
    }

    pub fn decompact_by_reference(&'a self) -> (&'a Context, &CI<'a>, &CI<'a>) {
        (self.context, &self.lside, &self.rside)
    }

    pub fn is_valid(&self, other_context: &'a Context) -> bool {
        /*
        verify that other_context is a valid context for self
         */
        (self.context.is_sub_context(other_context)
            && self.lside.is_valid(other_context)
            && self.rside.is_valid(other_context)
        )
    }

    pub fn clone_with_context<'b>(&'b self, context: &'a Context) -> Option<TBoxItem<'a>> {
        let new_lside: Option<CI<'a>> = self.lside.clone_with_context(context);
        let new_rside: Option<CI<'a>> = self.lside.clone_with_context(context);

        match (new_lside, new_rside) {
            (Some(_), Some(_)) => TBoxItem::new(context, new_lside.unwrap(), new_rside.unwrap()),
            _ => Option::None,
        }

    }

    pub fn reverse_negation<'b>(&'b self, context: &'a Context) -> Option<TBoxItem<'a>> {
        let lside_negated: Option<CI<'a>> = self.lside.negate(context);
        let rside_negated: Option<CI<'a>> = self.rside.negate(context);

        match (lside_negated, rside_negated) {
            (Some(_), Some(_)) =>
                TBoxItem::new(context, rside_negated.unwrap(), lside_negated.unwrap()),
            _ => Option::None,
        }
    }
}

pub struct TBox<'a> {
    context: &'a Context,
    items: HashSet<TBoxItem<'a>>,
    length: usize,
    completed: bool,
}

impl<'a> TBox<'a> {
    pub fn new<I>(context: &'a Context, vec: I) -> Option<TBox<'a>>
        where I: Iterator<Item = TBoxItem<'a>> {
        /*
        the context that you are setting must be a valid context for each item in the tbox
         */
        let mut counter: usize = 0;
        let mut items: HashSet<TBoxItem<'a>> = HashSet::new();

        for item in vec {
            if !item.is_valid(context) {
                return Option::None
            } else {
                items.insert(item);
                counter += 1;
            }

        }

        Some(TBox { context, items, length: counter, completed: false })
    }

    pub fn get_items(&self) -> Vec<&TBoxItem> {
        let mut vec: Vec<&TBoxItem> = Vec::new();

        for item in &(self).items {
            vec.push(item);
        }

        vec
    }

    pub fn add(mut self, tbi: TBoxItem<'a>) -> bool {
        /*
        whenever you add a new tboxitem, you can't be sure you uncomplet the tbox
         */
        if !self.items.contains(&tbi) && tbi.is_valid(self.context) {
            self.items.insert(tbi);
            self.completed = false;
            self.length += 1;

            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        (*self).length
    }

    fn __complete<'b>(&'b self, global_context: &'a Context, verbose: bool) -> Option<TBox<'a>> {
        // queue for the items to be treated
        let mut q: VecDeque<TBoxItem> = VecDeque::new();

        // what is this ?
        let mut new_lenght: usize = 0;

        // keep record (in the form of pointers) of what items have already been treated
        let mut already_done: HashSet<&TBoxItem> = HashSet::new();

        // from here we will built the new tbox
        let mut tbox_items: Vec<TBoxItem>  = Vec::new();

        // a super counter
        let mut counter: usize = 0;

        // the tbox item in course
        let mut in_course: TBoxItem;

        // roles witness
        let mut look_out_for_roles: bool;

        // first pass to put everything into the queue
        for tbi in &self.items {
            let (_, _, rside) = tbi.decompact_by_reference();  // good idea to duplicate ? no idea...

            let tbi_clone  = tbi.clone_with_context(global_context);
            if tbi_clone.is_none() {
                return Option::None
            }

            q.push_front(tbi_clone.unwrap());

            // first rule: A=>B then B=>A
            if rside.is_negated() {
                let new_tbi: Option<TBoxItem<'a>> = tbi_clone.unwrap().reverse_negation(global_context);

                if new_tbi.is_none() {
                    return Option::None
                }

                q.push_front(new_tbi.unwrap());

                if verbose {
                    println!("first rule applied here for: {}, giving: {}", &tbi_clone.unwrap(), &new_tbi.unwrap());
                }
            }
        }

        let mut while_condition = true; // this is necessary to bypass the mutable-immutable nightmare that is rust
        while while_condition {
            in_course = q.pop_back().unwrap();

            // I hate rust lifetimes
            let mut inside_in_course = in_course.clone_with_context(global_context).unwrap();

            if verbose {
                println!("treating now: {}", &in_course);
            }

            // do not treat the same item twice
            if already_done.contains(&inside_in_course) { //|| tbox_items.contains(&inside_in_course) {
                if verbose {
                    println!("    already treated, passing");
                }
                continue;
            }

            // the in_course item is added to the already done values
            tbox_items.push(inside_in_course);
            new_lenght += 1;

            already_done.insert(&inside_in_course);
            counter += 1;


            // the item is now in tbox_items, for the analysis, it suffices to use references
            let (_, lside, rside) = inside_in_course.decompact_by_reference();

            look_out_for_roles =
                [DLType::BaseRole, DLType::InverseRole, DLType::NegatedRole].contains(&lside.t());

            for tbi in &mut tbox_items {
                if verbose {
                    println!("comparing with {}", tbi);
                }

                let (_, tbi_lside, tbi_rside) = tbi.decompact_by_reference();

                // second rule: A=>B and B=>C then A=>C
                if rside == tbi_lside {
                    let tbi_for_q = TBoxItem::new(global_context,
                                                  lside.clone_with_context(global_context).unwrap(),
                                                  tbi_rside.clone_with_context(global_context).unwrap()).unwrap();

                    q.push_front(tbi_for_q);

                    if verbose {
                        println!("    second rule applied here for {} and {}", inside_in_course, tbi);
                    }
                }
            }


            while_condition = !q.is_empty();  // awful language, there is a difference between being safe and being an impossible language to write something
        }


        /*
        for item in &mut tbox_items {
            println!("{}", item);
        }

         */

        Option::None
    }
}