use crate::core::ik_segmenter::TokenMode;
use crate::core::lexeme::Lexeme;
use crate::core::lexeme_path::LexemePath;
use crate::core::ordered_linked_list::{Node, OrderedLinkedList};
use std::collections::{BTreeSet, HashMap};
use std::ptr::NonNull;

pub struct IKArbitrator {}

impl IKArbitrator {
    pub fn new() -> Self {
        IKArbitrator {}
    }

    pub unsafe fn process(
        &mut self,
        org_lexemes: &mut OrderedLinkedList<Lexeme>,
        mode: TokenMode,
    ) -> HashMap<usize, LexemePath> {
        let mut path_map = HashMap::<usize, LexemePath>::new();
        let mut cross_path = LexemePath::new();
        let mut cur_node = org_lexemes.head_node();
        while cur_node.is_some() {
            let org_lexeme = &(cur_node.as_ref().unwrap().as_ref().val);
            if !cross_path.add_cross_lexeme(org_lexeme) {
                if cross_path.size() == 1 || !(mode == TokenMode::SEARCH) {
                    path_map.insert(cross_path.get_path_begin() as usize, cross_path);
                } else {
                    let judge_result = self.judge(cross_path.get_head());
                    path_map.insert(
                        judge_result.as_ref().unwrap().get_path_begin() as usize,
                        judge_result.unwrap(),
                    );
                }
                cross_path = LexemePath::new();
                cross_path.add_cross_lexeme(org_lexeme);
            }
            cur_node = cur_node.as_ref().unwrap().as_ref().next.as_ref();
        }

        if cross_path.size() == 1 || !(mode == TokenMode::SEARCH) {
            path_map.insert(cross_path.get_path_begin() as usize, cross_path);
        } else {
            let judge_result = self.judge(cross_path.get_head());
            path_map.insert(
                judge_result.as_ref().unwrap().get_path_begin() as usize,
                judge_result.unwrap(),
            );
        }
        path_map
    }

    pub unsafe fn judge(&mut self, cur_node: Option<&NonNull<Node<Lexeme>>>) -> Option<LexemePath> {
        let mut path_options = BTreeSet::new();
        let mut option = LexemePath::new();
        let mut lexeme_stack = self.forward_path(cur_node, &mut option);
        path_options.insert(option.clone());
        let mut c;
        while !lexeme_stack.is_empty() {
            c = lexeme_stack.pop();
            self.back_path(c.unwrap(), &mut option);
            self.forward_path(c.unwrap(), &mut option);
            path_options.insert(option.clone());
        }
        // pick first one
        let mut a = None;
        if !path_options.is_empty() {
            for o in path_options.iter() {
                a = Some(o.clone());
                break;
            }
        }
        return a;
    }

    pub unsafe fn forward_path<'a>(
        &'a self,
        cur_node: Option<&'a NonNull<Node<Lexeme>>>,
        option: &mut LexemePath,
    ) -> Vec<Option<&NonNull<Node<Lexeme>>>> {
        let mut conflict_stack: Vec<Option<&NonNull<Node<Lexeme>>>> = Vec::new();
        let mut cur = cur_node;
        while cur.is_some() {
            let c = &(cur.as_ref().unwrap().as_ref().val);
            if !option.add_not_cross_lexeme(c) {
                conflict_stack.push(cur);
            }
            cur = cur.as_ref().unwrap().as_ref().next.as_ref();
        }
        return conflict_stack;
    }

    pub unsafe fn back_path(&self, l: Option<&NonNull<Node<Lexeme>>>, option: &mut LexemePath) {
        let lexeme = &(l.as_ref().unwrap().as_ref().val);
        while option.check_cross(lexeme) {
            option.remove_tail();
        }
    }
}
